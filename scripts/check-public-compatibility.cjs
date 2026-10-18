'use strict';
// Compare the previous public package with the new source using unchanged inputs.
const fs=require('node:fs');const path=require('node:path');const assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..');
const baseline=require(path.resolve(process.env.BASELINE_ASTRO_MODULE||'artifacts/public-baseline/node_modules/@7mlabs/astrology'));
const current=require(path.join(root,'bindings/node/index.cjs'));
const strip=value=>Array.isArray(value)?value.map(strip):value&&typeof value==='object'?Object.fromEntries(Object.entries(value).filter(([key])=>key!=='engineVersion').map(([key,v])=>[key,strip(v)])):value;
const normalizeErrors=value=>{const copy=strip(value);for(const error of copy.errors||[]){error.message=error.message.replace('expected one of `additionalPoints`, ','expected one of ').replace('domains, rulership, aspectPreset and customProfiles require operation natalDomains','domains, rulership and customProfiles require operation natalDomains');}return copy;};
const cases=JSON.parse(fs.readFileSync(path.join(root,'tests/conformance/cases.json'),'utf8'));
for(const test of cases){
 const raw=test.rawRequest||JSON.stringify(test.request);
 const previous=JSON.parse(baseline.calculateJson(raw));const next=JSON.parse(current.calculateJson(raw));
 assert.deepStrictEqual(normalizeErrors(next),normalizeErrors(previous),`Public compatibility: ${test.name}`);
}
const report={result:'passed',baselineVersion:require(path.resolve(process.env.BASELINE_ASTRO_MODULE||'artifacts/public-baseline/node_modules/@7mlabs/astrology','package.json')).version,version:require(path.join(root,'bindings/node/package.json')).version,platform:process.platform,arch:process.arch,cases:cases.length,ignoredFields:['engineVersion'],errorTextNormalization:'Added additionalPoints in expected-field lists and removal of aspectPreset from domain-only option warnings; all other error text/data exact'};
fs.mkdirSync(path.join(root,'artifacts'),{recursive:true});fs.writeFileSync(path.join(root,'artifacts/public-compatibility.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
