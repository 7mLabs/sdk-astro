# Calculation metadata (1.1.1)

[Astro 1.1.2 — tài liệu](README.md) · [Công thức (VI)](features-and-formulas.vi.md) · [Quy tắc (VI)](rules.vi.md) · [English formulas](features-and-formulas.md)


The engine computes facts and declares the methods used. It does not select an
interpretation school for the consuming application. Read the individual rules;
`rulership: modern` selects the ruler table, while traditional sign-only dignity
remains explicitly labelled in `context.advanced.rules.dignity`.

Astronomical and query callers opt into a consistent `calculation.operation`
with `includeMetadata: true`. The option defaults to false so unchanged calls
continue to satisfy the frozen 1.1.0 response schemas, including closed query,
couple, composite, event and forecast contracts. Original chart-kind values
and their existing locations are retained. Basic natal and events additionally
expose calculation.chartKind when metadata is requested. Supplied geometry
reports its operation and unverified input provenance without assuming a sky.

`schemaVersion: 1.0` identifies the envelope contract, not the SDK release.
Current response schemas accept legacy and enriched shapes. Applications that
compare output byte-for-byte must account for engineVersion on every upgrade.

## Birth-based queries

With `includeMetadata: true`, `query` points/houses/aspects `inspect` retains the resolved source in
`calculation.sourceBirth` (UTC, latitude/longitude, effective house system), plus
`rulershipRulesVersion` and `houseAssignment`. When using the local-time helper,
`calculation.timeResolution.birth` separately retains the civil conversion.
The selected data, source indexes, numbers and selection policies are unchanged.

## Externally supplied geometry

JSON `chart`, `harmonic` and `synastry` requests optionally accept `sourceContext`;
`synastry` additionally accepts `otherSourceContext` for the second position set.

```json
{
  "operation": "chart",
  "positions": [{"id":"sun","longitude":280.2}],
  "sourceContext": {
    "zodiac":"tropical",
    "coordinates":"geocentric",
    "referenceFrame":"ecliptic-of-date",
    "sourceId":"my-calculator",
    "sourceVersion":"2.0"
  }
}
```

Optional fields: zodiac (`tropical` or `sidereal`), coordinates (`geocentric`,
`topocentric`, `heliocentric`), referenceFrame, houseSystem, sourceId,
sourceVersion and ayanamsa. A nonnull ayanamsa requires sidereal zodiac. At least
one nonnull nonblank field is required; each string is limited to 128 UTF-8 bytes.
Null optional members mean unknown and are omitted from the echoed declaration;
the entire sourceContext must be an object when supplied. No implicit ayanamsa,
frame, ruler table or astronomical provider is assumed.

`calculation.inputProvenance.positions` (and `.otherPositions` for synastry) is:

```json
{"status":"callerDeclared","verified":false,"context":{"zodiac":"tropical"}}
```

With no declaration it is:

```json
{"status":"unspecified","verified":false,"context":null}
```

This is metadata about the input coordinates. The engine does not verify,
convert or reconcile supplied frames, zodiac systems or house systems. In
harmonic mode the output is still a longitude transformation, not a newly
calculated astronomical sky. Declarations for two different position sets remain
separate. Applications must determine whether those inputs can be compared.
Pure angular query helpers continue to operate on supplied numbers without
claiming an astronomical configuration.

The public Rust `Request` struct and `calculate(Request)` signature are unchanged;
the new optional declarations are available through the JSON entry point shared
by Node, Python and .NET. Typed Rust geometry calls report unspecified provenance.

## Existing metadata retained

Astronomical outputs retain tropical/geocentric/apparent/ecliptic-of-date,
provider/version, effective house and aspect settings, time scales, units and
warnings. Composite retains its symbolic construction and point provenance;
Davison retains variant, time/place method, source calculations and correction
residuals. Domain/fact/forecast profiles retain their separate IDs and versions.
Compact context encoding preserves calculation metadata; scoped exports must
retain source metadata alongside selected data.
