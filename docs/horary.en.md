# Horary question chart

[Tiếng Việt](horary.md) · [Request schema](../schemas/horary-request.schema.json) · [Response schema](../schemas/horary-response.schema.json)

In SDK **1.2.0**, pass the time and place of the question to `calculate`. Select the house that represents the matter asked about with `quesitedHouse`, an integer from 1 through 12. The response provides astronomical positions, houses, significators and the Void-of-Course Moon indicator for interpretation by the caller.

## Choose a ruleset

| What does the application need? | Select `ruleset` | How to use the result |
| --- | --- | --- |
| Reproduce a chart from SDK 1.2.0–1.2.1 or look up aspects with fixed orbs | `western-traditional-v1` (default) | Use `nextAspect` to answer “Which major lunar contact will perfect before the Moon changes sign?” A null value means there is no qualifying contact in that interval. |
| Read a chart using the Lilly rules implemented by the SDK | `lilly-1647-v1` | Select this ID when casting the chart to apply its planetary orb table and lunar contact criteria. [Sources and scope](release-1.2.2.md) |

`western-traditional-v1` names a combination of rules defined by 7mlabs. The SDK does not attribute the combination to a particular historical author or school. If you expose this option in an application, label it **SDK 1.2.0–1.2.1 conventions** under calculation options. In a school selector, **William Lilly** maps to `lilly-1647-v1`.

When reading a saved chart, check `calculation.ruleset` to interpret its indicators. To read an older chart using the Lilly profile, recalculate with the same question moment, location and quesited house, selecting `lilly-1647-v1` and Regiomontanus houses. Each profile’s `isVoidOfCourse` describes the Moon under its stated criteria. The application supplies any judgment about the question itself.

## Example using the SDK conventions

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

## Lilly profile in 1.2.2

Pass `ruleset: 'lilly-1647-v1'` to use Regiomontanus, planetary half-orbs, lunar in-orb continuity and the distance to the following cusp within 5°. The result identifies the selected method. [Exact rules and reference chart](release-1.2.2.md)

## Details of the default `western-traditional-v1` ruleset

- Omitting `ruleset` selects `western-traditional-v1`. Callers can also request this fixed ruleset by ID. Unsupported IDs raise a validation error. The response always identifies the effective rules in `calculation.ruleset` and `calculation.rulesetVersion`.
- The question chart uses tropical, geocentric positions. `moment.houseSystem` defaults to `regiomontanus` and also accepts `placidus` and `wholeSign`. The effective system appears in `calculation.houseSystem`.
- `data.chart` returns the seven classical bodies, four angles and twelve houses. The five body-to-body angles are 0°, 60°, 90°, 120° and 180° with maximum orbs of 8°, 6°, 8°, 8° and 8°. The `applying` flag describes motion at the question instant.
- `data.voidOfCourseMoon` uses `exactMajorAspectBeforeNextSignIngress`. The SDK finds the first exact major aspect the Moon makes with the Sun, Mercury, Venus, Mars, Jupiter or Saturn after the question instant and before the Moon next enters a sign. If found, `nextAspect` gives the other body, angle and time. Otherwise `nextAspect` is `null` and `isVoidOfCourse` is `true`. `nextIngress` always gives the next sign and ingress time. Both events report UTC and the UT1 Julian day. An exact contact within one millisecond of either endpoint counts as coincident with that endpoint and is excluded. The rules above define the scope of `western-traditional-v1`. The caller interprets the question.

At polar latitudes Regiomontanus houses 1 through 12 can run in decreasing ecliptic longitude. The SDK preserves the Swiss Ephemeris cusps and uses their direction for assigning bodies, identifying rulers and rendering the chart. [Swiss Ephemeris, section 6.4](https://www.astro.com/swisseph/swisseph.htm)

`calculation.houseOrder` reports `decreasingLongitude` for this case and `increasingLongitude` for the usual order. The `houseCusps` array always follows house numbers 1–12. A body exactly on a cusp belongs to the house starting at that cusp. Rendered house models expose positive `width` and the directed span in `signedWidth`. The calculation reports an error when all 12 cusps converge on a single axis within 0.0000001°. Individual houses can be narrower than this tolerance. Input latitude must be greater than −90° and less than 90°. [Longyearbyen example](../examples/horary-polar-request.json)

For local civil time, use `calculateFromLocal` with `moment.local`, `moment.location` and either `moment.timeZone` or `moment.utcOffsetSeconds`. The response records the conversion in `calculation.timeResolution.moment`. [Local-time conventions](time-and-svg.md)

`renderChartSvg(result)` draws the question chart directly from the response. To highlight house 7 in the question layer (`Q`), use `renderChartSvg(result, { chart: 'horary', focus: { house: 7, layer: 'Q' } })`. For a custom UI, use `buildChartModel(result)`. [Example JSON request](../examples/horary-request.json)

For historical differences in the definition of Void-of-Course, see [Deborah Houlding, The Moon as the transmitter of influences](https://www.skyscript.co.uk/moon2.html).

## Development after 1.2.2

### SDK conventions · `western-traditional-v1`

This profile serves saved-chart reproduction and fixed-orb lookup. For example, a calendar can show the next exact major lunar contact before ingress and its time. New applications can use this lookup independently of any historical chart data.

Proposed maintenance covers defects against the specification, performance and ingress-boundary tests. If applications need every contact before ingress, an optional query could expose the sequence. Changing an orb or the definition of void of course requires a new ruleset ID. Any future retirement decision should account for saved results and calls that omit `ruleset` before removing the ID.

### William Lilly · `lilly-1647-v1`

**Shipped in 1.2.2.** The SDK casts a Regiomontanus chart, identifies the rulers of the querent’s and caller-selected quesited houses, applies planetary pair orbs to five major aspects, reports proximity within 5° of the following cusp, and computes the documented lunar indicator. [Sources, computational conventions and reference chart](release-1.2.2.md)

| Proposed milestone | Data available to the application | Release checks |
| --- | --- | --- |
| Proposed · Dignities, reception and turned houses | Dignities by sign, degree and day/night where applicable. Directed reception with its grounds and mutuality. For turned houses, the caller selects the role and base house; the SDK counts houses, returns their rulers and identifies shared rulers. | Compare against Lilly’s dignity table on p. 104. Test both sides of degree boundaries, day/night, one-way and mutual reception, and counting past house 12. [Facsimile of Lilly’s table](https://www.skyscript.co.uk/ca_downloads.html) |
| 1.4.0 · Aspect history and perfection | For selected significators, return recent exact aspects and the order of upcoming contacts within a stated search interval. Identify direct perfection, translation and collection of light, prohibition, frustration and refranation from the event sequence and each mechanism’s documented conditions. | For each mechanism, cite the rule and test a qualifying example and a near miss. Cover sign changes, reversals of motion and applications still open at the search boundary. [William Lilly, Christian Astrology](https://lib.wildla.in/books/Christian%20Astrology%201-2%20by%20William%20Lilly.pdf) |

These are proposed milestones for computable Lilly indicators. Each released indicator should name its source, identify any convention selected by the SDK and expose the evidence needed to check the result. The application identifies the subjects of the question and weighs the indicators when forming a judgment.
