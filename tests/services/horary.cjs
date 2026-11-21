'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '../..');
const sdk = require(process.env.ASTRO_ENGINE_MODULE || path.join(root, 'bindings/node'));
const Ajv = require(process.env.AJV_2020_MODULE || 'ajv/dist/2020');
const ajv = new Ajv({strict: true, allErrors: true});
for (const filename of fs.readdirSync(path.join(root, 'schemas'))) {
  ajv.addSchema(JSON.parse(fs.readFileSync(path.join(root, 'schemas', filename))));
}
const schema = name => ajv.getSchema('urn:7mlabs:astrology:' + name + ':1.0');
const check = (name, value, valid = true) => {
  const validate = schema(name);
  assert.equal(validate(value), valid, JSON.stringify(validate.errors));
};
const request = require('../../examples/horary-request.json');
check('horary-request', request);
const expected = sdk.calculate(request);
check('horary-response', expected);
assert.equal(expected.calculation.houseOrder, 'increasingLongitude');
assert.deepEqual(sdk.calculate({...request, ruleset: 'western-traditional-v1'}), expected);
const {utc, location} = request.moment;
const local = {...request, moment: {local: {...utc, hour: 15}, timeZone: 'Asia/Ho_Chi_Minh', location}};
check('calculate-from-local', local);
const resolved = sdk.calculateFromLocal(local);
check('horary-response', resolved);
assert.deepEqual(resolved.data, expected.data);
const beforeIngress = {...request, moment: {utc: {...utc, day: 20, hour: 3}, location}};
const voidResult = sdk.calculate(beforeIngress);
check('horary-response', voidResult);
assert.equal(voidResult.data.voidOfCourseMoon.isVoidOfCourse, true);
assert.equal(voidResult.data.voidOfCourseMoon.nextAspect, null);
assert.equal(expected.data.voidOfCourseMoon.nextAspect.otherBodyId, 'sun');
const lillyRequest = require('../../examples/horary-lilly-request.json');
check('horary-request', lillyRequest);
const lilly = sdk.calculate(lillyRequest);
check('horary-response', lilly);
const legacy = sdk.calculate({...lillyRequest, ruleset: 'western-traditional-v1'});
assert.deepEqual(lilly.data.chart.houseCusps, legacy.data.chart.houseCusps);
assert.equal(legacy.data.voidOfCourseMoon.isVoidOfCourse, true);
const trine = lilly.data.chart.aspects.find(a => a.body1 === 'sun' && a.body2 === 'saturn');
assert.equal(trine.angle, 120);
assert.equal(trine.maxOrb, 13.5);
assert.ok(Math.abs(trine.orb - 10.48029) < 1e-5);
assert.ok(!legacy.data.chart.aspects.some(a => a.body1 === 'sun' && a.body2 === 'saturn'));
const lunar = lilly.data.voidOfCourseMoon;
assert.equal(lunar.isVoidOfCourse, false);
assert.equal(lunar.hasSeparatingContact, true);
assert.equal(lunar.nextAspect.otherBodyId, 'saturn');
assert.equal(lunar.nextAspect.angle, 0);
assert.ok(lunar.nextAspect.julianDayUt1 > lunar.nextIngress.julianDayUt1);
assert.equal(lunar.nextAspect.utc.hour, 20);
assert.equal(lunar.nextAspect.utc.minute, 44);
const localLilly = {...lillyRequest, moment: {local: {...lillyRequest.moment.utc, hour: 10}, timeZone: 'Asia/Ho_Chi_Minh', location}};
check('calculate-from-local', localLilly);
assert.deepEqual(sdk.calculateFromLocal(localLilly).data, lilly.data);
assert.deepEqual(sdk.expandContext(sdk.compressPayload(lilly)), lilly);
assert.match(sdk.renderChartSvg(lilly, {chart: 'horary'}), /<svg/);
for (const [day, hour, separating, application] of [[1, 15, false, 'saturn'], [3, 0, true, undefined]]) {
  const utc = {...lillyRequest.moment.utc, day, hour};
  const chart = sdk.calculate({...lillyRequest, moment: {utc, location}});
  check('horary-response', chart);
  const moon = chart.data.voidOfCourseMoon;
  assert.equal(moon.hasSeparatingContact, separating);
  assert.equal(moon.isVoidOfCourse, true);
  assert.equal(moon.nextAspect, null);
  assert.equal(moon.applicationWithoutSeparation?.otherBodyId, application);
}
const wrongHouse = {...lillyRequest, moment: {...lillyRequest.moment, houseSystem: 'wholeSign'}};
check('horary-request', wrongHouse, false);
assert.equal(JSON.parse(sdk.calculateJson(JSON.stringify(wrongHouse))).errors[0].code, 'INVALID_INPUT');
const polarRequest = require('../../examples/horary-polar-request.json');
check('horary-request', polarRequest);
const polar = sdk.calculate(polarRequest);
check('horary-response', polar);
assert.equal(polar.calculation.houseSystem, 'regiomontanus');
assert.equal(polar.calculation.houseOrder, 'decreasingLongitude');
const southernMoment = {
  utc: {...polarRequest.moment.utc, hour: 4},
  location: {...polarRequest.moment.location, latitude: -polarRequest.moment.location.latitude},
};
const south = sdk.calculate({...polarRequest, moment: southernMoment});
check('horary-response', south);
assert.equal(south.calculation.houseOrder, 'decreasingLongitude');
// Direct Swiss house positions with ecliptic latitude fixed at zero.
const bodyHouses = result => Object.fromEntries(result.data.chart.placements.map(p => [p.id, p.house]));
assert.deepEqual(bodyHouses(polar), {sun: 4, moon: 11, mercury: 4, venus: 4, mars: 9, jupiter: 10, saturn: 11});
assert.deepEqual(bodyHouses(south), {sun: 10, moon: 5, mercury: 10, venus: 11, mars: 3, jupiter: 4, saturn: 5});
const polarModel = sdk.buildChartModel(polar, {chart: 'horary'});
assert.equal(polarModel.houses.length, 12);
assert.ok(polarModel.houses.every(house => house.signedWidth === -house.width));
assert.ok(Math.abs(polarModel.houses.reduce((total, house) => total + house.width, 0) - 360) < 1e-7);
assert.deepEqual(sdk.expandContext(sdk.compressPayload(polar)), polar);
const wholeSign = sdk.calculate({...polarRequest, moment: {...polarRequest.moment, houseSystem: 'wholeSign'}});
assert.equal(wholeSign.calculation.houseOrder, 'increasingLongitude');
assert.notDeepEqual(polar.data.chart.houseCusps, wholeSign.data.chart.houseCusps);
for (const patch of [{ruleset: 'unknown'}, {quesitedHouse: 0}, {quesitedHouse: 13}]) {
  const invalid = {...request, ...patch};
  check('horary-request', invalid, false);
  const failure = JSON.parse(sdk.calculateJson(JSON.stringify(invalid)));
  check('horary-response', failure);
  assert.equal(failure.errors[0].code, 'INVALID_INPUT');
}
console.log('Horary: rulesets, local time, lunar contacts, polar Regiomontanus models and schemas passed');
