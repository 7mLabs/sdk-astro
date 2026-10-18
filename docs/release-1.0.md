# Coordinated SDK 1.0.0 release

The engine, npm `@7mlabs/astrology`, Python `sevenmlabs-astrology` and NuGet
`SevenMLabs.Astrology` use version **1.0.0** for this release. All artifacts come
from the same source commit. A version is online only after registry verification;
a source commit or successful local build does not imply publication.

## Public API and compatibility

The public surface is the documented calculation/service JSON contract, exported
wrapper functions and TypeScript declarations. JSON envelopes retain schemaVersion
`1.0`; schema version and package release version have different purposes.

After 1.0, backward-compatible additions increase MINOR; compatible fixes increase
PATCH; incompatible public API changes increase MAJOR. Callers should branch on
error codes, not prose error messages. Human-readable validation messages may list
new accepted fields. SVG presentation and renderer version are recorded separately;
pixel/byte-identical drawings across future releases are not promised.

Default UTC calls and the ten-body calculation retain their prior behavior. Time
conversion and extended points are explicit choices. Changing presentation reuses
existing source coordinates and does not invoke astronomy. Physical coordinate
accuracy is a separate question from binding parity and deterministic rendering.

## Release scope

| Registry | Artifact scope |
| --- | --- |
| npm | Node-API addon, darwin-arm64 and linux-x64, minimum Node 18; verified on Node 18/24 |
| PyPI | Python 3.10+, macOS ARM64 wheel; verified on Python 3.10/3.13 |
| NuGet | net10.0 assembly, osx-arm64 and linux-x64 native RID assets; verified on .NET 10 |

Linux native libraries require glibc 2.38+ and libgcc_s.so.1. The macOS native
deployment target is 11.0; each Node/.NET/Python runtime can impose a newer host OS.
Windows, Intel Mac, Linux ARM, musl and browser WASM are outside this release.
The Linux Python candidate is not a portable PyPI wheel and is not published.

CI pins Rust 1.99.1 for release builds and lint checks; a floating toolchain must
not change the release compiler midway through verification.

## Gates before publication

1. Rust format, Clippy, tests, pinned-data point reference checks and 339 decoded
   JSON comparisons with public npm 0.10.1. Only engineVersion and the two reviewed
   validation-message expansions for new options are normalized in that comparison.
2. Build native packages on Mac ARM64 and Linux x64 from the same commit. Run 339
   five-language conformance cases, concurrency, time/SVG, schema, compression and
   TypeScript checks on each candidate target.
3. Assemble a single final npm tarball and a single NuGet package with both native
   targets. Portable wrappers/notices must agree byte-for-byte. Managed assemblies must agree
   in every PE byte except MVID, timestamps, checksum and debug identities; API and
   IL hashes must also agree. The original Mac assembly is retained unchanged and
   that exact DLL is tested on both operating systems. Python
   preserves the tested macOS binary and includes matching source/build instructions.
4. Reinstall the final npm tarball on Mac/Linux, Node 18/24. Reinstall final managed
   artifacts on Mac Python 3.10/3.13 and Linux .NET 10. Compare all 339 cases plus
   DST, extended points, SVG and model-only outputs against the final Node package.
5. Require the complete evidence matrix to match artifact hashes and commit before
   publishing any package. Registry jobs cannot make all three registries atomic;
   a partial upload is recorded and only the missing registry is retried.
6. Verify registry identity, version, archive/native hashes and fresh installation.
   NuGet repository signing may change ZIP bytes; compare original package payload
   entries and record the signed registry archive hash separately.

The candidate workflow has read-only repository permissions and never publishes.
The existing npm trusted publisher remains restricted to npm-release.yml and its
release environment. First PyPI/NuGet uploads use the authenticated owner account;
future trusted publishers can be configured independently after ownership is proven.
