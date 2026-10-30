# Coordinated SDK 1.0.2 release

Published on 2026-10-30: npm `@7mlabs/astrology`, PyPI `sevenmlabs-astrology` and NuGet `SevenMLabs.Astrology` all use **1.0.2**. npm `latest` points to 1.0.2.

Source tag [`v1.0.2`](https://github.com/7mLabs/sdk-astro/tree/v1.0.2) points to `83f60ece84dff41b2c79eb473a48cc094428b109`. [Release run 41000539170](https://github.com/7mLabs/sdk-astro/actions/runs/41000539170) rebuilt both native targets and verified all final artifacts before npm publication.

## Changes

- Reject duplicate SVG request/option fields and raw JSON arguments that inject replacement wrapper fields. Valid requests remain compatible.
- Align packaged installation and corresponding-source references with 1.0.2. The source gate rejects stale binding documentation before packaging.
- 1.0.1 reached npm only. 1.0.2 is the coordinated update for all three registries.

## Installation and scope

```sh
npm install --save-exact @7mlabs/astrology@1.0.2
python -m pip install sevenmlabs-astrology==1.0.2
dotnet add package SevenMLabs.Astrology --version 1.0.2
```

| Package | Supported native targets |
| --- | --- |
| npm | macOS ARM64 and Linux x64/glibc 2.38+, Node 18+ |
| PyPI | macOS 11+ ARM64, Python 3.10+ |
| NuGet | .NET 10; osx-arm64 and linux-x64/glibc 2.38+ |

## Verification

133 Rust tests, format and Clippy passed locally. CI passed 339 cross-binding conformance cases on both targets, time/SVG, extended-point, schema, compression and TypeScript checks. Final npm archives passed Node 18/24 on Mac/Linux; the final wheel passed Python 3.10/3.13 on Mac; the final NuGet archive passed Mac/Linux.

Fresh installations from all three public registries on Mac ARM64 passed calculation, civil-time conversion, seven extended points, modular SVG, JSON model, house focus and native checksum checks. The original JSON injection/duplicate cases were rejected in the installed Node, Python and .NET bindings.

| Registry artifact | Verified SHA256 |
| --- | --- |
| npm tarball | `01e59a881d2ecc151c8406b020e7ba692f862b6ef81a2ad7d4a8b4dd4210df3f` |
| PyPI wheel | `9fbca640707dea06031dcea92f01f2f7eec7bb0b7db86bec23b36ede45255f55` |
| NuGet submitted archive | `be2e34b16a5e5eed0de866300f1ac5202d39739d14f6a6d3f083fe9c38664c81` |
| NuGet signed registry archive | `aa2d5489bc3a82751d082fe2700412c74f44e7c47fa5f7d3efb5b44a79ea3b36` |

NuGet repository signing changes ZIP bytes. Every original payload entry except the signature-related content-types declaration, and both native RID binaries, matched the submitted archive. [Machine-readable registry proof](release-1.0.2-verification.json).

Schema/ABI versions and historical release evidence retain their own version identifiers. The [1.0 release contract](release-1.0.md) records the original feature release.
