<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs — full logo"></a>

# Nguồn và kiểm chứng công thức Astro 1.1.2

**Tiếng Việt** · [English](formula-verification.md)

[SDK](../README.md) · [Chức năng và công thức](features-and-formulas.vi.md) · [Quy tắc tính](rules.vi.md) · [Nguồn kiểm chứng](formula-verification.vi.md)

---

Trang này giúp phân biệt **quy ước đã công bố**, **khớp triển khai** và **đối chiếu độc lập**. Không dùng một badge hoặc tên provider để thay thế bằng chứng. Tài liệu áp dụng cho SDK 1.1.2; lần làm lại README không phát hành engine mới.

<details open>
<summary><strong>Trên trang này</strong></summary>

- [Nguồn cho từng nhóm phép tính](#sources)
- [Ví dụ số học có thể tái lập](#arithmetic)
- [Bằng chứng đã có](#evidence)
- [Khi đối chiếu với website khác](#comparison)
- [Thay đổi tài liệu và phiên bản](#documentation)

</details>

<a id="sources"></a>

## Nguồn cho từng nhóm phép tính

| Phạm vi | Nguồn quyết định hành vi của bản này | Cách kiểm tra |
| --- | --- | --- |
| Normalize, signed delta, sign, house, orb, applying, harmonic | [astro-core/lib.rs](../neutral-engine/crates/astro-core/src/lib.rs) | Unit tests ngay trong module và fixtures đa binding. |
| Provider, TT/UT1, houses, optional point data | [provider Rust](../neutral-engine/crates/astro-provider-swiss/src/lib.rs), [PROVENANCE](../neutral-engine/crates/astro-provider-swiss/PROVENANCE.md), [natal](../neutral-engine/crates/astro-core/src/natal.rs) | Provider checks, natal cases, dữ liệu/flags và checksum được pin. |
| Mean/true definitions và lựa chọn thêm | [natal.rs](../neutral-engine/crates/astro-core/src/natal.rs) | Kiểm tra antipode South Node, định nghĩa riêng, dữ liệu Chiron. |
| Rulership, dignity, patterns, selectors | [advanced](../neutral-engine/crates/astro-core/src/advanced.rs), [domains](../neutral-engine/crates/astro-core/src/domains.rs), [profiles](profiles.md) | Unit topology/boundary tests; metadata công bố policy. |
| Composite | [composite.rs](../neutral-engine/crates/astro-core/src/composite.rs) | Swap symmetry, midpoint/cusp order, antipodal policies, trường vật lý null. |
| Davison | [davison.rs](../neutral-engine/crates/astro-core/src/davison.rs) | Derived epoch/place, MC residual, swap symmetry, reproduction từ natal. |
| Events và forecast | [events.rs](../neutral-engine/crates/astro-core/src/events.rs), [forecast.rs](../neutral-engine/crates/astro-core/src/forecast.rs) | Root/boundary/repeated-hit tests và mốc Moon phase/eclipse. |
| Đổi giờ | [astro-time](../neutral-engine/crates/astro-time/src/lib.rs) | Fold/gap, offsets, lịch sử, giới hạn năm và dữ liệu tzdb đã pin. |

Nguồn ngoài dùng để hiểu định nghĩa/phương pháp, không để suy rằng mọi chức năng của upstream đều có trong public API:

- [Swiss programming interface](https://www.astro.com/swisseph/swephprg.htm): tài liệu các hàm provider, thang thời gian và hệ nhà; thuật toán thực tế của SDK vẫn được xác định bởi source đã pin và flags đang dùng.
- [Astrodienst — Davison variants](https://www.astro.com/cgi/h.cgi?f=gch&h=gch65&lang=e): phân biệt corrected/uncorrected, mean geography với spherical/reference place. SDK chọn UT1 và thuật toán MC được ghi trong [contract](davison.md).

Không coi chính sách orb, domain selectors hoặc dignity sign-only của SDK là một chuẩn duy nhất được mọi trường phái chấp nhận. Với các chính sách đó, bảng quy tắc và source của dự án là nguồn trực tiếp.

<a id="arithmetic"></a>

## Ví dụ số học có thể tái lập

| Đầu vào | Kết quả mong đợi | Phạm vi |
| --- | --- | --- |
| Longitude −10° | 350°, Pisces 20° | Normalize/sign |
| Longitudes 350°, 10° | Separation 20°; midpoint 0° | Geometry, không đối đỉnh |
| Longitudes 10°, 101°, angle 90° | Separation 91°, orb 1° | Match nếu maxOrb ≥1° |
| Cùng chart: λ1=0, v1=1; λ2=91, v2=0; angle=90 | Applying true, orb 1° | Có tốc độ, instantaneous test |
| λ=350°, factor=5 | λ′=310° | Harmonic, không sky vật lý |
| Davison signed longitudes +179°, −179° | Mean longitude 0° | Arithmetic geography, không spherical midpoint |

Đây là ví dụ công thức, không phải birth record của người dùng. [Request geometry](examples/geometry-request.json) và [response natal thật](examples/natal-response.json) giúp đối chiếu cấu trúc. Midpoint hình học và geographical mean không dùng thay nhau.

<a id="evidence"></a>

## Bằng chứng đã có

1. [Báo cáo phát hành 1.1.2](release-1.1.2.md): 12 candidate checks và phát hành npm qua; Linux/macOS, Python 3.10/3.13, Node 18/24; 339 ca conformance trong các consumer cuối. Có cài mới npm/PyPI/NuGet từ kho công khai và checksum cụ thể.
2. [Báo cáo Davison 64 chart](reviews/2026-11-02-davison/README.md): so với một quy trình lắp ghép Swiss riêng và đối chiếu vị trí/góc bằng Skyfield/JPL. Tách parity cùng provider khỏi đối chiếu provider khác; kết quả chỉ mô tả bộ ca và tolerances trong báo cáo.
3. [Testing](testing.md) và [test nguồn](../neutral-engine/crates/astro-core/tests/): natal, calendars, các biên hình học, concurrency, response schemas, reports và event search. Các fixture Moon/eclipse có nguồn ghi trong test.
4. [Bằng chứng công khai](https://7mlabs.com/astro/sdk/docs/1.1.2/downloads/verification.json): commit/tag, các gói đã kiểm tra và phạm vi public-consumer verification.

Conformance đa binding chứng minh các wrapper nhất quán trên bộ ca; không phải bốn phép tính thiên văn độc lập. Tolerance giải nghiệm số không đồng nghĩa độ chính xác tuyệt đối của time model/ephemeris. Kiểm tra compile từ cùng mã nguồn cũng khác một lần kiểm tra bằng công cụ ngoài.

<a id="comparison"></a>

## Khi đối chiếu với website khác

So sánh trước: ngày Gregory, UTC/offset/IANA và DST choice; tọa độ; tropical hay sidereal; apparent/reference frame; hệ nhà; mean/true points; rulership; aspect angles/orbs; composite hay Davison và variant; time scale và geography convention. Dùng số nguồn thay vì số đã làm tròn trên màn hình.

Nếu còn lệch, lưu request tối thiểu, response có metadata/warnings, phiên bản engine/provider và cấu hình công cụ đối chiếu. Không đổi thuật toán chỉ để khớp một website khi chưa xác định khác biệt quy tắc. Các field geometry `verified: false` là khai báo nguồn của caller, không phải kiểm chứng thiên văn của SDK.

<a id="documentation"></a>

## Thay đổi tài liệu và phiên bản

Lần này bổ sung mô tả/ảnh/ví dụ và sửa câu trạng thái cũ. Source tính toán, binding, schemas, manifests và tag v1.1.2 giữ nguyên. Response mẫu được sinh bằng gói 1.1.2 đã phát hành, không viết tay các tọa độ.

Không tạo tag mới, không upload lại các gói 1.1.2 và không tăng lên 1.1.3. Source ở main có thể có commit tài liệu sau commit tag; đó không phải thay đổi binary của release.
