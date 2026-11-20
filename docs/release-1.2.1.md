# SDK 1.2.1 — Python wheel file inventory

Published on 20 November 2026. [npm](https://www.npmjs.com/package/@7mlabs/astrology/v/1.2.1) · [PyPI](https://pypi.org/project/sevenmlabs-astrology/1.2.1/) · [NuGet](https://www.nuget.org/packages/SevenMLabs.Astrology/1.2.1) · [GitHub Release](https://github.com/7mLabs/sdk-astro/releases/tag/v1.2.1)

Version 1.2.1 is a packaging update. Each Python wheel contains a
`.dist-info/RECORD` file that lists its contents. The 1.2.0 Linux wheel
listed eight directory paths alongside its files. The 1.2.1 packager writes
one row per file and skips directory entries when building the inventory.

The release checks compare the inventory with the final macOS ARM64 and
Linux x64 wheel archives. For every file other than `RECORD`, they check the
SHA-256 digest and byte count. The row for `RECORD` itself has empty hash
and size fields. [Wheel format](https://packaging.python.org/en/latest/specifications/binary-distribution-format/#the-dist-info-directory)

## Using 1.2.1

Choose the package for your application.

| Runtime | Install |
| --- | --- |
| Node.js | `npm install --save-exact @7mlabs/astrology@1.2.1` |
| Python | `python -m pip install sevenmlabs-astrology==1.2.1` |
| .NET 10 | `dotnet add package SevenMLabs.Astrology --version 1.2.1` |

Horary question charts and SVG rendering were introduced in 1.2.0. Applications
using `operation: "horary"` and `renderChartSvg` can use the same calls with
1.2.1. [Hướng dẫn Horary tiếng Việt](horary.md) · [English guide](horary.en.md) · [1.2.0 verification](release-1.2.0.md)

## Public verification

The [candidate workflow](https://github.com/7mLabs/sdk-astro/actions/runs/44656091078) passed 12/12 jobs. The [tag workflow](https://github.com/7mLabs/sdk-astro/actions/runs/44657978436) passed 13/13 jobs and published npm from [`34b9d8f52e80`](https://github.com/7mLabs/sdk-astro/commit/34b9d8f52e8050883455a39bc650b6321a786d33). The [PyPI trusted-publishing workflow](https://github.com/7mLabs/sdk-astro/actions/runs/44659982316) passed 2/2 jobs for the same commit.

Fresh installations from npm, PyPI and NuGet on macOS ARM64 passed 339 raw JSON cases per binding (161 successful calculations and 178 expected errors). Responses matched across all three bindings. Each binding also calculated two Horary charts, one for Ho Chi Minh City and one for Longyearbyen, and rendered both as SVG with house 7 highlighted.

### File inventory in the public PyPI wheels

| Wheel | Regular files | RECORD rows | Directory rows in RECORD | SHA-256 hashes checked | File sizes checked |
| --- | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 188 | 188 | 0 | 187 | 187 |
| Linux x64 | 188 | 188 | 0 | 187 | 187 |

Both wheels contained exactly one row per file. The row for `RECORD` itself had an empty digest and size.

### Public archive SHA-256

| Archive | Final CI SHA-256 | Registry SHA-256 |
| --- | --- | --- |
| `7mlabs-astrology-1.2.1.tgz` | `c0f0206941d0b4e9986848a03eed3018ce11d13837e6ae63d161f825d96e169d` | `c0f0206941d0b4e9986848a03eed3018ce11d13837e6ae63d161f825d96e169d` |
| `sevenmlabs_astrology-1.2.1-py3-none-macosx_11_0_arm64.whl` | `e45d86aa5c4b176af98728e8701ce41d06cfa293c5f9e10c151c964c7cf83443` | `e45d86aa5c4b176af98728e8701ce41d06cfa293c5f9e10c151c964c7cf83443` |
| `sevenmlabs_astrology-1.2.1-py3-none-manylinux_2_38_x86_64.whl` | `d34daf9e4b915a0aa6fedcae9f7a0a4f83e8d14873a23e97424e69c6a9e1ae2a` | `d34daf9e4b915a0aa6fedcae9f7a0a4f83e8d14873a23e97424e69c6a9e1ae2a` |
| `SevenMLabs.Astrology.1.2.1.nupkg` | `3cb30ca6d65abca2aaa0266f170fe4608d037f1455a4671e50afc3f50bf252f1` | `64c4e19d7439a736ee9f3e6e686a922294bdb79094ad0bc9970c54e7192d1bcd` |

The npm tarball and both Python wheels matched the final CI archives byte for byte. The unsigned NuGet package is listed in the CI column. The registry package preserved all 101 original ZIP members and added the NuGet repository signature `.signature.p7s`. GitHub reported matching SHA-256 digests for all six [release assets](https://github.com/7mLabs/sdk-astro/releases/tag/v1.2.1).
