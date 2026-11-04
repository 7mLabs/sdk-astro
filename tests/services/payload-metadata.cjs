'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const root=path.resolve(__dirname,'../..');
const sdk=require(process.env.ASTRO_ENGINE_MODULE||path.join(root,'bindings/node'));
const Ajv=require(process.env.AJV_2020_MODULE||'ajv/dist/2020');
const load=p=>JSON.parse(fs.readFileSync(path.join(root,p),'utf8'));
const oldAjv=new Ajv({strict:true,allErrors:true}),newAjv=new Ajv({strict:true,allErrors:true});
for(const name of fs.readdirSync(path.join(root,'tests/compatibility/1.1.0-schemas')))oldAjv.addSchema(load('tests/compatibility/1.1.0-schemas/'+name));
for(const name of fs.readdirSync(path.join(root,'schemas')).filter(n=>n.endsWith('.json')))newAjv.addSchema(load('schemas/'+name));
const legacy=oldAjv.getSchema('urn:7mlabs:astrology:query-response:1.0');
const current=new Ajv({strict:true,allErrors:true}).compile(load('schemas/query-response.schema.json'));
const birth={utc:{year:2000,month:1,day:1,hour:12,minute:0},location:{latitude:10.8231,longitude:106.6297}};
const calls=[
 ['geometry','normalize',{longitude:-10}],['geometry','separation',{longitude1:350,longitude2:10}],
 ['geometry','midpoint',{longitude1:350,longitude2:10}],
 ['aspects','between',{positions:[{id:'a',longitude:10},{id:'b',longitude:130}],rule:{angle:120,maxOrb:1}}],
 ['houses','locate',{longitude:10,houseCusps:Array.from({length:12},(_,i)=>i*30)}],
 ['points','inspect',{birth,pointIds:['sun']}],['houses','inspect',{birth,houseNumbers:[7]}],
 ['aspects','inspect',{birth,pointIds:['sun','moon']}]
];
for(const [group,action,input] of calls){
 const request={operation:'query',group,action,...input},old=sdk.calculate(request);
 assert(legacy(old),`${group}.${action}: legacy schema rejected ${JSON.stringify(legacy.errors)}`);
 assert(current(old),`${group}.${action}: current schema rejected legacy payload`);
 const enriched=sdk.calculate({...request,includeMetadata:true});
 assert(current(enriched),`${group}.${action}: current schema rejected ${JSON.stringify(current.errors)}`);
 assert.equal(enriched.calculation.operation,'query');
 assert.deepEqual(enriched.data,old.data);
 if(action==='inspect'){
  assert.equal(enriched.calculation.houseAssignment,'eclipticLongitude');
  assert.equal(enriched.calculation.rulershipRulesVersion,'1.0');
  assert.deepEqual(enriched.calculation.sourceBirth.location,birth.location);
 }
 assert.deepEqual(sdk.expandContext(sdk.compressPayload(enriched)),enriched);
 for(const value of [null,'true',1])assert.equal(JSON.parse(sdk.calculateJson(JSON.stringify({...request,includeMetadata:value}))).errors[0].code,'INVALID_INPUT');
}
const request={operation:'chart',positions:[{id:'sun',longitude:10},{id:'moon',longitude:130}]};
const a=sdk.calculate(request),b=sdk.calculate({...request,sourceContext:{zodiac:'tropical',sourceId:'fixture'}});
assert.deepEqual(a.data,b.data);
assert.equal(a.calculation.inputProvenance.positions.status,'unspecified');
assert.equal(b.calculation.inputProvenance.positions.status,'callerDeclared');
assert.equal(b.calculation.inputProvenance.positions.verified,false);
const geometry=new Ajv({strict:true,allErrors:true}).compile(load('schemas/geometry-request.schema.json'));
assert(geometry({...request,sourceContext:{zodiac:'sidereal',ayanamsa:'caller-model'}}));
for(const context of [null,{}, {zodiac:'tropical',ayanamsa:'x'},{sourceId:' '},{unknown:true}]){
 const input={...request,sourceContext:context};assert.equal(geometry(input),false);
 assert.equal(JSON.parse(sdk.calculateJson(JSON.stringify(input))).errors[0].code,'INVALID_INPUT');
}
console.log('Metadata: 8 query methods accept legacy/current schemas, opt-in provenance survives compression, invalid declarations rejected.');

for(const [name,schema] of [['natal-domains','natal-domains'],['couple','couple'],['composite','composite'],['forecast-day','forecast'],['events-year','events'],['davison-uncorrected','davison']]){
 const request=load('examples/'+name+'-request.json'),result=sdk.calculate(request),enriched=sdk.calculate({...request,includeMetadata:true});
 const id=load('schemas/'+schema+'-response.schema.json').$id;
 assert(oldAjv.getSchema(id)(result),name+': legacy response contract changed');
 assert(newAjv.getSchema(id)(enriched),name+': enriched response rejected '+JSON.stringify(newAjv.getSchema(id).errors));
 assert.deepEqual(enriched.data,result.data);
 assert.equal(enriched.calculation.operation,request.operation);
 if(request.operation!=='events'){
  const local=JSON.parse(JSON.stringify({...request,includeMetadata:true}));
  function convert(b){b.local=b.utc;delete b.utc;b.timeZone='Etc/UTC';}
  if(local.utc)convert(local);else if(local.birth)convert(local.birth);else{convert(local.personA);convert(local.personB);}
  const validateLocal=newAjv.getSchema('urn:7mlabs:astrology:calculate-from-local:1.0');
  assert(validateLocal(local),name+': local request schema '+JSON.stringify(validateLocal.errors));
  const result=sdk.calculateFromLocal(local);
  assert(newAjv.getSchema(id)(result),name+': local response schema '+JSON.stringify(newAjv.getSchema(id).errors));
  assert(result.calculation.timeResolution);
 }
}
console.log('Metadata: six astronomical response families retain frozen 1.1.0 schemas; enriched results validate against current schemas.');
