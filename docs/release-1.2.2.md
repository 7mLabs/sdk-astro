# 1.2.2 — Lilly Horary conventions

Select `"ruleset": "lilly-1647-v1"` in a Horary request. The same selector works with local civil time. Omission and `western-traditional-v1` retain the 1.2.1 computation. Both profiles return chart data for interpretation by the caller.

* **Houses.** The Lilly profile uses Regiomontanus. An explicit incompatible house system returns `INVALID_INPUT`. Each placement retains its geometric house. Its `nextCuspWithinFiveDegrees` gives the following house and directed longitude distance when the cusp is within five degrees, or null otherwise. The SDK leaves the significance of that proximity to the caller.
* **Aspects.** For the five major angles, the maximum separation from exact is half the sum of the two planetary orbs. The profile selects the first column on p. 107: Sun 17°, Moon 12.5°, Mercury 7°, Venus 8°, Mars 7.5°, Jupiter 12°, Saturn 10°. Lilly also prints alternatives. The inclusive limit for each pair appears in the aspect’s `maxOrb`. The angle list in `calculation.aspectRules` reports the outer search limit. The actual table appears in `calculation.orbPolicy`. Aspects use ecliptic longitude and can cross a sign boundary. [William Lilly, *Christian Astrology*, pp. 106–108 and 154](https://lib.wildla.in/books/Christian%20Astrology%201-2%20by%20William%20Lilly.pdf)
* **Moon.** `inOrbLunarContinuity` implements a specified reading of Lilly’s examples. Continuity requires a separating major contact still within the pair’s orb and another contact already applying within orb. The earliest such application may complete after ingress. A contact at the question instant is excluded from future events (one millisecond time tolerance). When continuity holds, `nextAspect` gives the first eligible perfection. When only an application exists, `isVoidOfCourse` is true, `nextAspect` is null and `applicationWithoutSeparation` preserves the upcoming contact. `hasSeparatingContact` exposes the other half of the test. [Deborah Houlding, analysis of Lilly’s charts](https://www.skyscript.co.uk/moon2.html)

## Example

For Ho Chi Minh City (10.8231° N, 106.6297° E) on 20 November 2026 at 10:58:40 ICT, the legacy Moon indicator is void before ingress. Under the Lilly profile the Moon is separating from a trine to the Sun and applying to a conjunction with Saturn. The conjunction completes after the Moon changes sign. Both results can be reproduced from [the request](../examples/horary-lilly-request.json) by selecting the corresponding ruleset.

This release covers the indicators listed here. Reception, a full dignity table and question-specific judgments are outside this profile’s current output.

## Public verification on 21 November 2026

Version 1.2.2 is available on [npm](https://www.npmjs.com/package/@7mlabs/astrology/v/1.2.2), [PyPI](https://pypi.org/project/sevenmlabs-astrology/1.2.2/) and [NuGet](https://www.nuget.org/packages/SevenMLabs.Astrology/1.2.2). The [GitHub release](https://github.com/7mLabs/sdk-astro/releases/tag/v1.2.2) includes the four archives, the manifest and SHA-256 checksums.

The [candidate](https://github.com/7mLabs/sdk-astro/actions/runs/44864467221) passed 12 jobs. The [tag run](https://github.com/7mLabs/sdk-astro/actions/runs/44866576950) passed 13 jobs, and the [PyPI workflow](https://github.com/7mLabs/sdk-astro/actions/runs/44867738624) verified the two published wheel hashes against the same commit, `88e88aaabb24873ba5b8b374906b17e48d794a9c`.

Fresh installations from the three registries on macOS ARM64 matched for all 339 raw requests per binding (161 successes and 178 expected errors). Each rendered matching SVG for the legacy Ho Chi Minh City and polar examples and for the new Lilly example. Both public wheels passed file inventory, size and hash checks. All 101 original NuGet members matched the tested archive after NuGet added its repository signature.

A separate comparison used 336 chart moments across four months in 1896, 2000, 2026 and 2398. All planetary aspect selections and all four combinations of lunar application and separation agreed with an independent comparison using the positions and event search of 1.2.1. Of 280 eligible lunar contacts, 27 completed after ingress. The largest difference between the two event solvers was under 0.0025 seconds. Both solvers use the same ephemeris. These checks measure reproducibility of the specified computation.
