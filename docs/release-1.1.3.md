# SDK 1.1.3 — verified licensing packaging release

Published **2026-11-13** from immutable tag [v1.1.3](https://github.com/7mLabs/sdk-astro/tree/v1.1.3), source commit `b769bdf38a7f8dd500b902661d28d97ebf8cddbc`. The npm, PyPI and NuGet packages were installed from their public registries and checked against the exact final CI artifacts before this record was completed.

## What changed

1.1.3 is the first coordinated registry release to carry `LICENSE-7MLABS-MIT`, `LICENSE-SCOPE.md`, `LICENSE-SCOPE.json` and the updated `NOTICE` with the three packages and their corresponding source. They identify separate additional MIT permission for original 7mlabs-owned contributions, including only the inventoried original 7mlabs changes to Swiss source.

The complete public native distribution remains **`AGPL-3.0-only`**. Keep the full AGPL, Swiss provenance/notices, dependency licenses and font notices. Swiss code/data, unchanged Swiss diff context, third-party material, branding and trademarks are outside the additional MIT grant. Neither `MIT` nor `AGPL-3.0-only OR MIT` describes the complete bundled native package.

The additional permission does not supply or expand Swiss Professional rights. The recipient's actual agreement must cover the intended use, modified source, data and combined distribution. No automatic Professional entitlement for prebuilt binaries is claimed. [Vietnamese guide](license.md) · [English guide](license.en.md) · [Exact permission scope](../LICENSE-SCOPE.md).

## Compatibility and supported targets

Package and `engineVersion` advance to **1.1.3**; envelope `schemaVersion` stays **1.0**. Public API names and call shapes, formulas, defaults, ABI and schemas remain unchanged from 1.1.2. There is no Professional-key entry or runtime entitlement check added to the SDK.

- npm: macOS ARM64 and Linux x64; declared Node.js minimum 18, with actual Node 18/24 final-consumer results below.
- PyPI: macOS ARM64 and `manylinux_2_38_x86_64`; Python 3.10+, with actual Python 3.10/3.13 results below.
- NuGet: `osx-arm64` and `linux-x64`, targeting `net10.0`.

Linux needs **glibc 2.38+ and `libgcc_s.so.1`**; Ubuntu 24.04 is the Linux build/test baseline. Native macOS has deployment target 11.0, while the chosen language runtime retains its own OS requirements. Windows, Intel Mac, Linux ARM64, Alpine/musl and Browser WASM remain outside this matrix. No new calculation operation or platform is introduced by this patch.

The source build pins .NET SDK **10.0.404** in `global.json`; actual final managed reports below confirm the selected SDK. This is a build-compiler pin, not a new runtime requirement for NuGet consumers. The managed API, IL and normalized-PE comparison checks stay enabled.

## Completed evidence

- [Candidate CI](https://github.com/7mLabs/sdk-astro/actions/runs/43347289393): **12/12 checks passed**.
- [Release-tag publication](https://github.com/7mLabs/sdk-astro/actions/runs/43349393594): **13/13 checks passed**, including the npm publication job.
- All public archives retain `AGPL-3.0-only` metadata; grant/scope/NOTICE contents match the release source byte-for-byte. Both Python source archives retain matching licensing files and the inventoried Swiss modifications.
- npm and both PyPI wheels match final CI SHA-256 exactly. Every original NuGet ZIP member matches the CI archive; the only permitted additional member is the repository signature `.signature.p7s`.
- Fresh public npm, PyPI and NuGet installations passed **339 raw-JSON conformance cases per binding**, including expected error envelopes, plus **9 time, 2 extended-point, 12 SVG, 2 model, 4 Davison, 5 object-API and 2 invalid-render-options checks**. The public consumer host was `darwin-arm64`. Raw request strings retain duplicate/invalid JSON cases instead of normalizing them through object helpers.
- A separately installed public npm **1.1.2** baseline matched **354 raw responses** from 1.1.3, removing **only `engineVersion`**. This covers all 339 conformance inputs plus time, extended-point and Davison cases. Existing application API calls do not need changes for this licensing packaging release.
- Object-wrapper payload/model data match across all three bindings. SVG drawing is identical; embedded JSON metadata is compared semantically to account for equivalent JSON number spellings. Renderer-source version metadata is not confused with a calculation change.
- pip resolved the Linux wheel for a CPython 3.10 target and matched its SHA-256; actual Linux native execution is recorded separately in the final CI matrix below.

### Actual final CI consumer matrix

The rows below are derived from passing reports for source commit `b769bdf38a7f8dd500b902661d28d97ebf8cddbc`, rather than inferred from configured matrix names.

| Final consumer | Native target | Actual selected runtime | Result |
| --- | --- | --- | --- |
| npm | `darwin-arm64` | Node `v18.20.8` | 339 conformance; TypeScript passed |
| npm | `darwin-arm64` | Node `v24.22.2` | 339 conformance; TypeScript passed |
| npm | `linux-x64` | Node `v18.20.8` | 339 conformance; TypeScript passed |
| npm | `linux-x64` | Node `v24.22.2` | 339 conformance; TypeScript passed |
| Python / .NET / Node | `linux-x64` | Python `3.10.22`; .NET SDK `10.0.404` | 339 conformance; 9 time; 2 extended; 12 SVG; 2 model; 4 Davison |
| Python / .NET / Node | `linux-x64` | Python `3.13.18`; .NET SDK `10.0.404` | 339 conformance; 9 time; 2 extended; 12 SVG; 2 model; 4 Davison |
| Python / .NET / Node | `osx-arm64` | Python `3.10.11`; .NET SDK `10.0.404` | 339 conformance; 9 time; 2 extended; 12 SVG; 2 model; 4 Davison |
| Python / .NET / Node | `osx-arm64` | Python `3.13.17`; .NET SDK `10.0.404` | 339 conformance; 9 time; 2 extended; 12 SVG; 2 model; 4 Davison |

These checks identify tested cases, runtimes and targets; they do not certify every possible input or an unsupported operating system.

## Public artifact SHA-256

| Package | Final CI/uploaded SHA-256 | Public registry SHA-256 |
| --- | --- | --- |
| [7mlabs-astrology-1.1.3.tgz](https://www.npmjs.com/package/@7mlabs/astrology/v/1.1.3) | `a004acb4f0df348c3afc7a61f7e214e1edf359ab3430b188dc017ebf87e04792` | `a004acb4f0df348c3afc7a61f7e214e1edf359ab3430b188dc017ebf87e04792` |
| [sevenmlabs_astrology-1.1.3-py3-none-macosx_11_0_arm64.whl](https://pypi.org/project/sevenmlabs-astrology/1.1.3/) | `7386c02e51db553690dd1b86f8586e6e60246167a787ea6a821374fc7e77585b` | `7386c02e51db553690dd1b86f8586e6e60246167a787ea6a821374fc7e77585b` |
| [sevenmlabs_astrology-1.1.3-py3-none-manylinux_2_38_x86_64.whl](https://pypi.org/project/sevenmlabs-astrology/1.1.3/) | `47d472963f9e7e76942a943de73fc966728deda0f8e9f65fde153e8cfa719401` | `47d472963f9e7e76942a943de73fc966728deda0f8e9f65fde153e8cfa719401` |
| [SevenMLabs.Astrology.1.1.3.nupkg](https://www.nuget.org/packages/SevenMLabs.Astrology/1.1.3) | `4796bb130c579fac1cc67b4c4d81169ac8ca20b808945e262fa18bbbb5f27d5f` | `68676c4010a0114ffb064fb22b143cf1f30087528a7421954ebbe14139e963b5` |

NuGet's public archive SHA-256 differs when its repository signature is added. The exact original-member comparison is the release identity check; it is not replaced by merely matching a version string.

## Historical releases remain intact

Published 1.1.2 artifacts, tags, source snapshots and checksums are not overwritten or repackaged. [The original 1.1.2 report](release-1.1.2.md) retains its own evidence and source identity. Older API calls remain supported; a prior release's test report is not used as a substitute for the completed 1.1.3 checks above.

The MIT permission was separately granted for covered source revisions. Shipping its files in 1.1.3 neither revokes previously granted rights nor licenses excluded Swiss or third-party material under MIT. This documentation update happens after the immutable release tag; it does not modify that tag or any registry archive.

[Public verification record](https://7mlabs.com/astro/sdk/docs/1.1.3/downloads/verification.json) · [Vietnamese guides](https://7mlabs.com/astro/sdk/docs/1.1.3/overview/) · [English guides](https://7mlabs.com/en/astro/sdk/docs/1.1.3/overview/)
