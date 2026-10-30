# Kế hoạch phát triển SVG chart cho Astro SDK

Cập nhật phạm vi 18/10/2026 theo yêu cầu: đã triển khai opt-in Chiron,
mean/true Nodes, mean/osculating Lilith cho natal/natalDomains; professional SVG,
ma trận, danh sách, bộ lọc và màu theo nhóm/loại. Các phần bên dưới là roadmap gốc;
[API hiện hành](time-and-svg.md) mô tả contract và giới hạn mới.

Cập nhật trạng thái 2026-10-30: SDK **1.0.2** đã phát hành trên npm/PyPI/NuGet; xem [bằng chứng hiện hành](release-1.0.2.md). Phần dưới là kế hoạch lịch sử, không phải trạng thái phát hành hiện tại.

Ngày lập: **17/10/2026**. Baseline source: **0.10.1**, commit `cbfadc1`. Kế hoạch gốc được giữ làm roadmap. **Đã có implementation candidate 1.0.0**, gồm local-time helpers, basic/advanced SVG và bindings; xem [API hiện hành và phạm vi alpha](time-and-svg.md). Chưa phát hành registry. Đánh giá baseline đi kèm: [SDK health review](health-review-2026-10-17.md).

## 1. Mục tiêu và ranh giới

Astro là một dự án của 7mlabs, gồm **SDK → Payload (Web) và Compass (App)**. Xuất SVG là tính năng riêng của SDK: ứng dụng có thể dùng dữ liệu tính toán, renderer, hoặc cả hai. Payload là nơi tích hợp và kiểm chứng thực tế đầu tiên.

Đầu ra cần có:

- Chart cơ bản và nâng cao, xuất SVG tự chứa, dùng được ngoài trang web.
- Bộ ký hiệu mặc định mang nhận diện 7mlabs; thay toàn bộ hoặc một phần bằng bộ custom.
- Hình học đúng dữ liệu SDK: longitude, cusp, angles, aspects và nguồn chart không bị thay đổi bởi layout.
- Kết quả nhất quán giữa các bindings được công bố hỗ trợ.

Không đưa React/DOM vào engine; không tính lại ephemeris hoặc tự đổi orb trong renderer. Không lấy toàn bộ giao diện web cũ làm thư viện SDK. Không bổ sung nodes/asteroids/Lots chỉ để giống ảnh cũ khi core chưa cung cấp chúng.

## 2. Kiến trúc đề xuất

```text
Calculation result JSON hiện tại
          │
          ▼
Adapter + validation ──► ChartScene có phiên bản
                              │
            symbol pack + theme + layout options
                              │
                              ▼
                  Pure SVG renderer (Rust)
                              │
                     SVG + diagnostics
                              │
                Node / Python / .NET / C / Rust
```

Thêm crate riêng, tên làm việc `astro-render-svg`, có thể dùng mà không gọi provider Swiss. Renderer đọc payload đã có, không yêu cầu tái cấu trúc toàn bộ core. Các bindings dùng cùng renderer native; browser dùng kết quả SVG từ host/backend trong giai đoạn đầu. Chưa coi native addon là module chạy trong browser; WASM là hạng mục riêng.

`ChartScene` chứa layers, placements, cusps, angles, aspect endpoints, source metadata và coverage. Đây là model nội bộ; không tạo một payload tính toán thứ hai để ứng dụng phải đồng bộ. Adapter kiểm tra phiên bản/schema và báo lỗi có mã, thay vì nhận tùy ý mọi object.

### Contract API dự kiến

```js
const payload = calculate(request);

const svg = renderChartSvg(payload, {
  detail: 'basic',
  theme: 'dark',
  size: 1000,
  locale: 'vi',
});

const result = renderChartSvgResult(payload, {
  detail: 'advanced',
  theme: 'light',
  zodiacSymbols: customPack,
  instanceId: 'payload-preview',
});
// result: { svg, metadata, warnings, coverage }
```

`renderChartSvg` là convenience trả string; `renderChartSvgResult` phục vụ ứng dụng cần diagnostics. API tính toán hiện tại giữ nguyên. Render không trở thành operation thứ 11 của `calculate`. Input không hợp lệ/thiếu trường bắt buộc phải thất bại rõ ràng; trường tùy chọn thiếu có thể bỏ lớp với lý do trong coverage.

Renderer có version/rules version riêng, ghi trong metadata. Thêm C export dự kiến `astro_render_svg_json` trả envelope JSON chứa SVG; quyền sở hữu output vẫn theo `astro_free_string`. Chỉ giữ ABI hiện tại nếu bổ sung tương thích; loader phải kiểm tra capability/version trước khi gọi export mới. Đổi signature không tương thích mới cần bump ABI.

## 3. Hai mức chart và các loại chart

### Basic: MVP natal trước

