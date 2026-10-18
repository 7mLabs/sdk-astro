# 7mlabs Astro SDK for Python

An offline SDK for astrology calculations, structured chart data, birth-time
conversion and modular SVG rendering. Astro is a project of
[7mlabs](https://7mlabs.com/astro/), with SDK, Payload (web) and Compass (app)
as its three development paths.

## Installation and platform support

```sh
python -m pip install 'sevenmlabs-astrology==1.0.0'
```

This wheel supports **Python 3.10+ on macOS 11+ Apple Silicon (ARM64)**.
Linux, Windows and Intel Mac wheels are not included in this initial Python
release. This native package runs in backend, desktop and command-line Python
applications; it does not execute directly in a standard web browser.

The wheel includes the native engine, timezone data, required ephemeris data,
vector symbols and font outlines. No Rust compiler, runtime downloads, API key
or external ephemeris installation is needed when using the supported wheel.

## Calculate from local birth time

```python
from pathlib import Path
import sevenmlabs_astrology as astro

payload = astro.calculate_from_local({
    "operation": "natalDomains",
    "local": {"year": 1995, "month": 6, "day": 15, "hour": 14, "minute": 30},
    "timeZone": "Asia/Ho_Chi_Minh",
    "location": {"latitude": 10.8231, "longitude": 106.6297},
    "domains": ["identity"],
    "additionalPoints": "standard",
    "aspectPreset": "extended",
})

svg = astro.render_chart_svg(payload, {
    "detail": "professional", "theme": "dark", "locale": "en",
    "sections": ["wheel", "positions"],
    "aspectColors": {
        "harmonious": "#54CFAA", "challenging": "#F57891",
        "neutral": "#A38BFA", "minor": "#8193A8",
    },
})
Path("chart.svg").write_text(svg, encoding="utf-8")
```

If your backend already converts to UTC, call `calculate` with `utc` and
`location` instead. Conversion remains optional:

```python
resolved = astro.resolve_birth_time({
    "local": {"year": 1995, "month": 6, "day": 15, "hour": 14, "minute": 30},
    "timeZone": "Asia/Ho_Chi_Minh",
})
payload = astro.calculate({
    "operation": "natalDomains", "utc": resolved["utc"],
    "location": {"latitude": 10.8231, "longitude": 106.6297},
    "domains": ["identity"],
})
```

IANA conversion uses bundled tzdb 2026e. Ambiguous local times require an explicit
earlier/later choice; nonexistent local times fail. Coordinates do not determine
a timezone automatically. Event/forecast period windows still use an explicit
fixed `utcOffsetMinutes`.

## Independent chart parts and interactive applications

Calculate once, then reuse that payload for different views without recalculating
planet positions:

```python
positions_svg = astro.render_chart_part(payload, "positions", {"theme": "light"})
houses_svg = astro.render_chart_part(payload, "houses")
focus_svg = astro.render_chart_svg(payload, {
    "sections": ["wheel", "houseFocus"],
    "focus": {"house": 7, "mode": "highlight"},
    "aspectEndpoints": "all",
})
model = astro.build_chart_model(payload, {
    "sections": ["wheel"],
    "focus": {"house": 7, "mode": "isolate"},
})
```

Parts: `wheel`, `positions`, `houses`, `aspectMatrix`, `aspects`, `houseFocus`.
Presets: `basic`, `advanced`, `professional`. The JSON model exposes source
longitudes, selected/related points, house rulers and occupants, final viewBox
coordinates and hit targets for a custom frontend. Model-only output does not
generate SVG. House-ruler focus requires source context such as `natalDomains`.

`render_chart_svg_result` returns SVG, model, warnings, coverage and metadata.
Filter relations by groups, angles, orb, phase or endpoint category. Override
colors by group or aspect type, and optionally supply custom safe vector geometry
for zodiac/point symbols. Glyphs and text are outlined; exports need no external
fonts, images, scripts or stylesheets.

## Calculation coverage

- Natal Sun–Pluto, houses, ASC/MC/DSC/IC and aspect relations.
- Optional Chiron, true/mean north and south nodes, mean/osculating lunar apogee
  (Lilith). `additionalPoints: "standard"` selects Chiron, true north node and
  mean Lilith; `"all"` selects all seven, or pass an explicit ID list. These points
  are supported for `natal` and `natalDomains`.
- Ten individual report domains, source facts/evidence and custom profiles.
- Two-person synastry, symbolic midpoint composite, transits, event searches,
  forecasts, harmonic/chart geometry and grouped on-demand queries.
- Separate payload context compression with explicit omission/coverage metadata.

Basic natal uses Gregorian dates 1800–2399, tropical geocentric apparent positions,
and Placidus or Whole Sign houses. Coordinates are degrees, north/east positive.
Core output retains floating-point coordinates; display formatting does not alter
calculation precision. Polar Placidus failures return errors rather than silently
switching house systems. Composite charts are symbolic and have no invented
physical speed/retrograde/applying values.

Calls are synchronous. Long event/forecast searches belong in an application
worker. The SDK returns structured data rather than generated narrative.
`EngineError` exposes `.code` and `.result`; raw JSON helpers retain the engine
envelope. `get_capabilities()` describes the installed engine.

## Source, documentation and license

This release is **1.0.0**. The matching project source snapshot
and build instructions are bundled at
`sevenmlabs_astrology/source/astro-sdk-1.0.0.tar.gz` inside the wheel.
The archive includes the Rust/C engine, Python wrapper, locked dependency
declarations, embedded data, renderer assets, schemas, tests and dependency
notices; it excludes credentials and local build outputs.

[Repository](https://github.com/7mlabs/sdk-astro) ·
[Issues](https://github.com/7mlabs/sdk-astro/issues)

The bundled source contains `docs/time-and-svg.md` for complete rendering/time
contracts and `docs/license.md` for license details. The public repository may
advance independently; use the bundled snapshot to reproduce this wheel.

The SDK and native renderer use **AGPL-3.0-only**, using the AGPL option of Swiss
Ephemeris. License texts and third-party notices are included; upstream font and
dependency terms are retained. See the included `LICENSE` and `NOTICE`.
