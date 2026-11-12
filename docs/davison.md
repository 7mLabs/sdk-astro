# Davison relationship charts — SDK 1.1.2

[Astro 1.1.2 — tài liệu](README.md) · [Công thức (VI)](features-and-formulas.vi.md) · [Quy tắc (VI)](rules.vi.md) · [English formulas](features-and-formulas.md)


Available since 1.1.0; SDK 1.1.2 supports both `uncorrected` and `corrected` through the existing
`calculate` / `calculateJson` entry points. Version **1.0.2 does not contain Davison**; install 1.1.2 for the current release. No new public binding function or
ABI export is required. Node, Python, .NET and C all pass the same JSON to Rust.

```json
{
  "operation": "davison",
  "variant": "corrected",
  "personA": {
    "utc": { "year": 1985, "month": 5, "day": 8, "hour": 7, "minute": 30 },
    "location": { "latitude": 10.8231, "longitude": 106.6297 }
  },
  "personB": {
    "utc": { "year": 1990, "month": 8, "day": 12, "hour": 5, "minute": 10 },
    "location": { "latitude": 21.0278, "longitude": 105.8342 }
  },
  "houseSystem": "placidus",
  "domains": ["love", "relationships"]
}
```

## Method and defaults

- `variant`: `uncorrected` by default, or `corrected`. No implicit choice of corrected.
- Geography: arithmetic mean of the two latitudes and of the two signed longitudes.
  This follows the definitions of corrected/uncorrected DRC in
  [Astrodienst's chart-type documentation](https://www.astro.com/cgi/h.cgi?f=gch&h=gch65&lang=e).
  Longitudes +179° and −179° produce 0°, **not** 180°. Spherical midpoint,
  shortest-arc geographical midpoint and reference-place Davison are different
  conventions and are not implemented by these two variants.
- Time: convert each validated birth to Julian UT1 using Swiss Ephemeris, then
  average those two instants. It is not an average of displayed local clocks.
  Swiss UTC/UT1 conversion uses its built-in leap seconds and Delta T; exact
  agreement with tools using another time scale or Earth-orientation model is
  not guaranteed. Calendar range and leap-second limitations match natal.
- Corrected: find the closest time along the shortest MC rotation to the
  shortest-arc midpoint of the two natal MC longitudes, keeping mean geography.
  All planets, angles and houses are recalculated at the corrected instant.
  Search is bounded to half a mean solar day and 48 bisections, with MC residual
  at most 0.000001°. Antipodal source MCs fail explicitly because the target is
  not unique. Failure never falls back silently to uncorrected.
- `houseSystem`: `placidus` by default or `wholeSign` for the derived chart.
  Each source birth retains its own house-system option. Polar/invalid houses
  fail explicitly; the engine does not silently change the selected system.
- `domains`, `customProfiles`, `rulership`, `aspectPreset` and `aspectRules` use
  the same rules as individual/composite domain reports. Default: ten domains,
  traditional rulership, extended aspects. Preset and explicit rules are mutually
  exclusive. Additional points are not currently accepted in Davison births.

## Result and rendering

`data.natal` contains the recalculated chart with derived UTC/place, physical
planetary speeds and retrograde flags. `data.context`, `domains`, `customDomains`
and `profileCatalog` reuse the existing report structures, with
`chartKind: "davisonRelationship"` and `subjectCount: 2` at the data root.
This is a derived relationship chart, not either person's natal or the date of
an actual event. Reports remain calculation facts, without compatibility scores
or narrative interpretation.

`data.provenance` records both source births/calculations, variant, geographic
convention, UT1 midpoint, derived UT1, time correction in seconds, target MC,
residual, iteration count and tolerance. Source identifiers A/B refer to the two
inputs; the output does not duplicate full source charts. Use `couple` separately
when source natal/synastry reports are wanted.

`calculateFromLocal` accepts local/IANA or offset births exactly as `couple` does.
Use the existing SVG/model adapter with `chart: "natal"` (also the default) to draw
the derived chart; its context supports house focus. Lossless context compression
uses the existing envelope without a Davison-specific encoding.

Requests reject unknown/duplicate fields. Errors use the existing envelope:
`INVALID_INPUT` for validation and `CALCULATION_FAILED` for provider, ambiguity or
convergence failures; no partial success is returned.

Contracts: [request schema](../schemas/davison-request.schema.json),
[response schema](../schemas/davison-response.schema.json),
[uncorrected example](../examples/davison-uncorrected-request.json),
[corrected example](../examples/davison-corrected-request.json).

Validation covers identity, input swap symmetry, physical natal reproduction,
MC target, arithmetic dateline geography, invalid inputs, local time, SVG/house
focus and compression. A [64-chart comparison](reviews/2026-11-02-davison/README.md) passed separate Swiss assembly and independent Skyfield/JPL position/angle checks. This comparison does not claim exact parity with every commercial Davison product or every input. Release-platform and installed-package checks are recorded in the 1.1.0 release evidence.

## TypeScript compatibility

`Request` retains its published 1.0.x union and calculate overload. Existing source does not need to add Davison to exhaustive switches or return-type unions. Use `DavisonRequest` for this method, or the new `CalculationRequest` union when dynamically selecting any operation.
