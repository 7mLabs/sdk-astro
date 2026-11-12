<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — full logo"></a>

# Quy tắc tính Astro 1.1.2

**Tiếng Việt** · [English](rules.md)

[SDK](../README.md) · [Chức năng và công thức](features-and-formulas.vi.md) · [Quy tắc tính](rules.vi.md) · [Nguồn kiểm chứng](formula-verification.vi.md)

---

Đây là mô tả cấu hình và quy ước **SDK 1.1.2 hiện hành**, không tạo một trường phái hay mã rules mới trong payload. Các version của domain, report, facts, provider và tzdb có ý nghĩa riêng; đọc giá trị thực mà response công bố.

<details open>
<summary><strong>Trên trang này</strong></summary>

- [Phạm vi và mặc định](#defaults)
- [Ngày giờ và tọa độ](#input)
- [Chọn hệ/quy tắc](#policies)
- [Đọc payload đúng phạm vi](#payload)
- [Phân phối và tương thích](#compatibility)

</details>

<a id="defaults"></a>

## Phạm vi và mặc định

| Quy tắc | Cấu hình đang dùng |
| --- | --- |
| Hoàng đạo | Tropical; không có tùy chọn ayanamsa/Sidereal trong birth API. |
| Tọa độ thiên văn | Geocentric, apparent, ecliptic-of-date. Không nhận topocentric birth configuration. |
| Provider | Swiss 2.10.03 đã pin; Moshier cho Sun–Pluto; Chiron tùy chọn có dữ liệu nhúng riêng. |
| Hệ nhà | Placidus mặc định, hoặc `wholeSign` tường minh. |
| Chủ tinh ở domain/reports | `traditional` mặc định, hoặc `modern`; không đổi hoàng đạo hay dignity. |
| Góc chiếu | Natal cơ bản: bộ major qua `aspectRules`. Domain/couple/composite/Davison: extended. Events/forecast: major. |
| Điểm mở rộng | Chỉ thêm khi chọn rõ trong natal/natalDomains; không thay thế 10 thiên thể chính. |
| Composite | `houseMethod: midpoint`, `antipodalPolicy: error`. |
| Davison | `variant: uncorrected`; hệ nhà chart dẫn xuất Placidus. |
| Metadata bổ sung | `includeMetadata: false` mặc định; bật `true` cho các astronomical/query operation hỗ trợ. |

Không có một trường duy nhất “school=modern” đại diện toàn bộ các lựa chọn. Ví dụ traditional rulers + Whole Sign + extended aspects vẫn là cấu hình hợp lệ. Các tên career/love là nhóm chọn dữ liệu của 7mlabs, không phải trường phái hay công thức luận đoán.

<a id="input"></a>

## Ngày giờ và tọa độ

- Birth dùng lịch Gregory, năm **1800–2399**, ngày hợp lệ; hour 0–23, minute 0–59, second hữu hạn trong `[0,60)`, mặc định 0. Không nhận leap second 60.
- Latitude có dấu bắc dương, nghiêm ngặt giữa −90 và +90; longitude đông dương trong −180…180. `(0,0)` hợp lệ. Đây là tọa độ địa lý, khác longitude hoàng đạo `[0,360)`.
- UTC do bên gọi chuẩn bị, hoặc dùng `calculateFromLocal`/`resolveBirthTime`. Mỗi birth không đồng thời có cả UTC và local.
- Local helper cần đúng một trong `timeZone` IANA hoặc `utcOffsetSeconds` nguyên có trị tuyệt đối <86400. Dương nghĩa phía đông UTC. Không suy timezone từ tọa độ hoặc cấu hình máy.
- tzdb nhúng **2026e**. Giờ lặp DST mặc định reject; chọn `earlier`/`later` khi cần. Giờ không tồn tại luôn lỗi, không tự đẩy qua gap. Năm local và UTC sau đổi đều phải trong phạm vi birth.
- Swiss dùng time model đã biên dịch để chuyển UTC/TT/UT1. Trước 1972 có quy ước upstream coi civil input là UT1; tương lai không có live Earth-orientation corrections. Giữ warnings và Julian days trong response.
- `events/forecast.period.utcOffsetMinutes` là **offset cố định −840…840 phút**, mặc định 0. Nó không phải timezone của birth và không tự đổi offset giữa kỳ qua DST. Membership của kỳ dùng `[start,end)`; eclipse xét mốc maximum.

Birth/raw JSON number nguyên được chuẩn hóa chính xác từ decimal/exponent; fractional value, string, boolean, unknown field, trường trùng hoặc kiểu không đúng bị từ chối theo contract từng API. Không dùng số đã làm tròn để “sửa” ngày giờ sai. [Natal](natal.md), [đổi giờ](time-and-svg.md), [period và events](forecast.md).

<a id="policies"></a>

## Chọn hệ và quy tắc rõ ràng

**Nhà:** dùng vị trí giữa cusps theo kinh độ, không phải phép gán nhà 3D. Một lỗi Placidus không tự chuyển sang Whole Sign/Porphyry. Ở geometry, cusps là dữ liệu bên gọi đưa vào; chúng cần đủ 12, đúng thứ tự đi một vòng, không lặp. Khi không có cusps, không giả nhà cho điểm.

**Góc chiếu:** tối đa 16 rules với angle không trùng, góc 0–180°, maxOrb theo validation của API. Không truyền đồng thời preset và rules. Geometry/aspect query dùng số đầu vào; chúng không suy một bộ quy tắc chiêm tinh nếu contract yêu cầu bên gọi chỉ định.

**Domains:** `natalDomains` mặc định 10 nhóm; `couple` mặc định 6 nhóm synastry riêng. `domains: []` chỉ hợp lệ khi có custom profiles không rỗng trong các operation hỗ trợ. Custom selectors chọn houses/bodies/angles, không nhận code hay công thức thực thi từ caller. Số liệu chung và evidence có thể xuất hiện trong nhiều views: không cộng các count của views để suy tổng hay điểm tương hợp.

**Mean/true và chart kind:** mean North Node phải đi với mean South Node; true đi với true. Mean Lilith và osculating Lilith là hai định nghĩa khác nhau. Composite không có chuyển động vật lý; Davison tính lại provider nên có tốc độ vật lý. Không hoán đổi hai loại khi luận giải.

**Rulership và facts:** table modern thay ba outer rulers; dignity vẫn traditional sign-only. Counts, patterns, dispositors và các report selectors là facts/policy công bố rõ, không phải bộ lời giải nghĩa hoặc điểm sức mạnh tổng hợp. [Bảng công thức](features-and-formulas.vi.md).

<a id="payload"></a>

## Đọc payload đúng phạm vi

| Phần | Cần giữ / kiểm tra |
| --- | --- |
| Envelope | `schemaVersion`, `engineVersion`, `warnings`, `errors`; lỗi engine thường có `data: null`. |
| `calculation` | Provider, epoch, hệ nhà, aspect rules và cấu hình có trong operation; bật `includeMetadata` để lấy các khai báo bổ sung được hỗ trợ. |
| Composite/Davison provenance | Phương pháp dẫn xuất, nguồn A/B, variant/chính sách và residual khi áp dụng. |
| Domain/report/facts | IDs/versions, selection reasons, evidence references; chỉ join ID/index trong cùng response. |
| Local conversion | `calculation.timeResolution` hoặc response helper, gồm input civil, timezone/offset và cách resolve. |
| Supplied geometry | `inputProvenance` khi bật metadata: `callerDeclared`/`unspecified`, luôn `verified: false`. |
| SVG/model/context | Giữ metadata nguồn; đọc coverage/omissions khi dữ liệu đã lọc/nén. |

`sourceContext` có thể khai báo tọa độ sidereal/topocentric do caller tính **ở ngoài**; đây không phải hỗ trợ tính sidereal/topocentric birth trong engine. SDK không tự đổi hệ tọa độ, kiểm chứng hoặc hòa giải hai declaration khác nhau. [Contract metadata](payload-metadata.md).

F64 nguồn không phải chuỗi độ-phút hiển thị. Giữ JSON number gốc để tính tiếp; làm tròn là quyết định của UI. Hai cách serialize `0`/`0.0` có thể cùng nghĩa. Node/Python/.NET tiện ích object ném lỗi có code/result, còn JSON thô giữ envelope. Không bỏ qua warnings vì một lần tính trả thành công.

<a id="compatibility"></a>

## Phân phối và tương thích

Bản cập nhật tài liệu giữ **engine/package 1.1.2, envelope 1.0**; không đổi tên hàm, default, schema, wrapper, binary hoặc công thức. Các gói và tag v1.1.2 đã phát hành giữ nguyên; tài liệu trên main có thể được làm rõ sau đó.

Binary hiện có: npm/Python/.NET cho macOS ARM64 và Linux x64; .NET 10, Python ≥3.10, Node ≥18. Linux cần glibc ≥2.38 và libgcc_s.so.1. [Ma trận và cài đặt](../README.md#platforms), [bằng chứng release](release-1.1.2.md).

Browser WASM, Sidereal/ayanamsa, topocentric natal, progression/return charts và targets ngoài ma trận chưa được triển khai. SVG là trình bày, MCP/worker là adapter; không tự mở rộng phạm vi tính toán của core.
