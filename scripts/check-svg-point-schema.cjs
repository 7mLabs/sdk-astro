'use strict';
// Development validator only, no Ajv dependency in shipped SDK packages.
const fs=require('node:fs');const path=require('node:path');const assert=require('node:assert/strict');
const Ajv=require(process.env.AJV_2020_MODULE||'ajv/dist/2020');
const sdk=require(process.env.ASTRO_ENGINE_MODULE||'../bindings/node');
const ajv=new Ajv({strict:true,allErrors:true});
const schema=name=>JSON.parse(fs.readFileSync(path.join(__dirname,'../schemas',name+'.schema.json')));
const validators={};for(const name of ['natal-request','natal-domains-request','natal-domains-response','calculate-from-local','render-chart-svg'])validators[name]=ajv.compile(schema(name));
let checks=0;function valid(name,value,expected=true){const ok=validators[name](value);assert.equal(ok,expected,name+': '+JSON.stringify(validators[name].errors));checks++;}
const birth={utc:{year:2000,month:1,day:1,hour:12,minute:0},location:{latitude:10.8,longitude:106.6}};
for(const points of ['standard','all',[],['chiron','osculatingLilith']]) {
 for(const operation of ['natal','natalDomains']) {
  const r={operation,...birth,additionalPoints:points,aspectPreset:'extended'};valid(operation==='natal'?'natal-request':'natal-domains-request',r);
  const payload=sdk.calculate(r);if(operation==='natalDomains')valid('natal-domains-response',payload);
  const opts={detail:'professional',aspectColors:{harmonious:'#6f9',trine:'#4cce8a',opposition:'#f7c8'},aspectGroups:['harmonious','challenging'],maxOrb:5,aspectPhase:'applying',display:{aspectMatrix:true,housesTable:true},maxAspectRows:4096};
  valid('render-chart-svg',{payload,options:opts});assert(sdk.renderChartSvg(payload,opts).includes('data-section="aspect-matrix"'));
 }
}
for(const bad of [null,'unknown',['chiron','chiron'],['northNode']])valid('natal-request',{operation:'natal',...birth,additionalPoints:bad},false);
valid('natal-request',{operation:'natal',...birth,aspectPreset:'major',aspectRules:[]},false);
valid('calculate-from-local',{operation:'natal',local:birth.utc,timeZone:'Etc/UTC',location:birth.location,additionalPoints:'all',aspectPreset:'extended'});
for(const opts of [{aspectColors:{trine:'red'}},{aspectColors:{unknown:'#123'}},{aspectPhase:'exact'},{detail:'unknown'},{bodyIds:['chiron','chiron']}]) valid('render-chart-svg',{payload:sdk.calculate({operation:'natal',...birth}),options:opts},false);
const domain=sdk.calculate({operation:'natalDomains',...birth,domains:['identity']});
for(const endpoints of ['bodies','bodiesAndAngles','all']) {
 const options={sections:['wheel','houseFocus'],focus:{house:7,mode:'isolate'},aspectEndpoints:endpoints};valid('render-chart-svg',{payload:domain,options});
 const model=sdk.buildChartModel(domain,options);assert.equal(model.focus.house.number,7);
 assert(model.points.every(p=>model.relatedPointIds.includes(p.id)));
 for(const a of model.aspects)assert(model.selectedPointIds.includes(a.endpoint1)||model.selectedPointIds.includes(a.endpoint2));
}
for(const options of [{sections:[]},{sections:['wheel','wheel']},{focus:{house:13}},{focus:{pointIds:['sun'],house:7}},{output:'html'},{aspectEndpoints:'unknown'}])valid('render-chart-svg',{payload:domain,options},false);
console.log(JSON.stringify({passed:true,schemaCases:checks,schemas:Object.keys(validators)}));
