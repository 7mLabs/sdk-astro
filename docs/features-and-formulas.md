<a href="../README.en.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — full logo"></a>

# Astro 1.1.2 features and formulas

[Tiếng Việt](features-and-formulas.vi.md) · **English**

[SDK](../README.en.md) · [Features and formulas](features-and-formulas.md) · [Calculation rules](rules.md) · [Verification](formula-verification.md)

---

This document describes **the current Astro SDK 1.1.2 implementation**. It introduces no new algorithms or payload fields. Paths refer to a successful engine envelope; the structure under `data` depends on the operation. API and JSON identifiers remain English in both documentation languages.

<details open>
<summary><strong>On this page</strong></summary>

- [Choose a scope](#operations)
- [Notation and geometry](#geometry)
- [Natal, houses and additional points](#natal)
- [Aspects](#aspects)
- [Rulers and derived facts](#facts)
- [Couple, composite and Davison](#relationships)
- [Events and forecast](#events)
- [Presentation and context](#presentation)
- [Examples and source mapping](#examples)

</details>

<a id="operations"></a>

## Choose a calculation scope

| Operation / service | Main input | Output |
| --- | --- | --- |
| `natal` | `utc`, `location`, optional house system | `data.placements`, `angles`, `houses`, `aspects`; optional `additionalPoints`. |
| `natalDomains` | Natal birth input and optional domains/profiles | `data.natal`, `context`, `domains`, `customDomains`, `profileCatalog`. |
| `couple` | `personA`, `personB` | Two natals, cross relations, overlays, 6 views; optional `data.composite`. |
| `composite` | Two births and midpoint policies | Symbolic chart C in `data.chart`, context/reports and provenance. |
| `davison` | Two births, optional variant | Recalculated `data.natal`, context/reports and `data.provenance`. |
| `events` | `period`, optional body/type filters | Events and snapshots; [contract](forecast.md). |
| `forecast` | `birth`, `period` | Fixed natal, transit snapshots, exact events, calendars and domain views. |
| `chart`, `harmonic`, `synastry` | Supplied positions; B/cusps/factor as required | Geometry, not an astronomical sky derived from a birth. |
| `query` | Helper-specific input | Angular calculations or selected facts; [8 helpers](query.md). |
| Time, SVG, compression | Civil-time request or existing payload | Separate services, not additional chart types. |

<a id="geometry"></a>

## Notation and geometry

`λ` is ecliptic longitude in degrees; `v` is longitude speed in degrees/day; `N(x)` wraps into `[0,360)`; `D(a,b)` is a signed difference in `[-180,180)`.

```text
N(x) = nonnegative remainder of x / 360
D(a,b) = N(N(a) - N(b) + 180) - 180
separation(a,b) = abs(D(a,b))
signIndex = floor(N(λ) / 30)          // 0 = Aries, ... 11 = Pisces
degreeInSign = N(λ) mod 30
isRetrograde = (v < 0)               // null without physical speed
```

Implementation uses `f64` and guards rounding near 0/360 and the ±180 cut. A sign name is not an interpretation. Examples: `N(-10)=350`, `separation(350,10)=20`, and `280°` is Capricorn `10°`.

Harmonic: `λ' = N(λ × factor)`, `v' = v × factor` when speed is supplied; integer factor 1–360. Natal houses are not retained on transformed positions. `350 × 5 → 310°`. [Geometry implementation](../neutral-engine/crates/astro-core/src/lib.rs).

<a id="natal"></a>

## Natal, houses and additional points

Sun–Pluto positions are not a linear formula implemented in the SDK. The provider calls Swiss with Moshier/speed flags, converts UTC to TT/UT1 and obtains longitudes, latitudes, AU distances and speeds for the epoch. The configuration is tropical/geocentric/apparent/ecliptic-of-date. [Provider](../neutral-engine/crates/astro-provider-swiss/src/lib.rs), [pinned provenance](../neutral-engine/crates/astro-provider-swiss/PROVENANCE.md).

| Data | Rule | Basic natal path |
| --- | --- | --- |
| Bodies | Provider coordinates; derived sign, degree, retrograde and house | `data.placements[]` |
| ASC, MC | Swiss calculation from UT1 and geography | `data.angles[]` |
| DSC, IC | Antipodes of ASC and MC | `data.angles[]` |
| Placidus cusps | Swiss Placidus house calculation, not equal 30° sectors | `data.houseCusps`, `houses[]` |
| Whole Sign cusps | ASC's sign starts house 1; subsequent cusps advance 30° | Same house fields |
| House assignment | House i satisfies `N(λ - cusp[i]) < N(cusp[next] - cusp[i])` | `placements[].house` |
| Additional points | Explicitly selected definitions, separate from ten main bodies | `data.additionalPoints[]` |

House assignment uses **ecliptic-longitude cusp intervals**, inclusive at the starting cusp and exclusive at the next. It does not apply a 3D house-position calculation using body latitude. Geographic poles are rejected; polar Placidus failure does not silently select another system. [Input rules](rules.md).

`additionalPoints: "standard"` selects `chiron`, `trueNorthNode`, `meanLilith`; `"all"` selects seven points: Chiron, mean/true north and south nodes, mean/true Lilith. South Node longitude is `N(λNorth + 180)` for its matching north-node definition. Chiron uses embedded `seas_18.se1`; Lilith here means the mean/osculating **lunar apogee**, not the similarly named asteroid. These options belong to natal/natalDomains and do not silently alter ten-body distributions or rulership. [Definitions and implementation](../neutral-engine/crates/astro-core/src/natal.rs).

<a id="aspects"></a>

## Aspects and applying/separating state

For angle `α`, `orb = abs(separation - α)`. A rule matches when `orb <= maxOrb`, including the boundary. Every matching rule is retained, even if configured orbs overlap.

| Preset | Angle → maxOrb in degrees |
| --- | --- |
| `major` / basic natal default | 0→8, 60→6, 90→8, 120→8, 180→8 |
| `extended` | Major plus 30→2, 45→2, 72→2, 135→2, 144→2, 150→3 |

Basic natal accepts `aspectRules`, not `aspectPreset`. Domain/couple/composite/Davison default to extended; events/forecast default to major. Preset and explicit rules are mutually exclusive. `aspectRules: []` disables matches, not every geometric pair relation.

For two points in the **same physical chart** with speeds and orb above `1e-12°`:

```text
δ = D(λ1, λ2)
distanceRate = (v1 - v2) × sign(δ)
applying = ((abs(δ) - α) × distanceRate < 0)
```

Exact-within-tolerance, missing speed, cross-chart synastry and symbolic composite use `applying: null` as applicable. A `91°` separation from a `90°` rule has `1°` orb; decreasing separation is applying. This is an instantaneous velocity test, not a future exact-event search.

<a id="facts"></a>

## Rulers, facts and selections

| Fact | Rule |
| --- | --- |
| Element/modality/polarity distributions | Count ten bodies by sign; no implicit personality weighting or score. |
| Traditional rulers | Aries→Mars, Taurus→Venus, Gemini→Mercury, Cancer→Moon, Leo→Sun, Virgo→Mercury, Libra→Venus, Scorpio→Mars, Sagittarius→Jupiter, Capricorn/Aquarius→Saturn, Pisces→Jupiter. |
| Modern rulers | Replace Scorpio→Pluto, Aquarius→Uranus, Pisces→Neptune; retain other traditional assignments. |
| Dispositor chain | Follow each body's sign ruler until self-rulership or a detected cycle. |
| Mutual reception | Each body occupies a sign ruled by the other under the selected table. |
| Dignity | **Traditional sign-only** domicile/detriment/exaltation/fall for seven classical bodies; unsupported outer-body flags are not invented. |
| Solar condition | Separation from Sun: cazimi ≤17′; combust through 8.5°; underSunbeams through 17°; otherwise free, with `1e-12°` boundary tolerance. Sun itself is notApplicable. |
| Aspect patterns | Topology of configured matched body-to-body aspect angles, not visual similarity. |
| Domain/custom profile | Select houses/bodies/angles, expand through occupants/rulers and retain evidence; no separate sky per domain. |

These rules are not the entirety of traditional or modern astrology. Modern rulers do not change dignity or add triplicity/term/face or strength scores. [Derived facts](../neutral-engine/crates/astro-core/src/advanced.rs), [domain implementation](../neutral-engine/crates/astro-core/src/domains.rs), [catalog/selectors](profiles.md), [report/evidence](individual-reports.md).

<a id="relationships"></a>

## Couple, composite and Davison

**Couple/synastry:** calculate each natal independently, then cross-chart separations/orbs. Overlay A longitudes into B cusps and vice versa. Cross-chart comparisons do not infer applying from two natal velocities. Couple has 6 views, distinct from the 10 individual/composite/Davison domains. [Contract](couple.md).

**Composite:** for normalized matching source longitudes:

```text
lo = min(λA, λB); hi = max(λA, λB)
mid = N(lo + (hi-lo)/2 + (180 if hi-lo > 180, else 0))
```

Antipodal sources within `1e-10°` have two midpoints: default is an error; explicit `antipodalPolicy: "lowerLongitude"` selects the smaller normalized midpoint. `350°/10° → 0°`, not 180°. `houseMethod: "midpoint"` constructs cusp midpoints; `"wholeSignFromMidpointAscendant"` derives Whole Sign cusps from midpoint ASC. Invalid cusp order fails. C is symbolic: **no physical UTC/JD, speed, retrograde or applying state**. [Contract](composite.md), [source](../neutral-engine/crates/astro-core/src/composite.rs).

**Uncorrected Davison:** average Julian UT1 instants, and separately average latitudes and signed longitudes arithmetically. `+179°` and `−179°` produce **0°**, not a dateline midpoint. Recalculate the astronomical sky at the derived time/place.

**Corrected Davison:** retain mean geography and shift time to match the shortest-arc midpoint of the two natal MCs. Search is bounded to half a mean solar day and 48 bisections, with MC residual ≤`1e-6°`. Antipodal MCs or convergence failure produce an error. **All** bodies, houses, angles and speeds are recalculated; planetary positions are not averaged as in composite.

Default variant is uncorrected; derived houses default to Placidus. Arithmetic mean geography is not spherical/reference-place Davison. [Contract/provenance](davison.md), [implementation](../neutral-engine/crates/astro-core/src/davison.rs).

<a id="events"></a>

## Events and forecast

| Type | Search condition |
| --- | --- |
| Ingress | Longitude crosses a multiple-of-30° sign boundary, including retrograde crossings. |
| Station | Longitude speed changes sign through zero. |
| Lunar phase | `N(λMoon - λSun)` reaches 0/90/180/270°. |
| Planetary aspect | Two moving longitudes reach a configured angle/branch. |
| Natal transit | Moving longitude reaches an angle to a **fixed** natal target. |
| Eclipse | Swiss global eclipse maximum, not inferred solely from new/full Moon. |

The scanner uses a six-hour grid, relative-velocity extrema and provider-based root refinement. Angular residual ≤`1e-6°`, time bracket ≤`0.25 s`, station residual ≤`1e-8°/day`. These are numerical solver tolerances, **not** certified real-sky accuracy. Global eclipse data does not establish local visibility.

`period.utcOffsetMinutes` is fixed and does not follow birth IANA/DST changes. Highlights, calendars and domain contexts select/organize evidence rather than write predictions. [Contract/limits](forecast.md), [scanner](../neutral-engine/crates/astro-core/src/events.rs), [forecast source](../neutral-engine/crates/astro-core/src/forecast.rs).

<a id="presentation"></a>

## SVG, models and context

The renderer consumes an existing payload without calling Swiss to change positions. viewBox coordinates, colors, aspect filters and house focus are presentation; models retain source longitudes. [Time/SVG](time-and-svg.md).

Compression runs after calculation. Lossless compact mode can be expanded; scoped/domain/section or budgeted exports can omit data and require reading coverage/omissions. Not every compressed mode is lossless. [Compression contract](payload-compression.md).

<a id="examples"></a>

## Examples and source mapping

- [Natal request](examples/natal-request.json), [actual 1.1.2 response](examples/natal-response.json), [geometry request](examples/geometry-request.json).
- [Defaults and rules](rules.md).
- [Sources, verification types and interpretation of evidence](formula-verification.md).

Metadata does not guarantee correct AI interpretation. Consumers must retain calculation settings, provenance and warnings and select a compatible knowledge base.
