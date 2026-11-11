# SDK 1.1.2 release verification

Published 2026-11-11 from tag [v1.1.2](https://github.com/7mLabs/sdk-astro/tree/v1.1.2), source commit `02da06b07a43cda79a219f3e1562f67b93239689`.

This packaging patch adds the Python Linux x64 wheel, alongside macOS ARM64. Python needs version 3.10+; Linux requires **glibc 2.38+ and libgcc_s.so.1** (`manylinux_2_38_x86_64`). Ubuntu 24.04 was tested with Python 3.10 and 3.13. Ubuntu 22.04, Debian 12 and Alpine/musl are outside this wheel's scope. See [Linux wheel instructions](python-linux.md).

Calculation crates, public function signatures, request schemas, defaults and ABI are unchanged from 1.1.1. The Python wrapper changes only `__version__`; engine/package versions advance to 1.1.2 and envelope `schemaVersion` stays 1.0. npm and NuGet keep their existing platform coverage. Davison was introduced in 1.1.0; optional calculation metadata in 1.1.1.

## Evidence

- [Candidate CI 43044915529](https://github.com/7mLabs/sdk-astro/actions/runs/43044915529): all 12 jobs passed.
- [Tag publication 43048604597](https://github.com/7mLabs/sdk-astro/actions/runs/43048604597): 12 candidate checks and npm publication passed. A macOS Node 18 job initially received no hosted runner; rerunning the interrupted job succeeded without changing the tag or artifacts.
- Final Python/.NET consumer checks: macOS ARM64 and Linux x64, each with Python 3.10 and 3.13. Each report covers 339 conformance cases, 9 time cases, 2 extended-point cases, 12 SVG cases, 2 model cases and 4 Davison combinations across Node, Python and .NET.
- Final npm consumer checks: Node 18 and 24 on both supported OS/CPU targets, including comparison against public 1.1.1, 1.1.0 and 1.0.2 baselines. Existing calls remain supported.
- The Linux wheel passes auditwheel and native/metadata/RECORD/source checks. Both Python wheels carry identical corresponding source archives; final native checksums match the verified candidates.
- Fresh installations from all three public registries on a real macOS ARM64 host compare natal/local-time data, all seven extended points, SVG drawing and semantic metadata, chart models, house focus and four Davison variant/house-system combinations. pip also resolves the new Linux wheel for a CPython 3.10 target and its checksum matches CI. Actual Linux execution is covered separately by the Ubuntu CI jobs.

These checks describe tested cases and supported targets; they do not imply support for untested operating systems or every possible input.

## Public artifact SHA-256

| Package | SHA-256 |
| --- | --- |
| [npm @7mlabs/astrology](https://www.npmjs.com/package/@7mlabs/astrology/v/1.1.2) | `5737ba31af7e3df2bff577d806ef58e521f7ed34fce7616069b0f2e71e3e3014` |
| [PyPI macOS ARM64](https://pypi.org/project/sevenmlabs-astrology/1.1.2/) | `82910b18aff262ad04426f3e911725821aaca62213ed97151f81516846d1d210` |
| [PyPI Linux x64](https://pypi.org/project/sevenmlabs-astrology/1.1.2/) | `b08ab76c66c69f965431c54d4bea508a764b046abbe8e12cfcdf946a8efbad52` |
| [NuGet SevenMLabs.Astrology](https://www.nuget.org/packages/SevenMLabs.Astrology/1.1.2) | `6b6ce8dfd8d58ec522e8d0fd3ae4c47f0bd8297da48b89c274a3c9be34f0f61a` |

npm and both PyPI wheel digests match the CI-verified files exactly. NuGet adds a repository signature; every original ZIP member matches the CI package, with only `.signature.p7s` added.

[Public release evidence](https://7mlabs.com/astro/sdk/docs/1.1.2/downloads/verification.json) · [Vietnamese guides](https://7mlabs.com/astro/sdk/docs/1.1.2/overview/) · [English guides](https://7mlabs.com/en/astro/sdk/docs/1.1.2/overview/)
