# Python Linux x64 wheel

SDK 1.1.2 introduced the Python 3.10+ Linux x64 wheel alongside macOS ARM64.
The current **1.1.3** release retains that target matrix and updates the bundled
licensing files: additional MIT permission for identified 7mlabs-owned code,
its explicit scope/inventory and the updated NOTICE. The public combined native
package remains AGPL-3.0-only; Swiss retains its separate terms.

The Python API, astronomical algorithms, JSON schemas, defaults and ABI remain
unchanged from 1.1.2. Package/engine identifiers advance to 1.1.3; envelope schema
remains 1.0. [Licensing guide](license.en.md).

## Supported host

The Linux wheel is tagged `py3-none-manylinux_2_38_x86_64`. It needs an x86-64
process, glibc 2.38 or newer and libgcc_s.so.1. CI installs the final wheel on
Ubuntu 24.04 using CPython 3.10 and 3.13. This is not a Windows, Linux ARM64,
Alpine/musl, Ubuntu 22.04 or Debian 12 wheel. The minimum glibc matches the
existing Linux npm and NuGet distributions; it does not expand their targets.

```sh
python3 -m pip install --only-binary=:all: sevenmlabs-astrology==1.1.3
python3 -c "import sevenmlabs_astrology as a; print(a.__version__)"
```

pip selects the platform wheel. Do not rename or force-install a macOS wheel on
Linux. Check the process architecture and libc before treating a missing wheel
as a calculation error. The wheel contains the native engine, offline timezone
and ephemeris data, SVG assets, license notices and corresponding source.

## Rebuild and audit

Use the matching source checkout or the archive included in the wheel. Build
with Rust, a C compiler, Python, setuptools, wheel, auditwheel and patchelf on
Ubuntu 24.04 x64. Use a clean checkout for each native target.

```sh
cargo build --manifest-path neutral-engine/Cargo.toml -p astro-ffi --lib --release --locked
mkdir -p bindings/python/sevenmlabs_astrology/native/linux-x64
cp neutral-engine/target/release/libastro_engine.so bindings/python/sevenmlabs_astrology/native/linux-x64/
python3 -m pip wheel --no-deps --no-build-isolation --wheel-dir dist bindings/python
python3 -m auditwheel repair --plat manylinux_2_38_x86_64 --only-plat --wheel-dir audited dist/*linux_x86_64.whl
```

The build gate audits ELF dependencies before assigning the manylinux tag.
An incompatible symbol/dependency must fail the build; a filename change is not
compatibility evidence. The release staging step adds the matching source
archive and regenerates RECORD without changing native bytes. The two Python
wheels carry the same corresponding-source archive and platform-specific
native libraries; the final Linux library must match the tested Linux native
asset in the coordinated release.

## Release checks

- Keep the existing macOS ARM64/Python 3.10 and 3.13 consumer jobs.
- Fresh-install the exact final Linux wheel on Python 3.10 and 3.13.
- Compare 339 shared cases, local/IANA time conversion, extended points, SVG,
  JSON models and both Davison variants with final npm and NuGet artifacts.
- Compare unchanged calls with public 1.1.2, 1.1.1, 1.1.0 and 1.0.2; ignore only the
  version marker and the previously documented legacy differences.
- Verify target tags, RECORD, native/source hashes, scoped MIT files and license notices before
  publication. Run the Python README and sample against the installed wheel.

Current checks and final evidence belong to [the 1.1.3 release record](release-1.1.3.md). [The original 1.1.2 report](release-1.1.2.md) retains the historical wheel-introduction results and checksums; those results do not by themselves verify a newly built 1.1.3 wheel.
