'use strict';
// Run old calls unchanged against public 1.0.2 and the release candidate.
const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..');
const previousPath=path.resolve(process.env.BASELINE_ASTRO_MODULE||'artifacts/public-baseline/node_modules/@7mlabs/astrology');
const currentPath=path.resolve(process.env.ASTRO_ENGINE_MODULE||path.join(root,'bindings/node'));
const previous=require(previousPath),current=require(currentPath);
const baselineVersion=require(path.join(previousPath,'package.json')).version;
assert(['1.0.2','1.1.0'].includes(baselineVersion));
const additions=[];
const extraFields=new Set(['operation','chartKind','inputProvenance','rulershipRulesVersion','houseAssignment','sourceBirth']);
const normalizeMessage=s=>s.replaceAll('`includeMetadata`, ','').replaceAll(', `includeMetadata`','').replaceAll('`sourceContext`, ','').replaceAll(', `sourceContext`','').replaceAll('`otherSourceContext`, ','').replaceAll(', `otherSourceContext`','').replaceAll('GeometryInput','Request');
const angularCorrections=[];
function compare(a,b,where='') {
 if(a===b)return;
 if(where.endsWith('/engineVersion'))return;
 if(where.includes('/errors/')&&where.endsWith('/message')){assert.equal(normalizeMessage(b),normalizeMessage(a),where);return;}
 // The only intended numerical change to legacy results, kept explicit.
 if(where.endsWith('/signedDelta')&&a===180&&b===-180){angularCorrections.push(where);return;}
 if(Array.isArray(a)){assert(Array.isArray(b),where);assert.equal(a.length,b.length,where);for(let i=0;i<a.length;i++)compare(a[i],b[i],`${where}/${i}`);return;}
 if(a!==null&&typeof a==='object'){assert(b!==null&&typeof b==='object',where);for(const key of Object.keys(b).filter(key=>!Object.hasOwn(a,key))){assert(/(?:\/calculation(?:\/composite)?|\/sourceCalculations\/[AB])$/.test(where)&&extraFields.has(key),`Unexpected addition: ${where}/${key}`);additions.push(`${where}/${key}`);}for(const key of Object.keys(a))compare(a[key],b[key],`${where}/${key}`);return;}
 assert.deepEqual(b,a,where);
}
const cases=JSON.parse(fs.readFileSync(path.join(root,'tests/conformance/cases.json')));
for(const test of cases){const raw=test.rawRequest||JSON.stringify(test.request);compare(JSON.parse(previous.calculateJson(raw)),JSON.parse(current.calculateJson(raw)),test.name);}
const helpers=[['geometry','normalize',{longitude:-10}],['geometry','separation',{longitude1:350,longitude2:10}],['geometry','midpoint',{longitude1:350,longitude2:10}],['houses','locate',{longitude:350,houseCusps:Array.from({length:12},(_,i)=>i*30)}]];
for(const [group,action,input]of helpers)compare(previous[group][action](input),current[group][action](input),`${group}.${action}`);
const local={operation:'natalDomains',local:{year:1995,month:6,day:15,hour:14,minute:30},timeZone:'Asia/Ho_Chi_Minh',location:{latitude:10.8231,longitude:106.6297},additionalPoints:'all',domains:['love']};
const a=previous.calculateFromLocal(local),b=current.calculateFromLocal(local);compare(a,b,'calculateFromLocal');
for(const sdk of [previous,current])compare(sdk.calculateFromLocal(local),sdk.expandContext(sdk.compressPayload(sdk.calculateFromLocal(local))),'compression roundtrip');
const options={sections:['wheel','houseFocus'],focus:{house:7,mode:'highlight'}};
compare(previous.buildChartModel(a,options),current.buildChartModel(b,options),'buildChartModel');
// Pass the identical legacy payload so embedded metadata cannot differ by version.
assert.equal(current.renderChartSvg(a,options),previous.renderChartSvg(a,options),'legacy SVG');
compare(previous.resolveBirthTime({local:local.local,timeZone:local.timeZone}),current.resolveBirthTime({local:local.local,timeZone:local.timeZone}),'resolveBirthTime');
const report={result:'passed',baseline:baselineVersion,candidate:require(path.join(currentPath,'package.json')).version,platform:process.platform,arch:process.arch,cases:cases.length,helpers:helpers.length,services:['calculateFromLocal','resolveBirthTime','SVG','model','compression roundtrip'],ignoredFields:['engineVersion'],addedMetadataPaths:[...new Set(additions)],angularCorrections};
fs.mkdirSync(path.join(root,'artifacts'),{recursive:true});fs.writeFileSync(path.join(root,'artifacts/upgrade-compatibility.json'),JSON.stringify(report,null,2)+'\n');fs.writeFileSync(path.join(root,'artifacts/public-compatibility.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
