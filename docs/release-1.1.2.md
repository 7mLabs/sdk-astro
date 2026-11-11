# SDK 1.1.2 release checks

Status: implementation prepared; final CI and registry publication are pending.
Do not treat this file as proof that 1.1.2 has been published.

This packaging patch adds the Python Linux x64 wheel with a
`manylinux_2_38_x86_64` compatibility tag. Python needs version 3.10+ and Linux
needs glibc 2.38+ plus libgcc_s.so.1. macOS ARM64 remains available. npm and
NuGet retain their existing platform scope. All package versions are 1.1.2;
calculation algorithms, APIs, schemas and defaults are unchanged.

The final release gate requires real wheel installation on both supported
platforms with Python 3.10 and 3.13, cross-binding conformance and time/SVG/
Davison parity, and comparison with the previous public 1.1.1 package.
Final run IDs, checksums and registry-install results will be added after
those steps finish. See [Linux wheel instructions](python-linux.md).
