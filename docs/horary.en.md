# Horary question chart

[Tiếng Việt](horary.md) · [Request schema](../schemas/horary-request.schema.json) · [Response schema](../schemas/horary-response.schema.json)

In SDK **1.2.0**, pass the time and place of the question to `calculate`. Select the house that represents the matter asked about with `quesitedHouse`, an integer from 1 through 12. The response provides astronomical positions, houses, significators and the Void-of-Course Moon indicator for interpretation by the caller.

```js
const { calculate } = require('@7mlabs/astrology');

const result = calculate({
  operation: 'horary',
  moment: {
    utc: { year: 2026, month: 11, day: 19, hour: 8, minute: 0 },
    location: { latitude: 10.8231, longitude: 106.6297 }
  },
  quesitedHouse: 7,
  ruleset: 'western-traditional-v1'
});

const { querent, quesited } = result.data.significators;
console.log(querent.rulerBodyId, quesited.rulerBodyId);
console.log(result.data.voidOfCourseMoon.nextAspect);
```

House 1 represents the querent. This example uses house 7 for the quesited at the caller’s choice. Each significator is the traditional domicile ruler of the sign on its house cusp. Both roles can have the same ruler. The Moon is identified separately as `moon`.

For this input, Aries on the first cusp gives Mars for the querent. Libra on the seventh cusp gives Venus for the quesited. The Moon completes a 120° aspect with the Sun at about 01:46 UTC on 20 November 2026, before entering Aries at about 05:52 UTC that day. The resulting `isVoidOfCourse` flag is `false`.

## Ruleset

- Omitting `ruleset` selects `western-traditional-v1`. Callers can also request this fixed ruleset by ID. Unsupported IDs raise a validation error. The response always identifies the effective rules in `calculation.ruleset` and `calculation.rulesetVersion`.
- The question chart uses tropical, geocentric positions. `moment.houseSystem` defaults to `regiomontanus` and also accepts `placidus` and `wholeSign`. The effective system appears in `calculation.houseSystem`.
- `data.chart` returns the seven classical bodies, four angles and twelve houses. The five body-to-body angles are 0°, 60°, 90°, 120° and 180° with maximum orbs of 8°, 6°, 8°, 8° and 8°. The `applying` flag describes motion at the question instant.
- `data.voidOfCourseMoon` uses `exactMajorAspectBeforeNextSignIngress`. The SDK finds the first exact major aspect the Moon makes with the Sun, Mercury, Venus, Mars, Jupiter or Saturn after the question instant and before the Moon next enters a sign. If found, `nextAspect` gives the other body, angle and time. Otherwise `nextAspect` is `null` and `isVoidOfCourse` is `true`. `nextIngress` always gives the next sign and ingress time. Both events report UTC and the UT1 Julian day. An exact contact within one millisecond of either endpoint counts as coincident with that endpoint and is excluded. The rules above define the scope of `western-traditional-v1`. The caller interprets the question.

At polar latitudes Regiomontanus houses 1 through 12 can run in decreasing ecliptic longitude. The SDK preserves the Swiss Ephemeris cusps and uses their direction for assigning bodies, identifying rulers and rendering the chart. [Swiss Ephemeris, section 6.4](https://www.astro.com/swisseph/swisseph.htm)

`calculation.houseOrder` reports `decreasingLongitude` for this case and `increasingLongitude` for the usual order. The `houseCusps` array always follows house numbers 1–12. A body exactly on a cusp belongs to the house starting at that cusp. Rendered house models expose positive `width` and the directed span in `signedWidth`. If cusps coincide and fail to define 12 distinct houses, the calculation returns an error. Input latitude must be greater than −90° and less than 90°. [Longyearbyen example](../examples/horary-polar-request.json)

For local civil time, use `calculateFromLocal` with `moment.local`, `moment.location` and either `moment.timeZone` or `moment.utcOffsetSeconds`. The response records the conversion in `calculation.timeResolution.moment`. [Local-time conventions](time-and-svg.md)

`renderChartSvg(result)` draws the question chart directly from the response. To highlight house 7 in the question layer (`Q`), use `renderChartSvg(result, { chart: 'horary', focus: { house: 7, layer: 'Q' } })`. For a custom UI, use `buildChartModel(result)`. [Example JSON request](../examples/horary-request.json)

For historical differences in the definition of Void-of-Course, see [Deborah Houlding, The Moon as the transmitter of influences](https://www.skyscript.co.uk/moon2.html).
