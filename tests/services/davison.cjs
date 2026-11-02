'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const Ajv = require(process.env.AJV_2020_MODULE || 'ajv/dist/2020');
const sdk = require(process.env.ASTRO_ENGINE_MODULE || path.resolve(__dirname, '../../bindings/node'));
const root = path.resolve(__dirname, '../..');
const ajv = new Ajv({strict: true, allErrors: true});
for (const file of fs.readdirSync(path.join(root, 'schemas'))) ajv.addSchema(JSON.parse(fs.readFileSync(path.join(root, 'schemas', file))));
const requestSchema = ajv.getSchema('urn:7mlabs:astrology:davison-request:1.0');
const responseSchema = ajv.getSchema('urn:7mlabs:astrology:davison-response:1.0');
const localSchema = ajv.getSchema('urn:7mlabs:astrology:calculate-from-local:1.0');
const check = (schema, value, expected = true) => assert.equal(schema(value), expected, JSON.stringify(schema.errors));
for (const variant of ['uncorrected', 'corrected']) {
 const r = JSON.parse(fs.readFileSync(path.join(root, `examples/davison-${variant}-request.json`)));
 check(requestSchema, r);
 const v = sdk.calculate(r);
 check(responseSchema, v);
 assert.equal(v.calculation.variant, variant);
 assert.equal(v.data.domains.love.report.chartKind, 'davisonRelationship');
 assert(sdk.renderChartSvg(v).startsWith('<svg'));
 assert(sdk.buildChartModel(v).points.length);
 const compressed = sdk.compressPayload(v);
 assert.deepEqual(sdk.expandContext(compressed), v);
 const local = {...r, personA: {...r.personA, local: r.personA.utc, utcOffsetSeconds: 0}};
 delete local.personA.utc;
 check(localSchema, local);
 const lv = sdk.calculateFromLocal(local);
 check(responseSchema, lv);
 assert.deepEqual(lv.data, v.data);
 for (const patch of [{variant: 'spherical'}, {variant: null}, {houseMethod: 'midpoint'}, {houseSystem: 'invalid'}, {domains: []}, {aspectPreset: 'major', aspectRules: []}]) {
  const invalid = {...r, ...patch}; check(requestSchema, invalid, false);
  const error = JSON.parse(sdk.calculateJson(JSON.stringify(invalid))); check(responseSchema, error);
  assert.equal(error.errors[0].code, 'INVALID_INPUT');
 }
}
console.log('Davison: both variants, request/response/local schemas, Node, SVG/model and compression passed');