- 12 cung; Sun–Pluto hiện có; 12 nhà; ASC/MC và các trục liên quan theo payload.
- Marker đúng longitude, có leader line khi nhãn phải dịch để tránh chồng.
- Aspects từ SDK; filter và số lượng đã vẽ/đã bỏ phải truy xuất được.
- Hai theme, bố cục vuông co giãn, title/description và metadata nguồn.

### Advanced: tăng độ chi tiết, không tự phát sinh phép tính

- Độ/phút và Rx theo dữ liệu hợp lệ; không tự tạo motion cho composite.
- Chọn điểm, lớp, aspects, nhãn và chú giải; hiển thị facts/configurations thật có trong payload khi người dùng bật.
- Bộ ký hiệu/theme custom, tùy chọn độ dày nét, cỡ nhãn và nền.
- Góc trục khác cusp phải được giữ riêng, đặc biệt MC trong Whole Sign.

Natal basic/advanced là milestone đầu. Sau đó mới thêm hai vòng A/B cho synastry/couple, composite C, rồi natal + transit snapshot. Mỗi loại có adapter và nghiệm thu riêng. Forecast timeline không mặc nhiên là một vòng transit tại một thời điểm; cần chọn snapshot rõ ràng.

## 4. Ký hiệu, font và SVG tự chứa

12 zodiac vector đã có trong brand kit: `icons/source/zodiac.json`, `icons/zodiac/registry.json`, `icons/zodiac/symbols.mjs`. Module hiện tại chỉ vẽ **một ký hiệu**, chưa phải renderer chart. Khi đưa vào SDK, sinh bundle geometry từ nguồn chuẩn, ghi revision/hash và notices; không đọc thư mục brand bên ngoài tại runtime.

Cần hoàn thiện thêm bộ vector Sun–Pluto, ký hiệu Rx và angle labels. Dùng một hệ nét/hình học thống nhất với zodiac; tránh font Unicode thiếu glyph. Kiểm tra độ dễ đọc ở kích thước chart nhỏ, không dùng gradient phức tạp mặc định cho mọi ký hiệu nhỏ.

Chốt policy text trước khi viết renderer:

