'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const sdk = require(process.env.ASTRO_ENGINE_MODULE || path.resolve(__dirname, '../../bindings/node'));
const root = path.resolve(__dirname, '../..');
const birth = { local: { year: 1995, month: 6, day: 15, hour: 2, minute: 30, second: 12.125 }, timeZone: 'Asia/Ho_Chi_Minh' };
const location = { latitude: 10.8231, longitude: 106.6297 };
const resolution = sdk.resolveBirthTime(birth);
assert.deepEqual(resolution.utc, { year: 1995, month: 6, day: 14, hour: 19, minute: 30, second: 12.125 });
assert.equal(resolution.utcOffsetSeconds, 25200);
assert.equal(resolution.tzdbVersion, sdk.getCapabilities().civilTime.tzdbVersion);
const fold = { local: { year: 2024, month: 11, day: 3, hour: 1, minute: 30 }, timeZone: 'America/New_York' };
assert.throws(() => sdk.resolveBirthTime(fold), { code: 'AMBIGUOUS_LOCAL_TIME' });
assert.equal(sdk.resolveBirthTime({ ...fold, disambiguation: 'earlier' }).utc.hour, 5);
assert.equal(sdk.resolveBirthTime({ ...fold, disambiguation: 'later' }).utc.hour, 6);
assert.throws(() => sdk.resolveBirthTime({ local: { year: 2024, month: 3, day: 10, hour: 2, minute: 30 }, timeZone: fold.timeZone }), { code: 'NONEXISTENT_LOCAL_TIME' });
for (const year of [2099, 2100, 2200, 2399]) {
  assert.equal(sdk.resolveBirthTime({ local: { year, month: 7, day: 15, hour: 12, minute: 0 }, timeZone: fold.timeZone }).utcOffsetSeconds, -14400);
}
assert.throws(() => sdk.resolveBirthTime({ ...birth, utcOffsetSeconds: 0 }), { code: 'INVALID_TIME_INPUT' });
assert.throws(() => sdk.calculateFromLocal({ ...birth, utc: resolution.utc, location }), { code: 'INVALID_TIME_INPUT' });
const input = { operation: 'natal', ...birth, location };
const before = JSON.stringify(input);const local = sdk.calculateFromLocal(input);
assert.equal(JSON.stringify(input), before);
assert.deepEqual(local.data, sdk.calculate({ operation: 'natal', utc: resolution.utc, location }).data);
let variants = 0;
for (const detail of ['basic', 'advanced', 'professional']) for (const theme of ['dark', 'light']) {
  const options = { detail, theme, title: 'Bản đồ Việt Nam · 7mlabs' };
  const a = sdk.renderChartSvgResult(local, options);
  assert.deepEqual(a, sdk.renderChartSvgResult(local, options));
  assert.equal(a.svg, sdk.renderChartSvg(local, options));
  for (const prohibited of ['<text', '<script', '<image', 'href=', 'var(--', 'font-family']) assert(!a.svg.includes(prohibited));
  const markers = [...a.svg.matchAll(/data-point-id="([^"]+)" data-longitude="([^"]+)"/g)];
  assert.equal(markers.length, 10);
  for (const [, id, longitude] of markers) assert.equal(Number(longitude), local.data.placements.find(p => p.id === id).longitude);
  assert.equal(a.coverage.availableAspects, a.coverage.renderedAspects);
  assert.equal(a.metadata.coordinatePolicy, 'source f64, no angular quantization');
  variants++;
}
const filtered = sdk.renderChartSvgResult(local, { maxAspectLines: 0 });
assert.equal(filtered.coverage.renderedAspects, 0);assert.equal(filtered.coverage.excluded.lineLimit, filtered.coverage.availableAspects);
const symbols = { aries: { viewBox: [0, 0, 32, 32], nodes: [['path', { d: 'M4 28L16 4L28 28' }]], label: 'Bạch Dương', signIndex: 0 } };
assert(sdk.renderChartSvg(local, { zodiacSymbols: symbols }).includes('M4 28L16 4L28 28'));
assert.throws(() => sdk.renderChartSvg(local, { zodiacSymbols: { aries: { viewBox: [0, 0, 32, 32], nodes: [['path', { d: 'M0 0', onload: 'alert(1)' }]] } } }), { code: 'INVALID_SYMBOL' });
assert.throws(() => sdk.renderChartSvg(local, { zodiacSymbols: { aries: { viewBox: [0, 0, 32, 32], nodes: [['path', { d: 'M0' }]] } } }), { code: 'INVALID_SYMBOL' });
assert.throws(() => sdk.renderChartSvg(local, { instanceId: 'bad"id' }), { code: 'INVALID_RENDER_OPTIONS' });
assert.throws(() => sdk.renderChartSvg(local, { title: 'unsupported 😀' }), { code: 'UNSUPPORTED_LABEL' });
assert(JSON.parse(sdk.renderChartSvgRequestJson('{')).errors.length);
// Raw arguments must not inject replacement fields into the render wrapper.
for (const output of [
  sdk.renderChartSvgJson(JSON.stringify(local), 'null,"options":{}'),
  sdk.renderChartSvgJson(`null,"payload":${JSON.stringify(local)}`),
  sdk.renderChartSvgJson(JSON.stringify(local), '{"detail":"basic","detail":"advanced"}'),
]) {
  const result = JSON.parse(output);
  assert.equal(result.errors[0].code, 'INVALID_RENDER_OPTIONS');
  assert.equal(result.data, null);
}
let charts = 0;
for (const [fixture, chart] of [['natal-domains', 'natal'], ['couple', 'synastry'], ['couple', 'A'], ['couple', 'B'], ['composite', 'C'], ['forecast-day', 'transits']]) {
  const payload = JSON.parse(fs.readFileSync(path.join(root, 'examples', `${fixture}-result.json`)));
  const out = sdk.renderChartSvgResult(payload, { chart, detail: 'advanced' });
  assert.equal(out.metadata.chart, chart);
  assert.equal(out.coverage.availableAspects, out.coverage.renderedAspects + Object.values(out.coverage.excluded).reduce((sum, n) => sum + n, 0));
  charts++;
}
console.log(JSON.stringify({ passed: true, variants, chartAdapters: charts, engineVersion: local.engineVersion, tzdbVersion: resolution.tzdbVersion }));

