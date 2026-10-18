# Civil time và SVG chart — 1.0.0

Các API dưới đây có trong **1.0.0** đã phát hành npm/PyPI; NuGet 1.0.0 đang chờ registry xác thực/lập chỉ mục. [Trạng thái và bằng chứng release](release-1.0.md). Các lời gọi UTC cũ giữ nguyên hành vi; đổi giờ và điểm mở rộng là tùy chọn rõ ràng.

## Hai luồng tích hợp

```js
// Node: npm install @7mlabs/astrology@1.0.0
const {
  calculate, resolveBirthTime, calculateFromLocal,
  renderChartSvg, renderChartSvgResult, getCapabilities
} = require('@7mlabs/astrology');

const time = {
  local: { year: 1995, month: 6, day: 15, hour: 14, minute: 30, second: 0 },
  timeZone: 'Asia/Ho_Chi_Minh'
};
const location = { latitude: 10.8231, longitude: 106.6297 };

// A: backend đã đổi UTC bằng cơ chế riêng.
const a = calculate({ operation: 'natal', utc: backendUtc, location });

// B: dùng helper của SDK, tách hai bước để tái sử dụng UTC.
const resolved = resolveBirthTime(time);
const b = calculate({ operation: 'natal', utc: resolved.utc, location });

// B, gọi gộp: SDK resolve rồi gọi chính core cũ.
const payload = calculateFromLocal({ operation: 'natal', ...time, location });
const svg = renderChartSvg(payload, { detail: 'advanced', theme: 'dark' });
```

Ví dụ trên là các lựa chọn riêng; `backendUtc` là UTC do backend của caller chuẩn bị. `calculateFromLocal` mặc định operation `natal` khi bỏ operation. Hỗ trợ `natal`, `natalDomains`; local birth bên trong `personA/personB` của `couple/composite` và `birth` của `forecast`/birth-based `query`. Có thể trộn người A đã có UTC, người B dùng local; cùng một birth không được có cả hai. Nếu không có birth local, dùng `calculate`.

Kết quả gọi gộp giữ envelope cũ, bổ sung `calculation.timeResolution.birth` (hoặc `personA/personB`) cùng warnings. Kết quả helper riêng là object có `utc`, `local`, `timeZone`, `utcOffsetSeconds`, `tzdbVersion`, `resolution`, `precision`, `warnings`.

### Quy tắc thời gian

- Bắt buộc đúng một trong `timeZone` IANA hoặc `utcOffsetSeconds` tường minh (số nguyên, trị tuyệt đối <86400). Offset dương là phía đông UTC.
- Dữ liệu IANA **2026e**, pin qua `jiff-tzdb 0.1.9`; dùng TZif trực tiếp, không đọc timezone máy, biến `TZ`, zoneinfo hệ điều hành hoặc gọi mạng. Có quy tắc lặp DST sau 2099; tương lai vẫn chỉ theo dữ liệu hiện biết, không dự đoán thay đổi chính trị.
- Mặc định `disambiguation: 'reject'`. Fold (giờ xuất hiện hai lần) báo `AMBIGUOUS_LOCAL_TIME`, có thể chọn `'earlier'` hoặc `'later'`. Gap (giờ không tồn tại) luôn báo `NONEXISTENT_LOCAL_TIME`; không tự dịch giờ dù chọn earlier/later.
- Năm local và UTC sau chuyển đổi phải trong 1800–2399. Ngày Gregorian và giờ phải hợp lệ; không nhận birth leap second 60.x. Thành phần ngày/giờ nguyên dùng integer deserialization; `second` được giữ tới nanosecond khi quy đổi và chuyển sang số thực f64 cho core.
- Múi giờ lịch sử trước 1970 có cảnh báo về độ đầy đủ của dữ liệu. Có thể dùng offset đã được kiểm chứng riêng, kể cả offset có phần giây.
- Helper này xử lý **birth time**. `events/forecast.period.utcOffsetMinutes` vẫn là offset cố định của kỳ tính, chưa thành IANA calendar window tự thay offset qua DST.
- Tọa độ địa lý không tự suy ra timezone. Việc tìm địa điểm → timezone là một lớp riêng của ứng dụng.

