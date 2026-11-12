<a href="../README.en.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — full logo"></a>

# Astro 1.1.2 calculation rules

[Tiếng Việt](rules.vi.md) · **English**

[SDK](../README.en.md) · [Features and formulas](features-and-formulas.md) · [Calculation rules](rules.md) · [Verification](formula-verification.md)

---

These are the effective conventions of **SDK 1.1.2**, not a new astrology school or a new payload rules ID. Domain, report, facts, provider and tzdb versions have separate meanings; read the values actually returned.

<details open>
<summary><strong>On this page</strong></summary>

- [Scope and defaults](#defaults)
- [Time and location](#input)
- [Explicit policies](#policies)
- [Reading payload provenance](#payload)
- [Distribution and compatibility](#compatibility)

</details>

<a id="defaults"></a>

## Scope and defaults

| Rule | Effective configuration |
| --- | --- |
| Zodiac | Tropical; birth API does not select Sidereal/ayanamsa. |
| Astronomical coordinates | Geocentric, apparent, ecliptic-of-date; no topocentric birth option. |
| Provider | Pinned Swiss 2.10.03; Moshier for Sun–Pluto; separate embedded data for optional Chiron. |
| Houses | Placidus default or explicit `wholeSign`. |
| Domain/report rulers | `traditional` default or `modern`; changes neither zodiac nor dignity. |
| Aspects | Basic natal: major through `aspectRules`. Domain/couple/composite/Davison: extended. Events/forecast: major. |
| Additional points | Explicit natal/natalDomains selection; separate from ten main bodies. |
| Composite | `houseMethod: midpoint`, `antipodalPolicy: error`. |
| Davison | `variant: uncorrected`; derived houses default to Placidus. |
| Additional metadata | `includeMetadata: false`; true opts in on supported astronomical/query operations. |

There is no single “school=modern” switch describing every policy. Traditional rulers + Whole Sign + extended aspects is a valid combination. Labels such as career/love identify 7mlabs selections, not an astrology school or an interpretation formula.

<a id="input"></a>

## Time and location

- Births use Gregorian dates, years **1800–2399**, valid calendar days, hour 0–23, minute 0–59 and finite second in `[0,60)`, default 0. Leap-second input 60 is unsupported.
- Geographic latitude is north-positive and strictly between −90/+90; longitude is east-positive within −180…180. `(0,0)` is valid. This differs from ecliptic longitude `[0,360)`.
- Supply UTC or use `calculateFromLocal`/`resolveBirthTime`; a birth must not contain both UTC and local input.
- Local conversion needs exactly one of an IANA `timeZone` or integer `utcOffsetSeconds` with absolute value <86400. Positive offsets are east of UTC. Location and machine settings do not infer timezone.
- Embedded tzdb is **2026e**. Repeated DST times default to rejection; explicitly select earlier/later. Nonexistent times always fail instead of moving across the gap. Both local and converted UTC years must remain in range.
- Swiss uses compiled UTC/TT/UT1 models. Before 1972 upstream treats civil input as UT1; future calculations lack live Earth-orientation corrections. Retain warnings and Julian days.
- `events/forecast.period.utcOffsetMinutes` is a **fixed offset −840…840 minutes**, default 0, separate from birth timezone. It does not follow DST changes during a period. Membership uses `[start,end)`; eclipses are selected by maximum time.

Integer birth/raw-JSON values are normalized exactly from decimal/exponent forms. Fractions, strings, booleans, unknown/duplicate fields or wrong types are rejected according to each API contract. Invalid times are not fixed by rounding. [Natal](natal.md), [time conversion](time-and-svg.md), [period/events](forecast.md).

<a id="policies"></a>

## Explicit system and policy choices

**Houses:** ecliptic-longitude cusp intervals, not a 3D house-position algorithm. Placidus failures do not silently select Whole Sign/Porphyry. Supplied geometry cusps must form twelve ordered nonduplicate boundaries traversing one zodiac cycle. Missing cusps do not imply invented houses.

**Aspects:** at most 16 unique-angle rules, angles 0–180° and maxOrb validated by the API. Preset and explicit rules are mutually exclusive. Pure geometry/query helpers operate on supplied numbers and do not infer a ruleset where explicit configuration is required.

**Domains:** natalDomains defaults to ten views; couple defaults to a separate six-view synastry catalog. Empty domains require nonempty custom profiles on operations supporting them. Custom selectors choose houses/bodies/angles, not executable user code/formulas. Evidence can be shared across views: do not sum view counts into a total or compatibility score.

**Mean/true and chart kind:** pair each south node with its matching mean/true north node; mean/osculating Lilith are distinct definitions. Composite has no physical motion; Davison recalculates physical velocities. They are not interchangeable interpretation inputs.

**Rulers and facts:** modern rulers change three outer-planet assignments; dignity remains traditional sign-only. Counts, patterns, dispositors and selectors are declared facts/policies, not narrative interpretations or aggregate strength scores. [Formula reference](features-and-formulas.md).

<a id="payload"></a>

## Read payload provenance in scope

| Part | Retain / inspect |
| --- | --- |
| Envelope | schemaVersion, engineVersion, warnings, errors; engine errors generally use null data. |
| `calculation` | Provider, epoch, effective houses/aspects and operation settings; opt into supported declarations with includeMetadata. |
| Composite/Davison provenance | Construction, A/B sources, variants/policies and correction residuals as applicable. |
| Domains/reports/facts | IDs/versions, selection reasons and evidence; join indexes only within the same response. |
| Local conversion | calculation.timeResolution or the time-helper response, civil input, timezone/offset and resolution policy. |
| Supplied geometry | With metadata enabled, inputProvenance is callerDeclared/unspecified and always verified=false. |
| SVG/model/context | Keep source metadata and read coverage/omissions after filtering/compression. |

A `sourceContext` may declare sidereal/topocentric positions calculated **outside** the engine. That does not enable sidereal/topocentric birth calculations. SDK declarations do not transform, verify or reconcile coordinate systems. [Metadata contract](payload-metadata.md).

Source f64 numbers differ from display degree/minute strings. Keep original numbers for subsequent calculations; rounding belongs to presentation. JSON spellings such as 0 and 0.0 may be semantically equal. Object helpers in Node/Python/.NET throw errors with code/result; raw JSON APIs keep the envelope. A successful result does not justify dropping warnings.

<a id="compatibility"></a>

## Distribution and compatibility

This documentation refresh retains **engine/package 1.1.2 and envelope 1.0**. Function names, defaults, schemas, wrappers, binaries and calculation algorithms are unchanged. Published packages and tag v1.1.2 remain immutable; main-branch documentation can be clarified later.

npm/Python/.NET binaries cover macOS ARM64 and Linux x64; .NET 10, Python ≥3.10 and Node ≥18. Linux requires glibc ≥2.38 and libgcc_s.so.1. [Installation matrix](../README.en.md#platforms), [release evidence](release-1.1.2.md).

Browser WASM, Sidereal/ayanamsa, topocentric natal, progression/return charts and other binary targets are not implemented. SVG is presentation and MCP/workers are adapters, not automatic extensions of calculation scope.
