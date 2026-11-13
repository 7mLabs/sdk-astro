<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — full logo"></a>

# Chức năng và công thức Astro 1.1.3

**Tiếng Việt** · [English](features-and-formulas.md)

[SDK](../README.md) · [Chức năng và công thức](features-and-formulas.vi.md) · [Quy tắc tính](rules.vi.md) · [Nguồn kiểm chứng](formula-verification.vi.md)

---

Tài liệu mô tả **cách Astro SDK 1.1.3 đang tính**, không thêm phép tính hoặc trường dữ liệu mới. Các đường dẫn bên dưới tính từ envelope thành công; đường dẫn tương đối trong `data` thay đổi theo operation. Tên API/JSON giữ nguyên tiếng Anh.

1.1.3 cập nhật bộ tệp giấy phép trong gói cài đặt; các phép tính, API và schema ở đây giữ nguyên. MIT bổ sung chỉ dành cho phần đóng góp thuộc 7mlabs được kiểm kê; toàn gói native công khai vẫn là AGPL-3.0-only. [Hồ sơ phát hành](release-1.1.3.md) · [Phạm vi giấy phép](license.md).

<details open>
<summary><strong>Trên trang này</strong></summary>

- [Chọn phạm vi](#operations)
- [Ký hiệu và hình học](#geometry)
- [Natal, hệ nhà và điểm mở rộng](#natal)
- [Góc chiếu](#aspects)
- [Chủ tinh và facts](#facts)
- [Couple, composite và Davison](#relationships)
- [Sự kiện và forecast](#events)
- [SVG, model và context](#presentation)
- [Ví dụ và đối chiếu mã nguồn](#examples)

</details>

<a id="operations"></a>

## Chọn phạm vi tính

| Operation / dịch vụ | Đầu vào chính | Kết quả |
| --- | --- | --- |
| `natal` | `utc`, `location`, hệ nhà tùy chọn | `data.placements`, `angles`, `houses`, `aspects`; có thể thêm `additionalPoints`. |
| `natalDomains` | Birth như natal; domains/profiles tùy chọn | `data.natal`, `context`, `domains`, `customDomains`, `profileCatalog`. |
| `couple` | `personA`, `personB` | Hai natal, quan hệ chéo, overlays, 6 views; `data.composite` khi bật rõ. |
| `composite` | Hai birth inputs và chính sách midpoint | `data.chart`, context/reports và provenance của chart C tượng trưng. |
| `davison` | Hai birth inputs; variant tùy chọn | `data.natal` tại thời gian/địa điểm dẫn xuất, context/reports, `data.provenance`. |
| `events` | `period`, bộ lọc bodies/event types tùy chọn | Sự kiện và snapshot trong kỳ; xem [contract](forecast.md). |
| `forecast` | `birth`, `period` | Natal cố định, snapshot transit, exact events, calendar và views theo lĩnh vực. |
| `chart`, `harmonic`, `synastry` | `positions`; vị trí B/cusps/factor tùy operation | Các quan hệ hình học; không tự tính bầu trời từ ngày sinh. |
| `query` | Helper-specific input | Tính góc hoặc chọn/kiểm tra facts; [8 helpers](query.md). |
| Time, SVG, compression | Yêu cầu giờ địa phương hoặc payload đã có | Dịch vụ riêng; không phải loại lá số mới. |

<a id="geometry"></a>

## Ký hiệu và phép tính hình học

`λ` là kinh độ hoàng đạo theo độ; `v` là tốc độ kinh độ theo độ/ngày; `N(x)` chuẩn hóa về `[0,360)`; `D(a,b)` là hiệu có dấu trong `[-180,180)`.

```text
N(x) = x mod 360, chọn phần dư không âm
D(a,b) = N(N(a) - N(b) + 180) - 180
separation(a,b) = abs(D(a,b))
signIndex = floor(N(λ) / 30)          // 0 = Aries, ... 11 = Pisces
degreeInSign = N(λ) mod 30
isRetrograde = (v < 0)               // null nếu không có tốc độ vật lý
```

Mã dùng `f64`, xử lý cả làm tròn gần 0/360 và điểm cắt ±180. Tên cung không phải một phép luận giải. Ví dụ `N(-10)=350`, `separation(350,10)=20`, `λ=280°` thuộc Capricorn, độ trong cung bằng `10°`.

Harmonic: `λ' = N(λ × factor)`, `v' = v × factor` khi có speed; factor nguyên 1–360. Không giữ nhà natal trên tọa độ đã biến đổi. Ví dụ `350 × 5 → 310°`. [Mã hình học](../neutral-engine/crates/astro-core/src/lib.rs).

<a id="natal"></a>

## Natal, hệ nhà và điểm mở rộng

Vị trí Sun–Pluto không được tính bằng một công thức tuyến tính trong SDK. Provider gọi Swiss Ephemeris với mode Moshier/speed, chuyển UTC sang TT/UT1 rồi tính kinh độ, vĩ độ, khoảng cách AU và tốc độ tại epoch thực. Hệ tọa độ cố định tropical/geocentric/apparent/ecliptic-of-date. [Mã provider](../neutral-engine/crates/astro-provider-swiss/src/lib.rs), [nguồn upstream đã pin](../neutral-engine/crates/astro-provider-swiss/PROVENANCE.md).

| Dữ liệu | Cách xác định | Vị trí trong natal cơ bản |
| --- | --- | --- |
| Thiên thể | Provider cho tọa độ; SDK suy sign, degree, retrograde và house | `data.placements[]` |
| ASC, MC | Provider tính theo UT1 và tọa độ địa lý | `data.angles[]` |
| DSC, IC | Đối điểm tương ứng của ASC, MC | `data.angles[]` |
| Cusp Placidus | Swiss giải hệ nhà Placidus; không phải chia đều 30° | `data.houseCusps`, `houses[]` |
| Cusp Whole Sign | Cung chứa ASC là nhà 1; cusp tiếp theo cách 30° | Cùng các trường house |
| Nhà chứa một điểm | Nhà i thỏa `N(λ - cusp[i]) < N(cusp[next] - cusp[i])` | `placements[].house` |
| Điểm mở rộng | Chọn rõ định nghĩa; giữ tách khỏi 10 thiên thể chính | `data.additionalPoints[]` |

Gán nhà dùng **kinh độ hoàng đạo giữa hai cusps**, gồm đầu cung nhà và loại đầu cung nhà kế; không dùng house position 3D theo vĩ độ thiên thể. Cực địa lý bị từ chối; Placidus lỗi ở vĩ độ cao sẽ báo lỗi, không tự đổi hệ nhà. [Quy tắc và miền đầu vào](rules.vi.md).

`additionalPoints: "standard"` chọn `chiron`, `trueNorthNode`, `meanLilith`; `"all"` chọn cả bảy: Chiron, mean/true north và south nodes, mean/true Lilith. South Node là `N(λNorth + 180)` của đúng định nghĩa North Node tương ứng. Chiron dùng tệp `seas_18.se1` nhúng sẵn; Lilith ở đây là **viễn điểm quỹ đạo Mặt Trăng** mean/osculating, không phải tiểu hành tinh tên Lilith. Các điểm này chỉ bật ở natal/natalDomains, không âm thầm thêm vào phân bố/chủ tinh của 10 thiên thể. [Mã và định nghĩa](../neutral-engine/crates/astro-core/src/natal.rs).

<a id="aspects"></a>

## Góc chiếu và trạng thái đang tiến/tách

Với angle `α`, sai số `orb = abs(separation - α)`. Một góc khớp khi `orb <= maxOrb`; ranh giới được tính trong. Mỗi rule khớp được giữ; không chọn duy nhất một góc nếu các orb chồng nhau.

| Preset | Angle → maxOrb, đơn vị độ |
| --- | --- |
| `major` / mặc định natal cơ bản | 0→8, 60→6, 90→8, 120→8, 180→8 |
| `extended` | Như major; thêm 30→2, 45→2, 72→2, 135→2, 144→2, 150→3 |

Natal cơ bản nhận `aspectRules` chứ không nhận `aspectPreset`. Domain/couple/composite/Davison mặc định extended; events/forecast mặc định major. Preset và explicit rules không truyền đồng thời. `aspectRules: []` tắt matching, không có nghĩa mọi quan hệ hình học đều biến mất.

Với hai điểm **cùng chart vật lý** có tốc độ, khi orb lớn hơn `1e-12°`:

```text
δ = D(λ1, λ2)
distanceRate = (v1 - v2) × sign(δ)
applying = ((abs(δ) - α) × distanceRate < 0)
```

Exact trong tolerance, thiếu speed, synastry so hai chart hoặc composite tượng trưng trả `applying: null` theo phạm vi tương ứng. Ví dụ separation `91°`, angle `90°` cho orb `1°`; nếu separation đang giảm thì applying. Đây là kiểm tra theo tốc độ tức thời, không phải một lần tìm nghiệm exact trong tương lai.

<a id="facts"></a>

## Chủ tinh, facts và lựa chọn dữ liệu

| Fact | Phép tính / quy tắc |
| --- | --- |
| Phân bố nguyên tố/tính chất/polarity | Đếm 10 thiên thể theo cung; không tự gán trọng số hoặc điểm tính cách. |
| Chủ tinh traditional | Aries→Mars, Taurus→Venus, Gemini→Mercury, Cancer→Moon, Leo→Sun, Virgo→Mercury, Libra→Venus, Scorpio→Mars, Sagittarius→Jupiter, Capricorn/Aquarius→Saturn, Pisces→Jupiter. |
| Chủ tinh modern | Đổi Scorpio→Pluto, Aquarius→Uranus, Pisces→Neptune; các cung khác như traditional. |
| Dispositor chain | Từ thiên thể → chủ tinh cung nó đang ở; lặp đến tự chủ hoặc một chu trình được nhận diện. |
| Mutual reception | Hai thiên thể nằm trong cung do đối phương làm chủ theo bảng đang chọn. |
| Dignity | Domicile/detriment/exaltation/fall theo bảng **traditional sign-only** của 7 thiên thể cổ điển; outer planets không được gán giá trị giả. |
| Solar condition | Separation với Sun: cazimi ≤17′; combust đến 8.5°; underSunbeams đến 17°; ngoài ra free, với tolerance biên `1e-12°`. Sun tự nó là notApplicable. |
| Aspect patterns | Topology của các matched body-to-body aspects theo angle đã cấu hình; không tính một pattern từ hình vẽ gần giống. |
| Domain/custom profile | Chọn houses/bodies/angles, mở rộng theo occupants/rulers và giữ nguồn; không tính lại vị trí cho từng domain. |

Các quy tắc này không phải toàn bộ chiêm tinh truyền thống hoặc hiện đại. Chọn modern rulers không đổi dignity, không bổ sung triplicity/term/face hoặc tổng điểm sức mạnh. [Mã facts](../neutral-engine/crates/astro-core/src/advanced.rs), [mã domains](../neutral-engine/crates/astro-core/src/domains.rs), [catalog và selectors](profiles.md), [report/evidence](individual-reports.md).

<a id="relationships"></a>

## Couple, composite và Davison

**Couple/synastry:** tính hai natal riêng, lấy cặp chéo A–B để tính separation/orb. House overlay dùng kinh độ của điểm bên A trong cusps B và ngược lại. So sánh cross-chart không gán applying từ hai tốc độ natal. `couple` có 6 nhóm quan hệ, khác catalog 10 nhóm cá nhân/composite/Davison. [Contract](couple.md).

**Composite:** với hai kinh độ cùng loại đã chuẩn hóa, dùng:

```text
lo = min(λA, λB); hi = max(λA, λB)
mid = N(lo + (hi-lo)/2 + (180 nếu hi-lo > 180, ngược lại 0))
```

Hai nguồn đối đỉnh trong `1e-10°` có hai trung điểm: mặc định báo lỗi; `antipodalPolicy: "lowerLongitude"` chọn trung điểm có kinh độ chuẩn hóa nhỏ hơn. `350°/10° → 0°`, không phải 180°. `houseMethod: "midpoint"` dùng cusp midpoint; `"wholeSignFromMidpointAscendant"` dựng Whole Sign từ midpoint ASC. Cusp sai thứ tự trả lỗi. C là chart tượng trưng nên **không có UTC/JD vật lý; speed, retrograde và applying không áp dụng**. [Contract](composite.md), [mã nguồn](../neutral-engine/crates/astro-core/src/composite.rs).

**Davison uncorrected:** chuyển hai thời điểm sang Julian UT1 rồi lấy trung bình; latitude và signed longitude lấy trung bình số học. Ví dụ `+179°` và `−179°` cho **0°**, không phải điểm gần đường đổi ngày. Tính lại bầu trời tại thời điểm/địa điểm dẫn xuất.

**Davison corrected:** giữ địa điểm trung bình; đổi thời điểm để MC của chart dẫn xuất khớp shortest-arc midpoint của hai MC natal. Tìm nghiệm tối đa nửa ngày Mặt Trời trung bình, 48 lần chia đôi, residual MC tối đa `1e-6°`; MC nguồn đối đỉnh hoặc không hội tụ báo lỗi. Sau đó **tính lại toàn bộ** vị trí, nhà, góc và tốc độ. Không lấy trung bình vị trí hành tinh như composite.

Variant mặc định uncorrected, hệ nhà dẫn xuất mặc định Placidus. Mean geography không phải spherical/reference-place Davison. [Contract và provenance](davison.md), [mã nguồn](../neutral-engine/crates/astro-core/src/davison.rs).

<a id="events"></a>

## Sự kiện và forecast

| Loại | Điều kiện tìm |
| --- | --- |
| Ingress | Kinh độ vượt biên cung bội của 30°, giữ cả lần đi lùi. |
| Station | Tốc độ kinh độ đổi dấu qua 0. |
| Lunar phase | `N(λMoon - λSun)` đạt 0/90/180/270°. |
| Planetary aspect | Hiệu kinh độ hai thiên thể quá cảnh đạt angle/branch đã cấu hình. |
| Natal transit | Thiên thể đang di chuyển đạt góc với một điểm natal **cố định**. |
| Eclipse | Tìm cực đại toàn cầu bằng hàm eclipse của Swiss; không suy chỉ từ new/full Moon. |

Scanner dùng grid 6 giờ, xét cực trị relative velocity rồi refine roots bằng provider thật. Angular residual ≤`1e-6°`, bracket thời gian ≤`0.25 s`; station residual ≤`1e-8°/day`. Đây là tolerance của phép giải số, **không** phải chứng nhận sai số so với bầu trời thật. Eclipse toàn cầu không chứng minh nhìn thấy từ tọa độ người dùng.

`period.utcOffsetMinutes` là offset cố định, không tự đổi theo IANA/DST của birth. Highlight/calendar/domain contexts là chính sách chọn và tổ chức sự kiện, không phải lời tiên đoán. [Contract và giới hạn](forecast.md), [scanner](../neutral-engine/crates/astro-core/src/events.rs), [forecast](../neutral-engine/crates/astro-core/src/forecast.rs).

<a id="presentation"></a>

## SVG, model và context

Renderer dùng payload đã tính, không gọi Swiss để “sửa” tọa độ. Các tọa độ viewBox, màu, lọc đường và chọn nhà thuộc trình bày; longitude nguồn được giữ trong model. [Time/SVG](time-and-svg.md).

`compressPayload` xử lý dữ liệu sau tính toán. Chế độ lossless compact có thể giải lại; lựa chọn domain/section hoặc budget có thể bỏ dữ liệu và phải đọc `coverage`/omissions. Không xem mọi dạng nén là lossless. [Contract nén](payload-compression.md).

<a id="examples"></a>

## Ví dụ và đối chiếu

- [Request natal](examples/natal-request.json), [response thật 1.1.3](examples/natal-response.json), [request hình học](examples/geometry-request.json).
- [Quy tắc và cấu hình mặc định](rules.vi.md).
- [Nguồn, loại kiểm chứng và giới hạn kết luận](formula-verification.vi.md).

Metadata không tự bảo đảm AI luận đúng. Bên tích hợp cần giữ `calculation`, provenance, cấu hình và cảnh báo cùng dữ liệu, và chọn bộ kiến thức phù hợp.