const extended = sdk.calculateFromLocal({...input,additionalPoints:'all',aspectPreset:'extended'});
assert.deepEqual(extended.data.placements,local.data.placements);
assert.equal(extended.data.additionalPoints.length,7);
const professional=sdk.renderChartSvgResult(extended,{detail:'professional',maxAspectRows:4096,aspectColors:{harmonious:'#129854',trine:'#bad123',challenging:'#da4572'}});
assert.equal(professional.coverage.renderedBodies,17);
assert.equal(professional.coverage.aspectTableOmitted,0);
assert(professional.svg.includes('data-section="aspect-matrix"'));
assert(professional.svg.includes('#bad123'));
assert.throws(()=>sdk.calculate({...input,utc:resolution.utc,local:undefined,timeZone:undefined,additionalPoints:['northNode']}));
assert.throws(()=>sdk.renderChartSvg(extended,{aspectColors:{trine:'url(https://bad)'}}),{code:'INVALID_RENDER_OPTIONS'});
console.log(JSON.stringify({extendedPoints:true,professional:true,points:extended.data.additionalPoints.map(p=>p.id)}));

const domains=sdk.calculateFromLocal({...input,operation:'natalDomains',domains:['identity'],additionalPoints:'standard'});
for(const part of ['wheel','positions','houses','aspectMatrix','aspects']) {
  const svg=sdk.renderChartPart(domains,part);
  assert.equal((svg.match(/data-part=/g)||[]).length,1);
  assert.equal(svg.includes('data-chart-wheel'),part==='wheel');
}
const selected=sdk.renderChartSvgResult(domains,{sections:['wheel','houseFocus'],focus:{house:7}});
const model=sdk.buildChartModel(domains,{sections:['wheel','houseFocus'],focus:{house:7}});
assert.deepEqual(model,selected.model);
assert.equal(model.focus.house.rulerId,domains.data.context.houses[6].rulerBodyId);
assert(model.selectedPointIds.includes('H7'));
assert(model.hitTargets.some(t=>t.kind==='house'&&t.house===7));
assert(model.aspects.every(a=>model.selectedPointIds.includes(a.endpoint1)||model.selectedPointIds.includes(a.endpoint2)));
assert.throws(()=>sdk.renderChartPart(local,'houseFocus',{focus:{house:7}}),{code:'MISSING_HOUSE_CONTEXT'});
const only=sdk.buildChartModel(domains,{sections:['positions']});assert.equal(only.hitTargets.length,0);
assert.equal(only.points[0].labelPosition,null);
console.log(JSON.stringify({modularParts:true,modelOnly:true,houseFocus:true}));
