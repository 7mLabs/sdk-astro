# Trạng thái Astro SDK 1.1.0 — 02/11/2026

Source phát hành là tag `v1.1.0`; npm `@7mlabs/astrology`, PyPI
`sevenmlabs-astrology` và NuGet `SevenMLabs.Astrology` đều đã phát hành 1.1.0.
Mã source và nội dung các gói đã được đối chiếu bằng checksum; cài mới từ cả ba
registry và kiểm tra tương thích 1.0.2 đã đạt. Xem [bằng chứng phát hành](release-1.1.0.md).
ABI, schema, renderer, profile và dữ liệu timezone có vòng phiên bản riêng.
Các release notes, paper, báo cáo và kế hoạch lịch sử giữ nguyên mốc phiên bản cũ.

## Đã có

- Natal, natalDomains, synastry/couple, midpoint composite; Davison uncorrected/corrected.
- Tropical geocentric apparent Sun–Pluto; tính cusps Placidus/Whole Sign, góc và aspects.
- Chiron, true/mean North/South Nodes, mean/osculating Lilith khi opt-in trong natal/natalDomains.
- Event search và forecast cá nhân, transit-natal contacts, ingress/stations/phases và global eclipse maxima.
- Profiles/bằng chứng có cấu trúc, geometry/query helpers, nén context; đổi giờ IANA và SVG/model.

## Chưa có trong API phát hành

| Nhóm | Phạm vi còn chưa triển khai/phát hành |
|---|---|
| Returns và tiến triển | Operation/lá số chuyên dụng cho solar/lunar/planetary returns, secondary progressions và solar arc. Bộ tìm aspect hiện có có thể cung cấp một số thời điểm liên quan; đó chưa phải một module returns hoàn chỉnh. |
| Hệ tọa độ | Tự tính sidereal/ayanamsha và topocentric; hiện birth calculations dùng tropical/geocentric. |
| Hệ nhà | Tự tính thêm Koch, Equal, Regiomontanus, Campanus…; birth API hiện chọn Placidus hoặc Whole Sign. Geometry vẫn nhận cusps do caller cung cấp. |
| Biến thể chart | Davison spherical/shortest-arc geographical midpoint hoặc reference-place; reference-place/progressed composite; forecast chuyên dụng cho các chart này. |
| Điểm và thiên thể | Tự tính thêm fixed stars/asteroids ngoài phạm vi đã công bố, Vertex/Lots. Bảy điểm opt-in hiện có chưa mở rộng sang couple/composite/Davison/forecast. Geometry vẫn xử lý được vị trí hợp lệ do caller cung cấp. |
| Event/forecast nâng cao | Thời lượng/orb-entry-exit windows, local eclipse visibility, horizon angles/nhà transit theo địa điểm hiện tại. |
| Browser | Package/provider WASM chạy tính toán trực tiếp trong browser offline. Hiện UI browser dùng host/backend có native SDK. |
| Nền tảng | Windows, Intel Mac, Linux ARM64, Alpine/musl. Python public wheel hiện chỉ macOS ARM64; Linux Python wheel portable chưa phát hành. npm/NuGet hỗ trợ macOS ARM64 và Linux x64 trong điều kiện runtime/libc đã công bố. |
| Công cụ tích hợp | CLI/MCP sản phẩm đóng gói/cài độc lập và tự động tích hợp host; hiện có source examples/adapters. Worker mẫu chưa có pool hoặc cancellation từng job. |

Phần luận giải, narrative và điểm tương hợp được thiết kế ở lớp ứng dụng.
Danh sách trên mô tả API SDK; không phải trạng thái hoàn thành của Payload Web
hoặc Compass App, hai sản phẩm khác trong dự án Astro.

## Cách đọc các kế hoạch cũ

[phases.md](phases.md) và [svg-rendering-plan.md](svg-rendering-plan.md) lưu tiến
trình/kế hoạch ở nhiều mốc. Câu “chưa phát hành” trong một mục lịch sử mô tả mốc
đó. Dùng trang này, contract API và release notes hiện hành để xác định phạm vi
1.1.0; không suy trạng thái hiện tại từ một đoạn kế hoạch alpha cũ.
