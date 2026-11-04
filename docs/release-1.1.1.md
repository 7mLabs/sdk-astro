# SDK 1.1.1 release verification

Published 2026-11-04 from tag [v1.1.1](https://github.com/7mLabs/sdk-astro/tree/v1.1.1), source commit `9216ebd9d8dbe17518d0afd09a2180e8d68f408c`.

This patch adds optional calculation metadata. Astronomical/query requests use `includeMetadata: true`; unchanged requests retain the frozen 1.1.0 response contracts. Caller-supplied geometry context is explicitly unverified. See [metadata contract](payload-metadata.md). Davison was introduced in 1.1.0.

## Evidence

- Candidate CI [41770379884](https://github.com/7mLabs/sdk-astro/actions/runs/41770379884): all 11 jobs passed, including final installed npm/Python/.NET consumers.
- Tag publication [41772027554](https://github.com/7mLabs/sdk-astro/actions/runs/41772027554): all candidate jobs and publication passed.
- Numeric comparison: 339 cases against 1.1.0 and 339 against 1.0.2 passed. The pre-existing signed-delta endpoint correction in 1.1.0 is accounted for in the 1.0.2 comparison.
- Frozen 1.1.0 schemas validate default outputs; enriched schemas validate opt-in results. Eight query methods and six astronomical response families are covered, including local-time facades.
- Real HTTP Payload integration: 70 tests passed using public npm and signed NuGet 1.1.1 packages.

These checks describe the tested inputs and supported targets; they are not a claim that every possible input has been independently verified.

## Public artifacts

| Package | Public version | SHA-256 |
| --- | --- | --- |
| [npm @7mlabs/astrology](https://www.npmjs.com/package/@7mlabs/astrology/v/1.1.1) | 1.1.1 | `93637aef90aae8c808c25699705376dacbe46cbe88f4e14ce00562aa0643da12` |
| [PyPI sevenmlabs-astrology](https://pypi.org/project/sevenmlabs-astrology/1.1.1/) | 1.1.1 | `9fb9cde3411801a218c8dd7e719f828e9b717367ade776ee29955197dce3b2e9` |
| [NuGet SevenMLabs.Astrology](https://www.nuget.org/packages/SevenMLabs.Astrology/1.1.1) | 1.1.1 | `981d0ec3e099af1afb6ab313b39b04c5a2ada14977ce0ff38e68d1b50ed59fe1` |

npm and PyPI digests match the CI-verified files. NuGet adds a repository signature; every original ZIP member matches the CI package and the only additional member is `.signature.p7s`.

Platform scope is unchanged: npm macOS ARM64/Linux x64, Python macOS ARM64 (Python 3.10+), .NET 10 macOS ARM64/Linux x64. Linux requires glibc 2.38+. No additional platform support is implied by this patch.
