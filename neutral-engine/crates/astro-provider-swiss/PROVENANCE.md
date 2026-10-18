# Swiss Ephemeris provider

C source and headers from https://github.com/aloistr/swisseph,
commit `aacf962d19d79f8bc921dbcdaacf306d85be1917`, provider version 2.10.03.
Source fetched 2026-10-02. `vendor/LICENSE` and `vendor/LICENSE.TXT` retain
upstream licensing terms (AGPL or commercial Professional License).
Local modification by 7mlabs on 2026-10-02: `sweph.c:swi_fopen` returns NULL when the build flag
`ASTRO_MOSHIER_ONLY` is set. This disables external ephemeris and time-table
files, so the selected Moshier model and compiled leap-second/Delta T tables
are reproducible regardless of working directory or SE_EPHE_PATH.
No ephemeris data files are required by the selected Moshier mode.
Local integration does not settle licensing of the entire derived distribution.
Public release requires the license decision described in docs/distribution.md.

## Optional additional natal points (2026-10-18)

`data/seas_18.se1` is from the same upstream commit, fetched from
https://raw.githubusercontent.com/aloistr/swisseph/aacf962d19d79f8bc921dbcdaacf306d85be1917/ephe/seas_18.se1
on 2026-10-18. SHA-256:
`a2cd8fc33807c78ca9a700c91c2e042258b12fc4796519e00781440b5ad8b2e2`.
It is embedded in the native binary and opened from memory on macOS/Linux only
when additional points are requested. Swiss uses its Chiron asteroid series
with Moshier planetary reference coordinates. Mean/true nodes and mean/osculating
apogee are computed by Swiss's built-in algorithms. No filesystem or environment
lookup has been enabled. The fallback on other operating systems uses a temporary
file containing these embedded bytes only; those targets are not yet validated.
The data has the same upstream Swiss licensing requirements as the provider.
