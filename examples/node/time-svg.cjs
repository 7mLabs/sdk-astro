'use strict';
// Generate real standalone charts. No browser or network is needed.
const fs = require('node:fs');
const path = require('node:path');
const sdk = require(process.env.ASTRO_ENGINE_MODULE || '@7mlabs/astrology');
const destination = path.resolve(process.argv[2] || 'svg-output');
fs.mkdirSync(destination, { recursive: true });
const birth = { local: { year: 1995, month: 6, day: 15, hour: 14, minute: 30, second: 0 }, timeZone: 'Asia/Ho_Chi_Minh' };
const location = { latitude: 10.8231, longitude: 106.6297 };
const resolved = sdk.resolveBirthTime(birth);
const fromUtc = sdk.calculate({ operation: 'natal', utc: resolved.utc, location });
const fromLocal = sdk.calculateFromLocal({ operation: 'natal', ...birth, location });
require('node:assert/strict').deepEqual(fromLocal.data, fromUtc.data);
for (const detail of ['basic', 'advanced', 'professional']) for (const theme of ['dark', 'light']) {
  const result = sdk.renderChartSvgResult(fromLocal, { detail, theme, title: 'Lá số mẫu · 7mlabs Astro' });
  const filename = path.join(destination, `natal-${detail}-${theme}.svg`);
  fs.writeFileSync(filename, result.svg);
  console.log(`${filename}: ${Buffer.byteLength(result.svg)} bytes, ${result.coverage.renderedAspects} aspects`);
}
fs.writeFileSync(path.join(destination, 'natal-payload.json'), JSON.stringify(fromLocal, null, 2) + '\n');
console.log(`Engine ${fromLocal.engineVersion}; bundled tzdb ${resolved.tzdbVersion}; local/UTC data parity passed.`);

const extended=sdk.calculateFromLocal({operation:'natal',...birth,location,additionalPoints:'all',aspectPreset:'extended'});
for(const theme of ['dark','light']) {
  const result=sdk.renderChartSvgResult(extended,{detail:'professional',theme,maxAspectRows:4096});
  fs.writeFileSync(path.join(destination,`extended-professional-${theme}.svg`),result.svg);
}
fs.writeFileSync(path.join(destination,'extended-payload.json'),JSON.stringify(extended,null,2)+'\n');

const modular=sdk.calculateFromLocal({operation:'natalDomains',...birth,location,domains:['identity'],additionalPoints:'standard'});
for(const part of ['wheel','positions','houses','aspectMatrix','aspects'])fs.writeFileSync(path.join(destination,`part-${part}.svg`),sdk.renderChartPart(modular,part));
const focused=sdk.renderChartSvgResult(modular,{sections:['wheel','houseFocus'],focus:{house:7}});
fs.writeFileSync(path.join(destination,'house-7-focus.svg'),focused.svg);
fs.writeFileSync(path.join(destination,'house-7-model.json'),JSON.stringify(sdk.buildChartModel(modular,{sections:['wheel'],focus:{house:7}}),null,2)+'\n');
