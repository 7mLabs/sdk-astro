<a href="../README.en.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — full logo"></a>

# Astro 1.1.3 formula sources and verification

[Tiếng Việt](formula-verification.vi.md) · **English**

[SDK](../README.en.md) · [Features and formulas](features-and-formulas.md) · [Calculation rules](rules.md) · [Verification](formula-verification.md)

---

This reference distinguishes **declared conventions**, **implementation parity** and **independent comparison**. A badge or provider name is not evidence by itself. It applies to SDK 1.1.3, whose licensing packaging update leaves public APIs, calculations and schemas unchanged. Evidence from earlier releases remains explicitly historical.

<details open>
<summary><strong>On this page</strong></summary>

- [Sources by calculation family](#sources)
- [Reproducible arithmetic examples](#arithmetic)
- [Existing evidence](#evidence)
- [Comparing another calculator](#comparison)
- [Documentation and versioning](#documentation)

</details>

<a id="sources"></a>

## Sources by calculation family

| Scope | Implementation source of truth | Verification |
| --- | --- | --- |
| Normalize, signed delta, signs, houses, orb, applying, harmonic | [astro-core/lib.rs](../neutral-engine/crates/astro-core/src/lib.rs) | Module unit tests and cross-binding fixtures. |
| Provider, TT/UT1, houses and optional data | [provider](../neutral-engine/crates/astro-provider-swiss/src/lib.rs), [PROVENANCE](../neutral-engine/crates/astro-provider-swiss/PROVENANCE.md), [natal](../neutral-engine/crates/astro-core/src/natal.rs) | Provider checks, natal cases, pinned flags/data/checksums. |
| Mean/true point definitions | [natal.rs](../neutral-engine/crates/astro-core/src/natal.rs) | South-node antipodes, separate definitions and Chiron data. |
| Rulers, dignity, patterns and selectors | [advanced](../neutral-engine/crates/astro-core/src/advanced.rs), [domains](../neutral-engine/crates/astro-core/src/domains.rs), [profiles](profiles.md) | Topology/boundary tests and declared policies. |
| Composite | [composite.rs](../neutral-engine/crates/astro-core/src/composite.rs) | Swap symmetry, midpoint/cusp order, antipodal policies and null physical fields. |
| Davison | [davison.rs](../neutral-engine/crates/astro-core/src/davison.rs) | Epoch/place, MC residual, swap symmetry and natal reproduction. |
| Events/forecast | [events.rs](../neutral-engine/crates/astro-core/src/events.rs), [forecast.rs](../neutral-engine/crates/astro-core/src/forecast.rs) | Roots/boundaries/repeated hits and phase/eclipse references. |
| Civil time | [astro-time](../neutral-engine/crates/astro-time/src/lib.rs) | Folds/gaps, offsets, history, year limits and pinned tzdb. |

External references explain definitions, not automatic public-API support for everything upstream implements:

- [Swiss programming interface](https://www.astro.com/swisseph/swephprg.htm): provider calls, time scales and houses. Actual SDK behavior is determined by pinned source and effective flags.
- [Astrodienst Davison variants](https://www.astro.com/cgi/h.cgi?f=gch&h=gch65&lang=e): corrected/uncorrected and arithmetic versus spherical/reference-place geography. SDK UT1 and MC-solver choices are specified in its [contract](davison.md).

Project orb defaults, selectors and sign-only dignity are not a universal standard shared by every school. The project rules and implementation are the direct references for those policies.

<a id="arithmetic"></a>

## Reproducible arithmetic examples

| Input | Expected result | Scope |
| --- | --- | --- |
| Longitude −10° | 350°, Pisces 20° | Normalize/sign |
| 350° and 10° | Separation 20°; midpoint 0° | Non-antipodal geometry |
| 10°, 101°, angle 90° | Separation 91°, orb 1° | Match when maxOrb ≥1° |
| Same chart: λ1=0, v1=1; λ2=91, v2=0; angle=90 | Applying true, orb 1° | Instantaneous speed test |
| λ=350°, factor=5 | λ′=310° | Harmonic, not a physical sky |
| Davison signed longitudes +179°, −179° | Mean longitude 0° | Arithmetic, not spherical geography |

These examples are not personal birth records. [Geometry request](examples/geometry-request.json) and [actual natal response](examples/natal-response.json) expose concrete shapes. Angular midpoint and arithmetic geographical mean are not interchangeable.

<a id="evidence"></a>

## Existing evidence

The [1.1.3 release record](release-1.1.3.md) tracks checks for the actual new packages. Its status and reports determine which checks have completed; the records below are not a substitute for final 1.1.3 artifact or public-install verification.

Historical and source-level evidence:

1. [1.1.2 release report](release-1.1.2.md): 12 candidate checks and npm publication passed; Linux/macOS, Python 3.10/3.13, Node 18/24; 339 conformance cases in final consumers. Fresh public npm/PyPI/NuGet installations and artifact checksums are recorded.
2. [64-chart Davison review](reviews/2026-11-02-davison/README.md): separate Swiss assembly and independent Skyfield/JPL position/angle checks. Same-provider assembly parity is distinguished from independent-provider comparison; scope/tolerances belong to the reviewed cases.
3. [Testing guide](testing.md) and [source tests](../neutral-engine/crates/astro-core/tests/): natal/calendar/geometric boundaries, concurrency, response schemas, reports and event search. Moon/eclipse fixture provenance is recorded with the tests.
4. [Public release evidence](https://7mlabs.com/astro/sdk/docs/1.1.2/downloads/verification.json): commit/tag, checked packages and installed-consumer scope.

Cross-binding conformance checks wrapper consistency on the cases, not four independent astronomical implementations. Numerical root tolerances do not certify absolute ephemeris/time-model accuracy. Rebuilding the same source is distinct from independent reference verification.

<a id="comparison"></a>

## Comparing another calculator

First align Gregorian date, UTC/offset/IANA and DST resolution; geography; tropical versus sidereal; apparent/reference frame; houses; mean/true points; rulers; aspect angles/orbs; composite versus Davison/variant; time scale and geographic convention. Compare raw numbers before display rounding.

For remaining differences, retain a minimal request, response metadata/warnings, engine/provider versions and the other calculator's settings. Do not change algorithms merely to match one website without identifying policy differences. Geometry verified=false records caller declarations, not astronomical verification.

<a id="documentation"></a>

## Documentation and versioning

The current 1.1.3 update ships the additional scoped MIT permission, contribution inventory and updated NOTICE in registry packages. Public APIs, calculation algorithms, defaults, ABI and schema are unchanged; package and engine release identifiers advance to 1.1.3. The complete native package retains AGPL-3.0-only metadata, with separate Swiss terms. [Licensing guide](license.en.md).

Historical context: the 12 November 2026 documentation refresh explained the already published 1.1.2 implementation without rebuilding or re-uploading those packages. The current response example was regenerated from the unchanged request using the public 1.1.3 package; its coordinates are actual SDK output. The frozen 1.1.2 documentation retains its original example. The 1.1.2 tag, archives, run IDs and checksums remain immutable; 1.1.3 has its own release and verification record.
