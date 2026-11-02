# SDK 1.1.0 release candidate

This candidate adds standalone Davison corrected/uncorrected through the existing
calculation API. Publication will use the same tested source commit and artifacts
for npm `@7mlabs/astrology`, PyPI `sevenmlabs-astrology` and NuGet
`SevenMLabs.Astrology`. Registry upload status is recorded after publication.

## Compatibility

Existing natal, natalDomains, couple, composite, forecast, events and query request
shapes remain supported. Existing 1.0.x TypeScript `Request` and its calculate
result union remain unchanged; the additive `CalculationRequest` union includes
Davison. No extra runtime dependency or new ABI export is required.

One numerical boundary correction normalizes a signed angular difference rounded
to +180 degrees into the documented [-180, 180) interval. Exact JSON snapshots at
that boundary may change to -180; this is the same opposite direction. Compatibility
evidence distinguishes this correction from other output differences.

## Verification and distribution scope

- Independent comparison: [64 charts and 3,648 checks](reviews/2026-11-02-davison/README.md).
- Native targets: macOS ARM64; Linux x64/glibc 2.38+ for npm and NuGet.
- npm: Node 18+; final archives tested on Node 18 and 24 on both targets.
- PyPI: macOS 11+ ARM64, Python 3.10+; final wheel tested with 3.10 and 3.13.
- NuGet: .NET 10; osx-arm64 and linux-x64.

Release gates include locked Rust tests/fmt/Clippy, unchanged 1.0.2 inputs,
legacy TypeScript consumers, fresh installation of final archives, cross-binding
Davison calculation/local-time/SVG/model checks, schemas, compression, artifact
identity and public registry verification. Passing evidence must refer to the
exact source/artifact hashes; CI and public installation results are recorded
when complete.
