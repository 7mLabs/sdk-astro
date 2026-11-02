# Coordinated SDK 1.1.0 release

Published on **2026-11-02**: [npm @7mlabs/astrology](https://www.npmjs.com/package/@7mlabs/astrology/v/1.1.0), [PyPI sevenmlabs-astrology](https://pypi.org/project/sevenmlabs-astrology/1.1.0/) and [NuGet SevenMLabs.Astrology](https://www.nuget.org/packages/SevenMLabs.Astrology/1.1.0). npm latest is 1.1.0.

## Changes and compatibility

Davison corrected/uncorrected is available through the existing calculation API;
see [method, options and provenance](davison.md). Existing natal, natalDomains,
couple, composite, forecast, events and query request shapes are unchanged.
The published 1.0.x `Request`/`LocalCalculationRequest` TypeScript unions,
calculate overloads and utility-type contracts are preserved. Additive
`CalculationRequest`/`AllLocalCalculationRequest` unions support callers selecting
Davison dynamically. No new runtime dependency or ABI export is required.

One numerical correction maps a signed angular difference rounded to +180 into
the documented [-180, 180) interval. Exact snapshots at that boundary may change
to -180 (the same opposite direction). Upgrade comparisons accept only that
explicit correction and engineVersion changes; other legacy output changes fail.

## Validation

- 139 Rust tests, fmt and Clippy passed. Final CI passed all 11 build/package/runtime jobs on macOS ARM64 and Linux x64.
- 339 unchanged public 1.0.2 requests, existing helper/time/SVG/model calls and compression roundtrips passed upgrade checks. Old TypeScript consumers, including Request/ReturnType/Parameters usage, compile unchanged.
- Final npm archives passed Node 18/24 on both targets. The final Python wheel passed Python 3.10/3.13 on Mac. Final NuGet passed Mac/Linux; all installed bindings passed four Davison variant/house-system combinations with UTC, local-time, SVG and model parity.
- The actual final Mac native artifact passed 64 charts / 3,648 comparisons against separate Swiss assembly and independent Skyfield/JPL position/angle calculations. [Protocol and limits](reviews/2026-11-02-davison/README.md).
- Fresh installations from all three public registries passed old calculation/time, seven extended points, SVG, chart model, house focus, injection regression, and four Davison combinations. Public archive/native hashes match the tested artifacts; NuGet repository-signature differences were accounted for.

[Machine-readable release evidence](release-1.1.0-verification.json).

## Immutable source and publication

Source tag [v1.1.0](https://github.com/7mLabs/sdk-astro/tree/v1.1.0) points to
`687cb7db0375b426f7392a812d3c1f061d16904a`. [Candidate run 41375190775](https://github.com/7mLabs/sdk-astro/actions/runs/41375190775) passed every build/test job but stopped before npm upload at an outdated final report dictionary. The updated publication controller requires all old counts plus four Davison checks, validates the original source/run and all artifact/report hashes, and [published the identical tested tarball in run 41377062414](https://github.com/7mLabs/sdk-astro/actions/runs/41377062414). The source tag and release archives were not changed or rebuilt.

## Install and scope

```sh
npm install --save-exact @7mlabs/astrology@1.1.0
python -m pip install sevenmlabs-astrology==1.1.0
dotnet add package SevenMLabs.Astrology --version 1.1.0
```

| Package | Supported targets |
|---|---|
| npm | Node 18+; macOS ARM64, Linux x64/glibc 2.38+ |
| PyPI | Python 3.10+; macOS 11+ ARM64 |
| NuGet | .NET 10; osx-arm64, linux-x64/glibc 2.38+ |

## Archive SHA256

- `sevenmlabs_astrology-1.1.0-py3-none-macosx_11_0_arm64.whl`: `f244cba04818bb43ae6b49048dcac26e5f82fd7029e28efa513aac11eda7d279`
- `SevenMLabs.Astrology.1.1.0.nupkg`: `5d30e6ba48853e441fcf6076f8263141c8e9bec460a4d17729ae0834c6933ee8`
- `7mlabs-astrology-1.1.0.tgz`: `c78900822ffc26320608ffb4a5af03af81f898d14534564219bae9bd6f8acc7e`