Nguồn hành vi: [Jiff TZif](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html), [bundled tzdb](https://docs.rs/jiff-tzdb/latest/jiff_tzdb/), [giới hạn lịch sử IANA](https://www.iana.org/time-zones/theory).

## Renderer SVG

`renderChartSvg(payload, options)` trả string. `renderChartSvgResult` trả `{ svg, metadata, warnings, coverage }`. Renderer không gọi provider và không sửa payload.

| Option | Giá trị |
| --- | --- |
| `detail` | `basic` mặc định; `advanced` thêm độ/phút trên wheel một lớp và bảng vị trí; `professional` thêm nhà/trục, ma trận và danh sách góc chiếu |
| `theme`, `locale` | `dark`/`light`; `vi`/`en` |
| `size` | Chiều rộng 128–4096 px, mặc định 1000; vẫn là vector với viewBox |
| `chart` | `natal`, `A`, `B`, `C`, `synastry`, `transits`; tự chọn từ loại payload nếu bỏ |
| `orientation` | `ascendant` mặc định (ASC bên trái), hoặc `aries` |
| `showAspects` | Mặc định true |
| `maxAspectLines`, `aspectAngles` | Filter hiển thị; mặc định không âm thầm cắt 40 nét |
| `bodyIds` | Sun–Pluto và các ID điểm mở rộng có trong payload |
| `aspectColors` | Màu hex theo nhóm hoặc loại; loại > nhóm > theme |
| `aspectGroups`, `maxOrb`, `aspectPhase` | Lọc theo nhóm, orb tối đa và applying/separating/unknown từ payload |
| `display` | Bật/tắt degreeTicks, planetDegrees, houses, angles, positionsTable, housesTable, aspectTable, aspectMatrix, legend |
| `maxAspectRows` | Giới hạn hàng danh sách, mặc định 256; tối đa 4096, công bố số hàng bỏ qua, không cắt wheel/ma trận |
| `zodiacSymbols`, `planetSymbols` | Partial override geometry; cung/hành tinh còn lại giữ default |
| `title`, `instanceId` | Title tối đa 100 ký tự font hỗ trợ; ID prefix ASCII khi nhúng nhiều chart |

Đầu vào là envelope SDK thành công schema `1.0`. Natal basic/advanced, natalDomains, natal A/B, synastry/couple, symbolic composite C và natal/transit snapshot có adapter. Transit snapshot không phải toàn bộ forecast timeline. Harmonic/positions có thể dùng adapter natal nếu đủ geometry nhà và góc; không công bố hỗ trợ mọi payload geometry thiếu house/ASC.

### Độ chính xác và layout

- Dùng longitude/cusp/angle nguồn f64 để đặt điểm và nối aspects. Không làm tròn về phút trước khi vẽ.
- Nhãn có thể được giãn bằng isotonic layout; marker giữ nguyên tọa độ, leader line chỉ về vị trí thật. Mỗi layer tránh chồng nhãn hành tinh riêng.
- DMS **cắt xuống trong cung nguồn**, không hiện 30° với tên cung cũ. Metadata giữ tọa độ gốc; DMS không thay thế dữ liệu tính toán.
- ASC/MC/DSC/IC tách khỏi cusp; không ép MC thành H10 khi Whole Sign.
- Aspects lấy từ SDK, không tính lại orb. Endpoint không được vẽ/filter/giới hạn nét đều có số đếm và lý do tại `coverage.excluded`; `renderedAspectData` giữ aspects đã vẽ.
- Chart hai lớp lấy house grid và góc trục theo lớp thứ nhất; cusp lớp thứ hai nét đứt. Composite không tạo timestamp/speed/Rx giả.

### Trình bày và đọc biểu đồ

Wheel dùng các vùng cung có nền màu nhẹ, ký hiệu lớn, trục ASC/DSC và MC/IC phân màu, marker tại vị trí thật và đường dẫn tới nhãn đã giãn. Chú giải sinh từ các góc thực sự được vẽ và dùng đúng màu/nét đã resolve. Mặc định sextile xanh lam, trine xanh lục, square đỏ hồng, opposition cam và conjunction tím; nét cũng khác để dễ phân biệt. Bảng nâng cao có tiêu đề cột và hàng xen nền. Tất cả vẫn là vector tự chứa; màu và kích thước hiển thị không thay tọa độ nguồn. Độ/phút trên wheel dùng cùng hàm cắt DMS với bảng, chỉ bỏ phần giây.

Playground có **Phóng to / Vừa khung**: xem SVG ở chiều rộng 1000 px và cuộn bên trong khung trên màn hình hẹp. Trạng thái phóng to chỉ thuộc UI, không thay file SVG xuất ra. Đây vẫn là biểu đồ tĩnh; có ma trận và danh sách góc chiếu trong preset professional; chưa có chọn/hover tương tác phân tích trong file SVG.

### SVG tự chứa và custom pack

Zodiac dùng geometry 7mlabs, 10 glyph hành tinh và 7 glyph điểm mở rộng là vector convention riêng; chữ/số được outline từ Plus Jakarta Sans Medium (721 glyph). Không có `<text>`, CSS ngoài, font ngoài hoặc ảnh raster. Title/desc/XML metadata vẫn giữ nội dung đọc được. Hai theme dùng màu đã resolve, không dùng CSS variables.

```js
const rendered = renderChartSvgResult(payload, {
  detail: 'advanced', theme: 'light', instanceId: 'chart-profile-42',
  zodiacSymbols: {
    aries: {
      viewBox: [0, 0, 32, 32],
      nodes: [['path', { d: 'M4 28L16 4L28 28' }]]
    }
  }
});
```

Chấp nhận geometry `path/circle/ellipse/line/rect/polyline/polygon`; font/label bất kỳ ngoài bộ Latin/Vietnamese hiện có báo `UNSUPPORTED_LABEL`. Có thể truyền map từ `createZodiacSymbols` của brand kit; metadata tên/signIndex được kiểm tra nhưng không đổi dữ liệu thiên văn. Không nhận raw SVG, script/event/style/href; kiểm tra path grammar, finite coordinates và budgets. Cùng input/options/version cho cùng SVG bytes. Dùng `instanceId` khác nhau khi nhúng nhiều SVG trực tiếp trong một DOM.

Alpha này chưa vẽ mọi bảng report/facts/pattern annotations, chưa có browser WASM và chưa tuyên bố giải mọi va chạm giữa nhãn với góc trục trong mọi chart nhiều lớp. Các lớp chưa cung cấp dữ liệu không được bù bằng dữ liệu mẫu.

## Bindings và phân tách dependency

- Node: `resolveBirthTime`, `calculateFromLocal`, `renderChartSvg`, `renderChartSvgResult`, `getCapabilities`; raw variants `resolveBirthTimeJson`, `calculateFromLocalJson`, `renderChartSvgJson(payloadJson, optionsJson)` và `renderChartSvgRequestJson(wrapperJson)`.
- Python: cùng ý nghĩa, tên snake_case; helper và renderer result trả `dict`, SVG convenience trả string.
- .NET: `Engine.ResolveBirthTime`, `CalculateFromLocal`, `RenderChartSvg`, `RenderChartSvgResult`, `GetCapabilities`; raw JSON methods tương ứng.
- C: thêm `astro_resolve_birth_time_json`, `astro_calculate_from_local_json`, `astro_render_chart_svg_json`, `astro_capabilities_json`; luôn free output bằng `astro_free_string`. ABI vẫn 1 do chỉ thêm export.
- Rust: `astro-core` vẫn là core UTC; `astro-time` chỉ chuyển giờ; `astro-render-svg` không phụ thuộc provider; `astro-sdk` là lớp compose/JSON facade. Người chỉ dùng core hoặc renderer có thể chọn crate đó, không bắt liên kết toàn bộ services.

Raw APIs trả envelope chứa errors; object APIs kiểm tra envelope và throw/raise theo code. Giới hạn input: time helper 16 KiB, local calculation 4 MiB, renderer 32 MiB. Binary/wrapper phải cùng candidate; addon cũ không có services sẽ báo cần rebuild/install.

## Chạy preview và xuất file

Sau `scripts/build-packages.py`, chạy host bằng package đã cài hoặc chỉ định module source có binary vừa build:

```sh
ASTRO_ENGINE_MODULE="$PWD/bindings/node/index.cjs" PORT=4184 node examples/frontend/host.cjs
# Mở http://127.0.0.1:4184/ui/svg.html
ASTRO_ENGINE_MODULE="$PWD/bindings/node/index.cjs" node examples/node/time-svg.cjs examples/svg-output
```

Preview có nhập local/timezone/location, xử lý fold/gap, hai theme, basic/advanced/professional, lựa chọn điểm mở rộng, bộ màu và bộ lọc đường, cùng tải đúng SVG đang xem. Host chỉ bind loopback, kiểm tra Host/Origin, bounded queue. `/api/astro/calculate` giữ nguyên; thêm POST `calculate-from-local`, `resolve-birth-time`, `render-svg`, `capabilities` dưới `/api/astro/`. Đây là harness local, chưa thay backend production của Payload.

Font license/provenance nằm trong `neutral-engine/crates/astro-render-svg/assets/`; packages chứa OFL và Rust dependency notices. Muốn đồng bộ brand assets, chạy `scripts/sync-service-assets.py --brand-root ...` với fontTools; build/runtime không cần fontTools hoặc thư mục brand bên ngoài.

## Kiểm chứng

Các lệnh chính: `cargo test`, `cargo fmt --check`, Clippy `-D warnings`; `node tests/services/time-svg.cjs`; fresh package install/parity qua `scripts/test-packages.py`; service parity qua `scripts/check-time-svg-bindings.py`. Không suy từ pass một máy sang mọi nền tảng. Chi tiết candidate và giới hạn kiểm chứng được cập nhật trong [testing.md](testing.md).

## Điểm mở rộng và cấu hình từ cơ bản đến chuyên sâu

```js
const payload = calculateFromLocal({
  operation: 'natal',
  local: { year: 1995, month: 6, day: 15, hour: 14, minute: 30 },
  timeZone: 'Asia/Ho_Chi_Minh',
  location: { latitude: 10.8231, longitude: 106.6297 },
  additionalPoints: 'standard',
  aspectPreset: 'extended'
});
const rendered = renderChartSvgResult(payload, {
  detail: 'professional',
  aspectColors: {
    harmonious: '#29b999', challenging: '#f16e82', neutral: '#b39aff',
    trine: '#4cce8a' // Loại cụ thể ưu tiên hơn nhóm.
  },
  display: { aspectMatrix: true, housesTable: true },
  maxAspectRows: 4096
});
```

`additionalPoints` dùng được cho `natal` và `natalDomains`, cả UTC lẫn local facade:

| Giá trị | Điểm được tính |
| --- | --- |
| bỏ option hoặc `[]` | Không thêm điểm; giữ contract 10 thiên thể |
| `standard` | `chiron`, `trueNorthNode`, `meanLilith` |
| `all` | Cả 7 ID dưới đây |
| mảng ID | Chọn chính xác; không trùng, không alias mơ hồ |

Các ID: `chiron`, `trueNorthNode`, `trueSouthNode`, `meanNorthNode`,
`meanSouthNode`, `meanLilith`, `osculatingLilith`.
South Node là điểm đối của North Node cùng định nghĩa. Lilith ở đây là viễn điểm
quỹ đạo Mặt Trăng (mean hoặc osculating), không phải asteroid 1181 Lilith.
Chiron là minor body; node/apogee là điểm tính toán. Không coi tất cả là hành tinh.
Kết quả nằm ở `data.additionalPoints`, hoặc `data.natal.additionalPoints` với
natalDomains. Mỗi điểm có longitude/speed/house/Rx và definition/ephemeris.
Tọa độ giữ f64, không làm tròn trước khi tính nhà, góc chiếu hay SVG.

Các góc chiếu natal tính cả điểm được chọn; natalDomains thêm chúng vào context
relations nhưng giữ body distributions/dignity/rulership trên 10 thiên thể.
Custom profile body selectors vẫn chỉ Sun–Pluto; điểm thêm xuất hiện như endpoint
của các góc liên quan. Chưa mở rộng tính các điểm này cho couple/composite,
forecast/events/query: birth lồng nhau không nhận option này và sẽ báo lỗi,
không âm thầm bỏ qua. Preset professional là preset **hiển thị**, không tự bật
thêm tính toán hoặc thay aspect rules. `aspectPreset: 'extended'` trên natal
bật 11 góc; không truyền đồng thời `aspectRules`.

Chiron dùng `seas_18.se1` chính thức, nhúng trong binary; metadata ghi checksum.
Không cần tải file lúc chạy hoặc cấu hình đường dẫn. Sun–Pluto và các điểm lunar
vẫn dùng Moshier. Phạm vi civil UTC natal vẫn 1800–2399.
Provenance: [Swiss provider](../neutral-engine/crates/astro-provider-swiss/PROVENANCE.md).
Không khẳng định đây là độ chính xác JPL hoặc toàn bộ hệ điểm chiêm tinh.

Nhóm màu: `harmonious` (60/120), `challenging` (90/180), `neutral` (0),
`minor` (30/45/72/135/144/150), `other` (góc tự cấu hình khác).
Loại màu: conjunction, sextile, square, trine, opposition, semisextile,
semisquare, quintile, sesquiquadrate, biquintile, quincunx.
Chỉ nhận hex `#RGB/#RGBA/#RRGGBB/#RRGGBBAA`, không nhận CSS/script/URL.
Wheel, chú giải, ma trận và danh sách dùng chung màu và tập góc đã lọc.
Orb và applying lấy từ source; không suy đoán khi source không có.
`showAspects:false` tắt toàn bộ tập góc; `maxAspectLines` cũng áp dụng cho ma trận
và danh sách để các phần nhất quán. Giới hạn riêng danh sách được báo tại
`coverage.aspectTableRows/aspectTableOmitted` cùng warning.

Python dùng cùng object options (dict), .NET dùng anonymous object/JSON và C dùng
request JSON; tên key và enum giống nhau qua các binding.

## Functional parts, chart model and house focus (2026-10-18)

The renderer 1.2.0 supports independent parts. Existing
`renderChartSvg(payload, options)` calls keep preset-based composition; no new
astronomical calculation is needed to switch views.

```js
const payload = calculate({
  operation: 'natalDomains', utc, location,
  domains: ['identity'], // shared context still includes all twelve houses
  additionalPoints: 'standard', aspectPreset: 'extended'
});
const wheel = renderChartPart(payload, 'wheel');
const houses = renderChartPart(payload, 'houses');
const selected = renderChartSvgResult(payload, {
  sections: ['wheel', 'houseFocus'],
  focus: { house: 7, mode: 'highlight' }
});
const model = buildChartModel(payload, {
  sections: ['wheel'], focus: { house: 7, mode: 'isolate' }
}); // no SVG string is generated
const report = renderChartSvg(payload, {
  sections: ['positions', 'houses', 'aspectMatrix', 'aspects']
}); // order is explicit; no hidden wheel or blank wheel-sized placeholder
```

Parts: `wheel`, `positions`, `houses`, `aspectMatrix`, `aspects`, `houseFocus`.
`sections` is an ordered, nonempty, unique list and takes precedence over preset
section selection. Wheel layer options in `display` still apply. Omit `sections`
to use basic/advanced/professional composition as before. `houseFocus` requires
`focus.house`; other parts may also use focus. A standalone part has its own
header, footer, viewBox and vector content. Parts are drawn directly, not cropped
from a prebuilt long SVG.

`buildChartModel` returns the same `model` included in `renderChartSvgResult`:

- `points`: original longitude, house, Rx, sign and display DMS; separate label
  and marker positions in final viewBox units, selection and opacity.
- `houses`: cusp, angular width, all occupant IDs (including selected additional
  points), source ruler ID/placement and declared rulership when available.
- `angles`: original ASC/MC/DSC/IC longitudes for each layer.
- `aspects`: the filtered source aspects, endpoint IDs, resolved colors and line
  styles, with start/end geometry when a wheel is included.
- `sections` and `viewBox`: final composition geometry.
- `hitTargets`: point and house targets in viewBox units for DOM, canvas or native
  controls. An SVG/HTML overlay can use `x/viewBoxWidth`, `y/viewBoxHeight`;
  preserve the same aspect ratio. No wheel means null wheel positions and no
  hit targets, rather than misleading coordinates in a nonexistent component.
- `selectedPointIds`: primary focus; `relatedPointIds`: primary IDs and the other
  endpoints of selected aspects. Focus expansion is one hop, not recursive.

Focus accepts either `{house: 1..12, layer?: 'N'|'A'|'B'|'C'}` or
`{pointIds: [...]}`, plus `mode: 'highlight'|'isolate'`. Point focus accepts body/additional-point IDs present
in the source (use `house` for house focus); multi-layer point IDs are qualified (`A:sun`, `B:moon`). In house
focus, the primary set is the cusp, its occupants and its source ruler. Only
aspects with a primary endpoint are selected. Highlight retains context and dims
unrelated labels; isolate retains primary points and the endpoints of their
filtered aspects. `bodyIds`, aspect filters and budgets continue to apply.
A house cusp can be an aspect endpoint during focus. It does not become a fake
physical body in the positions table.

House focus requires source rulership context: use natalDomains, an existing
subject context or composite context. A plain natal payload can still render
wheel/positions/houses/aspects and point focus, but missing rulers remain null;
requesting house focus returns `MISSING_HOUSE_CONTEXT`. Renderer never chooses a
new rulership convention. `natalDomains`'s `rulership` controls traditional/modern
upstream. Pair focus follows the chosen chart's cross aspects; it does not mix in
individual natal aspects silently. House grid/angle labels remain anchored to
the first wheel layer.

In Python use `render_chart_part` / `build_chart_model`; in .NET use
`Engine.RenderChartPart` / `Engine.BuildChartModel`. C/Rust raw transport uses
existing `render_chart_svg_json` with `options.sections/focus`; set
`options.output: 'model'` for a model-only response. ABI stays 1.

The playground defaults to wheel only. It offers functional view buttons, house
and point selection, direct wheel hit targets, highlight/isolate, SVG export of
the current composition and JSON model export. Only form submission recalculates;
view/filter/focus changes reuse the payload. Full report remains an explicit view.
The SDK model does not install an interactive UI framework or browser WASM.

`aspectEndpoints` controls which source endpoint categories can be drawn:
`bodies` (Sun–Pluto and additional points), `bodiesAndAngles`, or `all` (including
house cusps). Default is `bodies` without focus, `all` with focus. It applies to
wheel, matrix, list and model together. The playground explicitly defaults to
`bodies` for readability; choose “Cả nhà & trục” when exploring cusp contacts.
