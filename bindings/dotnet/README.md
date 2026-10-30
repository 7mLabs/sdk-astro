# 7mlabs Astro SDK for .NET

Offline astrology calculations, structured chart data, optional IANA birth-time
conversion and modular SVG charts, using the shared 7mlabs Astro engine.

## Install and supported runtimes

```sh
dotnet add package SevenMLabs.Astrology --version 1.0.2
```

The managed assembly targets **.NET 10 (`net10.0`)**. The public package includes
native assets for **macOS ARM64 (`osx-arm64`)** and **Linux x64 (`linux-x64`,
glibc 2.38+ with `libgcc_s.so.1`)**. The host OS must also meet .NET 10 runtime
requirements. Windows, Intel Mac, Linux ARM64 and Alpine/musl are not included.
The engine needs no server, runtime download, external ephemeris installation,
Rust compiler or API key. Native libraries are shipped under `runtimes/<rid>/native`.

## Birth time and SVG

```csharp
using SevenMLabs.Astrology;

var payload = Engine.CalculateFromLocal(new {
    operation = "natalDomains",
    local = new { year = 1995, month = 6, day = 15, hour = 14, minute = 30 },
    timeZone = "Asia/Ho_Chi_Minh",
    location = new { latitude = 10.8231, longitude = 106.6297 },
    domains = new[] { "identity" },
    additionalPoints = "standard"
});
var svg = Engine.RenderChartSvg(payload, new {
    theme = "dark", locale = "en",
    sections = new[] { "wheel", "houseFocus" },
    focus = new { house = 7, mode = "highlight" },
    aspectColors = new { harmonious = "#54CFAA", challenging = "#F57891" }
});
File.WriteAllText("chart.svg", svg);
var positions = Engine.RenderChartPart(payload, "positions");
var model = Engine.BuildChartModel(payload, new { sections = new[] { "wheel" } });
```

Already have UTC? Call `Engine.Calculate` with `operation`, `utc` and `location`.
`Engine.ResolveBirthTime` is also available separately, so an application can
convert first and reuse the UTC result. Conversion is optional and uses bundled
tzdb 2026e; DST gaps fail, folds require an explicit earlier/later choice. Location
does not infer a timezone. Event/forecast periods still use fixed offsets.

## Calculation and presentation APIs

- Natal Sun–Pluto, Placidus/Whole Sign houses, angles and configurable aspects.
- Optional Chiron, true/mean north and south nodes, mean/osculating lunar apogee
  (Lilith) for natal/natalDomains. `standard` selects Chiron, true north node and
  mean Lilith; `all` selects all seven definitions. Extended points remain separate
  from the ten physical bodies and their distributions/rulership.
- Ten individual domains, source evidence, custom selection profiles, synastry,
  symbolic midpoint composite, transits, forecasts and event searches.
- Geometry, aspect, house and point queries through grouped helpers.
- Separate payload context compression with explicit coverage and omission data.
- Self-contained SVG presets and independent `wheel`, `positions`, `houses`,
  `aspectMatrix`, `aspects`, `houseFocus` parts. Themes, relation filters, grouped
  colors and custom safe vector symbols are configurable.
- JSON chart models include original longitudes, source house rulers/occupants,
  viewBox positions and hit targets for a custom UI. View changes reuse the payload.

Basic natal uses Gregorian dates 1800–2399, tropical geocentric apparent positions.
Coordinates are degrees, north/east positive; the engine retains floating-point
coordinates. House-ruler focus requires source context such as natalDomains.
Polar Placidus failures return an error without silently switching house systems.
Composite charts are symbolic and have no invented physical speed or epoch.

Calls are synchronous; use application workers for long searches. `Engine.Calculate`
returns an owned `JsonElement`; `EngineException` exposes `Code` and `ResultJson`.
Raw JSON APIs preserve the engine envelope. `Engine.RenderChartSvgResult` returns
SVG, model, metadata, warnings and coverage; `Engine.GetCapabilities` describes
available services. Native output is freed by the wrapper.

## Documentation and license

[Repository](https://github.com/7mlabs/sdk-astro) ·
[API contracts](https://github.com/7mlabs/sdk-astro/blob/main/docs/time-and-svg.md) ·
[1.0 release contract](https://github.com/7mlabs/sdk-astro/blob/main/docs/release-1.0.md)

The SDK and renderer use **AGPL-3.0-only**, using the AGPL option of Swiss Ephemeris.
Full license text, NOTICE and upstream dependency/font notices are included.
Corresponding source and build scripts are in the matching `v1.0.2` source release.
