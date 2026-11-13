# SDK 1.1.3 — licensing packaging update

**Status: release preparation. Final CI results, registry publication and public-install evidence have not yet been recorded here.** This record must be completed from the actual final artifacts; the 1.1.2 evidence is historical, not proof of a 1.1.3 installation.

## What changes

1.1.3 is the first coordinated registry release to carry `LICENSE-7MLABS-MIT`, `LICENSE-SCOPE.md`, `LICENSE-SCOPE.json` and the updated `NOTICE` with the npm, PyPI and NuGet packages and their corresponding source. These files describe the separate, additional MIT permission for identified original 7mlabs-owned contributions, including only the inventoried original 7mlabs changes to Swiss source.

The complete public native distribution continues to use `AGPL-3.0-only`; retain the full `LICENSE`, Swiss provenance and notices, dependency licenses and font notices. Swiss code/data, unchanged Swiss diff context, other third-party components, branding and trademarks are outside the additional MIT grant. Neither `MIT` nor `AGPL-3.0-only OR MIT` describes the license of the complete bundled native package.

The MIT permission does not supply or expand Swiss Professional rights. The recipient's actual Swiss agreement must cover the intended use, modified source, data and combined distribution; no automatic Professional entitlement is claimed for a prebuilt binary. [Vietnamese guide](license.md) · [English guide](license.en.md) · [Permission scope](../LICENSE-SCOPE.md).

## Compatibility and target matrix

Package and engine release identifiers advance to **1.1.3**. Public APIs, call shapes, calculation algorithms, defaults, ABI and schema remain unchanged from 1.1.2; envelope `schemaVersion` stays `1.0`. Individual rules/policies retain their own versions. The licensing update does not add an API-key requirement or change calculation results.

| Package | Native target matrix | Runtime / host requirements |
| --- | --- | --- |
| npm `@7mlabs/astrology` | macOS ARM64; Linux x64 | Node.js 18+; final consumer matrix Node 18/24 |
| PyPI `sevenmlabs-astrology` | macOS ARM64; `manylinux_2_38_x86_64` | Python 3.10+; final consumer matrix Python 3.10/3.13 |
| NuGet `SevenMLabs.Astrology` | `osx-arm64`; `linux-x64` | .NET 10 / `net10.0` |

Linux requires glibc 2.38+ and `libgcc_s.so.1`; Ubuntu 24.04 is the build/test baseline. macOS native deployment target is 11.0, with separate language-runtime OS requirements. Windows, Intel Mac, Linux ARM64, Alpine/musl and Browser WASM remain outside this matrix. This table describes release targets, not a claim that final consumers have passed before their reports exist.

The repository pins the source-build .NET SDK to **10.0.404** in `global.json`
with exact SDK selection. CI logs and verifies the selected SDK before building;
a newer SDK preinstalled on a hosted runner must not change the compiler. The
cross-platform managed-assembly API, IL and normalized-PE checks remain enabled.
Consumers of the NuGet package retain the .NET 10 runtime requirement above.

## Evidence to finalize

| Evidence | Current record |
| --- | --- |
| Tagged source commit and release date | Pending final release |
| Candidate CI run and results | Pending final CI |
| Tagged publication run and results | Pending publication |
| npm final archive SHA-256 and registry identity | Pending final artifact / registry check |
| PyPI macOS ARM64 and Linux x64 wheel SHA-256 | Pending final artifacts / registry check |
| NuGet final archive SHA-256 and repository-signature comparison | Pending final artifact / registry check |
| Fresh installations from npm, PyPI and NuGet | Pending publication and consumer checks |
| Website release evidence / checksums | Pending final artifact verification |

Final validation must establish exact grant/scope/NOTICE contents in each archive and corresponding-source snapshot, consistent source identities and metadata, supported-target final consumers, and compatibility with published 1.1.2 plus the existing legacy baselines. Normalize only declared release-version fields when comparing unchanged payloads. Run/hash/test counts must come from the actual reports; none are inferred here.

## Historical releases remain intact

Already published 1.1.2 registry artifacts, tags, source snapshots and checksums are not replaced or repackaged. [The original 1.1.2 report](release-1.1.2.md) retains its own source commit, CI results and public artifact digests. Its sample payloads continue to identify their actual engine version.

The source-level MIT permission was separately granted for the covered revisions. Shipping the files in 1.1.3 makes that scope available in the installation packages; it neither revokes permissions for previously covered revisions nor grants MIT rights to Swiss or other excluded material in old or new archives.
