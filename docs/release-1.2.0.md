# SDK 1.2.0 — Horary question charts

Published to npm, PyPI and NuGet on 19 November 2026. The [GitHub release](https://github.com/7mLabs/sdk-astro/releases/tag/v1.2.0) includes the four tested archives, their manifest and SHA-256 checksums.

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

## Comparison with Swiss Ephemeris

On macOS ARM64, 2,160 Horary charts were compared with direct calls to the
bundled Swiss Ephemeris C library. All 25,920 cusps and 15,120 planetary
house assignments matched, including 605 charts with decreasing longitude.
Both implementations used the same ephemeris. The SDK also produced matching
results across Node.js, Python, .NET, Rust and C for the two example requests.

## Public verification

The [release tag](https://github.com/7mLabs/sdk-astro/tree/v1.2.0) points to [`517177c5a60a`](https://github.com/7mLabs/sdk-astro/commit/517177c5a60ac74aab75d89c54a4b39e67463e47). The [candidate workflow](https://github.com/7mLabs/sdk-astro/actions/runs/44556232312) passed 12/12 jobs. The [tag workflow](https://github.com/7mLabs/sdk-astro/actions/runs/44559746806) passed 13/13 jobs and published npm. The [PyPI trusted-publishing workflow](https://github.com/7mLabs/sdk-astro/actions/runs/44563906036) passed 2/2 jobs for the tagged commit.

Fresh installations from npm, PyPI and NuGet on macOS ARM64 passed 339 raw JSON cases per binding, for 1,017 calls in total. Each binding returned 161 successful calculations and 178 expected errors. All responses matched across bindings. Each install also calculated and rendered the Ho Chi Minh City and Longyearbyen Horary charts, including the seven classical bodies, twelve houses and highlighted seventh house.

On both hosted platforms the compatibility check compared 339 existing requests with the published npm SDK 1.1.3. Every result matched after excluding the updated `engineVersion` field. Applications can opt into Horary through the existing calculation and rendering APIs.

### Hosted tests of the final archives

| Consumer | Target | Runtime | Checks passed |
| --- | --- | --- | --- |
| npm | `darwin-arm64` | Node v18.20.8 | 339 conformance, Horary and TypeScript |
| npm | `darwin-arm64` | Node v24.22.2 | 339 conformance, Horary and TypeScript |
| npm | `linux-x64` | Node v18.20.8 | 339 conformance, Horary and TypeScript |
| npm | `linux-x64` | Node v24.22.2 | 339 conformance, Horary and TypeScript |
| Python, .NET, Node | `linux-x64` | Python 3.10.22, .NET SDK 10.0.404, Node v24.22.2 | 339 conformance, 9 time, 2 extended, 12 svg, 2 model, 4 davison, 4 horary |
| Python, .NET, Node | `linux-x64` | Python 3.13.18, .NET SDK 10.0.404, Node v24.22.3 | 339 conformance, 9 time, 2 extended, 12 svg, 2 model, 4 davison, 4 horary |
| Python, .NET, Node | `osx-arm64` | Python 3.10.11, .NET SDK 10.0.404, Node v24.22.2 | 339 conformance, 9 time, 2 extended, 12 svg, 2 model, 4 davison, 4 horary |
| Python, .NET, Node | `osx-arm64` | Python 3.13.17, .NET SDK 10.0.404, Node v24.22.3 | 339 conformance, 9 time, 2 extended, 12 svg, 2 model, 4 davison, 4 horary |

### Public archive SHA-256

| Archive | Final CI SHA-256 | Registry SHA-256 |
| --- | --- | --- |
| `sevenmlabs_astrology-1.2.0-py3-none-macosx_11_0_arm64.whl` | `1ecf00e0d7f4d2be3347790a0953077a0a707fd5e895f0dcd98e3b7e67df8248` | `1ecf00e0d7f4d2be3347790a0953077a0a707fd5e895f0dcd98e3b7e67df8248` |
| `sevenmlabs_astrology-1.2.0-py3-none-manylinux_2_38_x86_64.whl` | `037addec25ea56b758bd650c2d335b720cbe5ef5e1d9530739060164d1b0fae1` | `037addec25ea56b758bd650c2d335b720cbe5ef5e1d9530739060164d1b0fae1` |
| `SevenMLabs.Astrology.1.2.0.nupkg` | `ba952b00a782f3cc41a1044a119ec211e7df203aebe3a238c368d0887018b278` | `4e30010401861483044e0218c43bf1cbcb91bc974eea0f5a1c91b88cc591cd40` |
| `7mlabs-astrology-1.2.0.tgz` | `898c78c7b4d49af19db377d9459f8f456ec542ba4711cf5b57bd063524ab9ad5` | `898c78c7b4d49af19db377d9459f8f456ec542ba4711cf5b57bd063524ab9ad5` |

The npm tarball and both Python wheels matched CI byte for byte. NuGet preserved all 101 original ZIP members and added the repository signature `.signature.p7s`. The SHA-256 digests of all six GitHub release assets matched their local copies.
