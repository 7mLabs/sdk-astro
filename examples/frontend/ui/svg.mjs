const $=id=>document.getElementById(id);
const form=$('birth-form');let payload=null;let currentUrl=null;let modelUrl=null;let busy=false;let view='wheel';let focus=null;let model=null;let calculationCount=0;
const parts={wheel:['wheel'],positions:['positions'],houses:['houses'],relations:['aspectMatrix','aspects'],houseFocus:['wheel','houseFocus']};
async function api(route,body){
  const response=await fetch(`/api/astro/${route}`,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});const result=await response.json();
  if(!response.ok||result.error||result.errors?.length){const error=result.error||result.errors?.[0]||{code:'REQUEST_FAILED',message:'Không gọi được SDK.'};throw new Error(`${error.code}: ${error.message}`);}return result;
}
function status(text,error=false){$('status').textContent=text;$('status').classList.toggle('error',error);}
function field(key){return form.elements.namedItem(key).value;}
function release(){for(const u of [currentUrl,modelUrl])if(u)URL.revokeObjectURL(u);currentUrl=null;modelUrl=null;}
function clear(){release();model=null;$('chart').hidden=true;$('chart').removeAttribute('src');$('empty').hidden=false;$('hit-targets').replaceChildren();$('focus-summary').hidden=true;}
function lock(value){busy=value;for(const control of document.querySelectorAll('input,select,button'))control.disabled=value; if(!value){$('download').disabled=!currentUrl;$('download-model').disabled=!modelUrl;$('zoom').disabled=!currentUrl;$('clear-focus').disabled=!focus;}}
function showResolution(){
  const r=payload.calculation.timeResolution.birth;const u=r.utc;const z=n=>String(n).padStart(2,'0');
  const values=[['UTC',`${u.year}-${z(u.month)}-${z(u.day)} ${z(u.hour)}:${z(u.minute)}:${z(u.second)}`],['Offset',`${r.utcOffsetSeconds} giây`],['IANA tzdb',r.tzdbVersion],['Engine',payload.engineVersion],['Lần tính',String(calculationCount)]];
  $('resolution').replaceChildren(...values.flatMap(([name,value])=>{const dt=document.createElement('dt');dt.textContent=name;const dd=document.createElement('dd');dd.textContent=value;return[dt,dd];}));
}
function selectFocus(next){focus=next; $('focus-house').value=next?.house?String(next.house):'';$('focus-point').value=next?.pointIds?.[0]||'';}
function focusSummary(){
  const box=$('focus-summary');box.replaceChildren();box.hidden=!model?.focus;if(box.hidden)return;
  const h=model.focus.house;const title=document.createElement('strong');const detail=document.createElement('p');
  if(h){title.textContent=`H${h.number} · chủ tinh ${h.rulerId}`;detail.textContent=`Các điểm bên trong: ${h.occupantIds.join(', ')||'Không có'}. Chủ tinh ở H${h.rulerPlacement?.house??'—'} (${h.rulership}). ${model.aspects.length} góc sau bộ lọc.`;}
  else{title.textContent=`Đang chọn: ${model.focus.pointIds.join(', ')}`;detail.textContent=`${model.aspects.length} góc chiếu liên quan sau bộ lọc. Các điểm còn lại giữ tọa độ nguồn.`;}
  box.append(title,detail);
}
function hitTargets(){
  const targets=$('hit-targets');targets.replaceChildren();
  const [, ,width,height]=model.viewBox;
  for(const target of model.hitTargets){
    const b=document.createElement('button');b.type='button';b.className='hit-target';b.style.left=`${target.center.x/width*100}%`;b.style.top=`${target.center.y/height*100}%`;
    b.setAttribute('aria-label',target.kind==='house'?`Khám phá nhà H${target.house}`:`Chọn điểm ${target.id}`);b.title=target.kind==='house'?`H${target.house}`:target.id;
    b.dataset.kind=target.kind;b.dataset.target=target.kind==='house'?String(target.house):target.id;b.disabled=busy;targets.append(b);
  }
}
function pointOptions(){
  const select=$('focus-point');const current=focus?.pointIds?.[0]||'';const first=document.createElement('option');first.value='';first.textContent='Không chọn';
  // The full source population stays selectable even when isolate hides labels.
  const natal=payload.data.natal||payload.data;const source=[...natal.placements,...(natal.additionalPoints||[])];
  select.replaceChildren(first,...source.map(p=>{const o=document.createElement('option');o.value=p.id;o.textContent=p.id;return o;}));select.value=current;
}
async function render(){
  if(!payload)return;
  const palette=$('palette').value;const aspectColors=palette==='classic'?{sextile:'#5599ff',trine:'#28bb89',square:'#f85d70',opposition:'#f85d70',conjunction:'#b39aff'}:palette==='groups'?{harmonious:'#51c9b0',challenging:'#f7869c',neutral:'#b39aff',minor:'#b4bed1'}:{};
  const aspectGroups=$('filter').value==='all'?undefined:[$('filter').value];
  const options={aspectEndpoints:$('endpoints').value,sections:parts[view],focus:focus?{...focus,mode:$('focus-mode').value}:undefined,aspectColors,aspectGroups,detail:$('detail').value,theme:$('theme').value,locale:'vi',title:'Lá số mẫu · 7mlabs Astro'};
  const out=(await api('render-svg',{payload,options})).data;release();model=out.model;
  currentUrl=URL.createObjectURL(new Blob([out.svg],{type:'image/svg+xml;charset=utf-8'}));modelUrl=URL.createObjectURL(new Blob([JSON.stringify(model,null,2)],{type:'application/json'}));
  $('chart').src=currentUrl;$('chart').hidden=false;$('empty').hidden=true;
  for(const b of document.querySelectorAll('[data-view]'))b.setAttribute('aria-pressed',String(b.dataset.view===view));
  $('calculation-note').textContent=`Payload #${calculationCount} · đổi phần xem, chọn điểm/nhà và xuất file không tính lại thiên văn.`;
  $('evidence').textContent=JSON.stringify({view,sections:out.model.sections,focus:out.model.focus,coordinatePolicy:out.metadata.coordinatePolicy,rendererVersion:out.metadata.rendererVersion,coverage:{...out.coverage,renderedAspectData:undefined},warnings:out.warnings},null,2);
  status(`${out.coverage.renderedBodies} thiên thể / điểm · ${out.coverage.renderedAspects}/${out.coverage.availableAspects} góc chiếu · ${out.model.sections.length} phần đang xem`);
  pointOptions();focusSummary();hitTargets();
}
async function update(calculate){
  if(busy)return;lock(true);
  try{
    status(calculate?'Đang đổi giờ và tính dữ liệu…':'Đang dựng phần xem từ payload…');
    if(calculate){
      const [year,month,day]=field('date').split('-').map(Number);const [hour,minute,second=0]=field('time').split(':').map(Number);
      payload=await api('calculate-from-local',{operation:'natalDomains',domains:['identity'],local:{year,month,day,hour,minute,second},timeZone:field('zone'),disambiguation:field('ambiguity'),location:{latitude:Number(field('lat')),longitude:Number(field('lon'))},houseSystem:field('houses'),additionalPoints:field('points')==='none'?[]:field('points'),aspectPreset:field('aspects')});calculationCount++;
      if(focus?.pointIds?.some(id=>![...payload.data.natal.placements,...(payload.data.natal.additionalPoints||[])].some(p=>p.id===id)))selectFocus(null);
      showResolution();
    }
    await render();
  }catch(error){if(calculate){payload=null;clear();$('resolution').replaceChildren();$('evidence').textContent='';}status(error.message,true);}
  finally{lock(false);}
}
form.addEventListener('submit',event=>{event.preventDefault();update(true);});
for(const id of ['theme','detail','palette','filter','focus-mode','endpoints'])$(id).addEventListener('change',()=>update(false));
for(const b of document.querySelectorAll('[data-view]'))b.addEventListener('click',()=>{view=b.dataset.view;if(view==='report')$('detail').value='professional';if(view==='houseFocus'&&!focus?.house)selectFocus({house:7});update(false);});
$('focus-house').addEventListener('change',()=>{const h=Number($('focus-house').value);selectFocus(h?{house:h}:null);if(h)view='houseFocus';else if(view==='houseFocus')view='wheel';update(false);});
$('focus-point').addEventListener('change',()=>{const id=$('focus-point').value;selectFocus(id?{pointIds:[id]}:null);view='wheel';update(false);});
$('clear-focus').addEventListener('click',()=>{selectFocus(null);if(view==='houseFocus')view='wheel';update(false);});
$('hit-targets').addEventListener('click',event=>{const b=event.target.closest('button[data-target]');if(!b||busy)return;selectFocus(b.dataset.kind==='house'?{house:Number(b.dataset.target)}:{pointIds:[b.dataset.target]});view=b.dataset.kind==='house'?'houseFocus':'wheel';update(false);});
$('zoom').addEventListener('click',()=>{const zoomed=$('chart-viewport').classList.toggle('zoomed');$('zoom').setAttribute('aria-pressed',String(zoomed));$('zoom').textContent=zoomed?'Vừa khung':'Phóng to';$('zoom-note').textContent=zoomed?'Cuộn trong khung để đọc chi tiết. Vùng bấm dùng tọa độ model do SDK trả về.':'Bấm vào ký hiệu thiên thể hoặc số nhà trên bản đồ để tập trung. SVG và model dùng chung tọa độ nguồn.';if(!zoomed)$('chart-viewport').scrollLeft=0;});
function download(url,name){if(!url)return;const a=document.createElement('a');a.href=url;a.download=name;document.body.append(a);a.click();a.remove();}
$('download').addEventListener('click',()=>download(currentUrl,`7mlabs-astro-${view}-${$('theme').value}.svg`));
$('download-model').addEventListener('click',()=>download(modelUrl,`7mlabs-astro-${view}-model.json`));window.addEventListener('pagehide',release);
try{const capabilities=(await api('capabilities',{})).data;if(!capabilities.svg?.parts)throw new Error('Cần build/cài package renderer có parts và chart model.');await update(true);}
catch(error){status(error.message,true);$('calculate').disabled=true;}