- Mặc định outline các glyph/nhãn builtin và chữ số; không phụ thuộc font hệ thống.
- Title/desc/metadata vẫn là text XML để truy xuất nội dung.
- Text tùy ý và locale `vi/en` phải có font được phép nhúng hoặc pipeline outline có đủ dấu tiếng Việt. Nếu thiếu glyph, báo lỗi/fallback rõ; không cam kết xuất tùy ý mọi ngôn ngữ ngay MVP.
- Màu/nét/gradient/style nằm trong SVG; không để `var(--...)`, stylesheet hoặc font URL ngoài. SVG có thể tham chiếu tài nguyên ngoài theo [đặc tả W3C](https://www.w3.org/TR/SVG11/struct.html), nên xuất XML thành công chưa đủ chứng minh file tự chứa.
- Namespace IDs bằng prefix ổn định; caller dùng `instanceId` riêng khi nhúng nhiều chart vào cùng DOM. Cùng input/options/version phải cho cùng bytes, không chèn timestamp hiện tại hoặc random IDs.

Custom pack dùng geometry có cấu trúc, cùng ý nghĩa với module zodiac hiện có. Override một cung thì các cung còn lại dùng default. Validate IDs/viewBox, giới hạn số nodes/độ dài path, finite coordinates và thuộc tính cho phép; XML escape label. Không nhận raw SVG, scripts, event handlers, external href hoặc CSS tùy ý. Custom chỉ thay hình thức, không thay thứ tự cung hoặc dữ liệu thiên văn.

## 5. Quy tắc layout cần khóa bằng test

1. Longitude chuẩn hóa và wrap 0°/360° đúng; chiều vòng và vị trí ASC có định nghĩa cố định.
2. Tách vị trí marker thật với nhãn đã giãn; không dịch hành tinh để giải quyết va chạm chữ.
3. Giải va chạm theo kích thước glyph/label thực, gồm cụm qua ranh 359°/0° và nhiều layers; phát diagnostics nếu không đủ không gian.
4. Định dạng độ/phút có chính sách carry qua cung: không hiện `30° 00′` với tên cung cũ. Không sửa longitude nguồn để làm tròn nhãn.
5. Dùng aspects SDK trả về, kể cả custom/disabled rules. Không áp lại orb cố định của renderer cũ.
6. Mọi giới hạn nét/lớp/điểm phải có coverage: available, rendered, excluded và lý do. Không âm thầm cắt 40 nét như một mặc định xuất bản.
7. Chart lỗi/thiếu dữ liệu không được biến thành chart mẫu; composite không được gán birth time hoặc retrograde giả.

## 6. Thứ tự triển khai và cửa nghiệm thu

| Mốc | Công việc | Kết quả cụ thể để nghiệm thu |
| --- | --- | --- |
| R0: baseline | Sửa formatter ở ví dụ UI, đồng bộ artifact/version và docs; lưu baseline tests | Không còn nhãn 30° trong cung cũ; engineVersion/package version của artifact dùng thử khớp; regression không đổi dữ liệu |
| S0: contract | Chốt ChartScene, options, errors, coverage, asset/font policy và schema renderer | Fixture hợp lệ/không hợp lệ; thiết kế API dùng được từ Rust, C và Node; không lệ thuộc DOM |
| S1: basic natal | Pure renderer, vector packs, layout, SVG standalone; nối local Payload harness ngay | Xuất chart thật từ SDK; mở độc lập với mạng tắt; hai theme; cusp/angles/aspects khớp payload |
| S2: advanced natal | Độ/phút/Rx, filters, custom pack, legends và diagnostics | Partial override fallback đúng; va chạm/0° wrap được kiểm tra; coverage giải thích được phần không vẽ |
| S3: nhiều chart | Couple A/B, composite C, transit snapshot theo thứ tự | IDs/layers không lẫn A/B/C; hướng overlay/aspect đúng; không chế thêm motion cho C |
| S4: bindings | C exports, Node/Python/.NET wrappers, declarations, examples và package assets | Fresh install các nền tảng đã công bố; normalized SVG/diagnostics parity; ownership/error paths đạt |
| S5: Payload MVP | Nhập UTC/location hoặc dữ liệu sinh đã chuyển múi giờ; preview, basic/advanced, theme, SVG download | Gọi SDK thật qua host; hiển thị errors/warnings; file tải xuống đúng preview; xử lý tác vụ/queue rõ ràng |
| S6: release | Alpha, migration docs, supported-platform matrix; sau đó mới stable | Cài từ artifact cuối, tests đạt, fonts/assets/notices đủ, không phát hành platform chưa chạy thử |

S4 bắt đầu song song về contract với S0/S1; không đợi hoàn thành mọi kiểu chart mới thử bindings. S5 dùng harness sớm từ S1, hoàn thiện luồng sản phẩm sau S2. Nếu cần ra bản đầu nhanh, chốt phạm vi **S0 + S1 + S2 + Node + Payload natal**; S3 và các nền tảng mới là các đợt sau.

Chưa chốt ngày phát hành. `1.0.0` chỉ là tên phiên bản gợi ý sau khi scope được khóa, chưa phải bản tồn tại trên registry.

## 7. Ma trận kiểm chứng

| Nhóm | Trường hợp bắt buộc |
| --- | --- |
| Dữ liệu | 0/360°, sát ranh cung/nhà, ngày nhuận, đổi ngày UTC, nhiều năm sinh, hai house systems, ASC/MC khác cusp |
| Hình học | Cụm hành tinh dày, cụm qua 0°, labels dài, viewBox nhỏ/lớn, glyph bounds, leader lines, hai vòng A/B |
| Contract | Payload lỗi/phiên bản không hỗ trợ, thiếu dữ liệu, custom/disabled aspects, composite thiếu motion, option sai |
| Xuất file | XML parse được, không external resource, ID không đụng khi nhúng nhiều chart, deterministic bytes, không cắt nhãn |
| Custom | Bộ đầy đủ, partial fallback, viewBox khác, geometry không hợp lệ, input quá lớn, text escape và không mutation |
| Đối chứng | Tọa độ SVG truy ngược đúng longitude/cusp; visual goldens ở nhiều size; rasterize ít nhất hai trình render độc lập |
| Bindings | Cùng payload/options cho cùng normalized SVG và diagnostics; free/memory/error paths; final-package install |
| Hiệu năng | Đo cold/warm latency, serialize time, RSS, SVG bytes và nhiều chart liên tiếp trước khi đặt ngân sách p95/size |

Chart web cũ là tham chiếu trải nghiệm, không phải chân lý thiên văn. Khi so số phải khóa UTC/TT, location, provider, house system và reference frame. [JPL Horizons manual](https://ssd.jpl.nasa.gov/horizons/manual.html) là nguồn đối chiếu về frames/timescales; dùng cấu hình mặc định khác nhau có thể tạo sai lệch không đến từ renderer. Thêm các trường hợp nhiều năm vào bộ reference có provenance và tolerance, không chỉ chụp ảnh giống nhau.

## 8. Quyết định đề nghị

Làm **SVG natal basic/advanced + local Payload integration trước**, đồng thời xử lý các lỗi nền có bằng chứng. Giữ engine đang hoạt động; chưa cần viết lại SDK. Mở rộng kiểu chart và phát hành Python/.NET/nền tảng mới theo từng đợt đã kiểm chứng. Khả năng tính toán mới như sidereal, nodes hoặc returns cần roadmap riêng và nhu cầu sản phẩm rõ ràng.
