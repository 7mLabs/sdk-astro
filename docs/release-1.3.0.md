# Transit Range in Astro SDK 1.3.0

Use the same forecast JSON with Node.js, Python, .NET or the C ABI. A forecast for a day, month or year retains its existing output by default. Set `includeTransitRange: true` to add `data.transitRange`. This requires natal transits (enabled by default). A forecast can also start on a specified local date and cover 1–366 civil days, with the same fixed offset throughout.

## Integration example

[Complete public fixture](../examples/forecast-transit-range-request.json). It uses Mars, the synthetic natal in the annual example, the major aspects and the identity domain. The scan begins on 28 December 2026 at 00:00 UTC and ends on 7 January 2027 at 00:00 UTC (exclusive).

```js
const { calculate } = require('@7mlabs/astrology');
const request = require('../examples/forecast-transit-range-request.json');
const { windows, houseBoundaries, groups } = calculate(request).data.transitRange;
const marsSun = windows.find(w => w.targetPointId === 'N:sun' && w.angle === 120);
console.log(marsSun.exact.map(t => t.utc));
```

Mars enters this 8° trine window on 2 December 2026 at 04:23 UTC, reaches exactitude twice (7 January 2027 at 10:51 UTC and 13 January at 14:23 UTC), and leaves on 15 February at 14:50 UTC. Both hits occur after the requested slice. The window intersects it throughout. Its domain reference is `builtin:identity`. This fixture returns six windows and one overlap group. There are no Mars house crossings in the ten requested days.

## Reading the result

- `data.events` retains the existing exact aspects to all 26 fixed natal targets, including `N:H1` through `N:H12`. A square to a cusp is a geometrical aspect. A house boundary requires a conjunction to the cusp. `houseBoundaries` independently checks these conjunctions for every selected moving body, even when the configured aspect list is empty. Each record identifies the cusp, time, motion, house before and house after. A stationary touch has identical houses. Retrograde crossings reverse the order, including H1 → H12.
- Each window is a connected time interval in which the absolute difference between the angular separation and the configured aspect angle is at most `maxOrb`. The search splits longitude at velocity extrema and solves boundary, exact and turning contacts on continuous arcs. The `exact` list includes repeated passes and may be empty. `closest` reports the earliest minimum deviation across the window. `minimumOrb` is expressed in degrees. A zero-width orb can yield an instantaneous window.
- Results include windows intersecting the half-open query period. The engine extends its search in both directions until it finds the physical entry and exit. The supported ephemeris search span is 1800-01-01 to 2400-01-01 UTC. A null entry or exit denotes clipping at that limit. Every timestamp carries UTC civil components and a Julian day in UT1. Compare Julian days for ordering. To display only exact hits inside a query, filter on the normalized period bounds.
- `groups` are connected components of overlapping windows, clipped to the requested period. Endpoints that coincide count as contact. A chain A–B–C can form one group even when A and C do not coincide. Each group returns member window IDs and `themeEvidence`, mapping selected built-in/custom profile IDs to the windows whose fixed targets are in that profile's selected natal points or houses. Use the member windows' actual entry/exit times to show which transits are active at a particular instant. Window IDs are references within a response. Numerical search budgets return an error rather than a partial calculation.

The SDK supplies geometry and evidence. Applications choose the period, bodies, aspects, domains and presentation.
