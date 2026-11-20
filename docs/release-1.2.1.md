# SDK 1.2.1 — Python wheel file inventory

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
