# SDK 1.2.0 — Horary question charts

Pass `operation: "horary"`, the time and location of the question, and the
house representing its subject to the existing calculation entry point. The
result contains seven classical bodies, twelve houses, the significators for
the querent and the question, and the Moon's next major aspect before sign
ingress. An absent aspect is reported as a Void-of-Course Moon. The same
result can be rendered as an SVG or used to build a chart model.
[English guide](horary.en.md) · [Hướng dẫn tiếng Việt](horary.md)

The default ruleset is `western-traditional-v1`. It uses Regiomontanus houses,
including when the twelve numbered cusps run in decreasing ecliptic longitude
at polar latitudes. `calculation.houseOrder` records the direction. Compare
the [Ho Chi Minh City request](../examples/horary-request.json) with the
[Longyearbyen request](../examples/horary-polar-request.json).

## Packages

| Registry | Package | Native targets |
| --- | --- | --- |
| [npm](https://www.npmjs.com/package/@7mlabs/astrology/v/1.2.0) | `@7mlabs/astrology@1.2.0` | macOS ARM64 and Linux x64 |
| [PyPI](https://pypi.org/project/sevenmlabs-astrology/1.2.0/) | `sevenmlabs-astrology==1.2.0` | macOS ARM64 and Linux x64 |
| [NuGet](https://www.nuget.org/packages/SevenMLabs.Astrology/1.2.0) | `SevenMLabs.Astrology` 1.2.0 | `osx-arm64` and `linux-x64` |

Linux binaries require glibc 2.38 or later and `libgcc_s.so.1`. See the
[installation instructions](../README.en.md#install) for language runtime
requirements and the [license scope](../LICENSE-SCOPE.md) for the combined
native package.

## Verification

On macOS ARM64, 2,160 Horary charts were compared with direct calls to the
bundled Swiss Ephemeris C library. All 25,920 cusps and 15,120 planetary
house assignments matched, including 605 charts with decreasing longitude.
Both implementations used the same ephemeris. The SDK also produced matching
results across Node.js, Python, .NET, Rust and C for the two example requests.

The public [candidate workflow](https://github.com/7mLabs/sdk-astro/actions/workflows/neutral-engine.yml)
tests the package archives on macOS ARM64 and Linux x64. The
[release workflow](https://github.com/7mLabs/sdk-astro/actions/workflows/npm-release.yml)
publishes the verified npm archive. The verified wheels and NuGet package
are used for their respective registry uploads. Completed run identifiers
and published file hashes are recorded after fresh public installs.
