'use strict';
// Development-only schema/implementation agreement. No validator in SDK runtime.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const Ajv = require(process.env.AJV_2020_MODULE || 'ajv/dist/2020');
const sdk = require(process.env.ASTRO_ENGINE_MODULE || path.resolve(__dirname, '../../bindings/node'));
const ajv = new Ajv({ strict: true, allErrors: true });
const root = path.resolve(__dirname, '../..');
for (const file of fs.readdirSync(path.join(root, 'schemas'))) {
  ajv.addSchema(JSON.parse(fs.readFileSync(path.join(root, 'schemas', file))));
}
const schema = name => ajv.getSchema(`urn:7mlabs:astrology:${name}:1.0`);
const time = schema('resolve-birth-time');
const local = schema('calculate-from-local');
const render = schema('render-chart-svg');
const birth = { local: { year: 1995, month: 6, day: 15, hour: 14, minute: 30, second: 0.125 }, timeZone: 'Asia/Ho_Chi_Minh' };
const location = { latitude: 10.8231, longitude: 106.6297 };
const subject = { ...birth, location };
const utcSubject = { utc: sdk.resolveBirthTime(birth).utc, location };
let checks = 0;
function check(validator, value, expected) {
  assert.equal(validator(value), expected, JSON.stringify(validator.errors)); checks++;
}
for (const value of [birth, { ...birth, disambiguation: 'earlier' }, { local: birth.local, utcOffsetSeconds: 25200 }]) {
  check(time, value, true); assert(sdk.resolveBirthTime(value).utc);
}
for (const value of [{}, { local: birth.local }, { ...birth, utcOffsetSeconds: 0 }, { ...birth, disambiguation: 'shift' },
  { ...birth, local: { ...birth.local, second: 60 } }, { ...birth, unknown: true }]) check(time, value, false);
const requests = [subject, { operation: 'natal', ...subject }, { operation: 'natalDomains', ...subject, domains: ['career'] }];
for (const operation of ['couple', 'composite']) {
  for (const [personA, personB] of [[subject, subject], [subject, utcSubject], [utcSubject, subject]]) requests.push({ operation, personA, personB });
}
requests.push({ operation: 'forecast', birth: subject, period: { kind: 'day', year: 2026, month: 3, day: 3 } });
for (const group of ['points', 'houses', 'aspects']) requests.push({ operation: 'query', group, action: 'inspect', birth: subject });
for (const request of requests) {
  check(local, request, true);
  assert.deepEqual(JSON.parse(sdk.calculateFromLocalJson(JSON.stringify(request))).errors, []);
}
for (const request of [{ ...subject, utc: utcSubject.utc }, { operation: 'natal', ...utcSubject },
  { operation: 'couple', personA: utcSubject, personB: utcSubject },
  { operation: 'composite', personA: utcSubject, personB: utcSubject },
  { operation: 'query', group: 'geometry', action: 'normalize', longitude: 5 },
  { operation: 'forecast', birth: utcSubject, period: { kind: 'day', year: 2026, month: 3, day: 3 } },
  { ...subject, unsupported: true }]) check(local, request, false);
const payload = sdk.calculateFromLocal(subject);
for (const options of [{}, { detail: 'advanced', theme: 'light', locale: 'vi' }, { bodyIds: ['sun'], maxAspectLines: 0 },
  { zodiacSymbols: { aries: { viewBox: [0, 0, 32, 32], nodes: [['path', { d: 'M0 0L10 10' }]] } } }]) {
  check(render, { payload, options }, true); assert(sdk.renderChartSvg(payload, options).startsWith('<svg'));
}
for (const options of [{ size: 127 }, { size: 4097 }, { detail: 'full' }, { bodyIds: [] }, { bodyIds: ['sun', 'sun'] },
  { maxAspectLines: -1 }, { unknown: true }, { instanceId: 'bad id' }, { zodiacSymbols: { ophiuchus: {} } }]) check(render, { payload, options }, false);
// Schemas describe structure; runtime enforces dates, IANA rules, geometry and SVG safety.
check(time, { ...birth, local: { ...birth.local, month: 2, day: 31 } }, true);
assert.throws(() => sdk.resolveBirthTime({ ...birth, local: { ...birth.local, month: 2, day: 31 } }));
console.log(JSON.stringify({ passed: true, checks, validators: 3, engineVersion: payload.engineVersion }));
