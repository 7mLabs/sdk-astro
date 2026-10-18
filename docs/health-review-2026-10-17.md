# Astro SDK: đánh giá chất lượng và nhu cầu nâng cấp

Ngày: **17/10/2026**. Source `0.10.1`, commit `cbfadc1`. Đọc source core/provider/FFI, bindings, ví dụ UI/host, schemas, conformance, scripts, CI và docs. Đây là đánh giá kỹ thuật và kiểm chứng có giới hạn, không phải chứng nhận toàn bộ độ chính xác hoặc một cuộc security audit chuyên sâu.

Kết luận: **cần nâng cấp có trọng tâm, không cần viết lại SDK**. Core đã có nền kiểm thử và contract đáng giữ. Khoảng trống lớn để phục vụ Payload là renderer SVG tự chứa, đối chứng rộng hơn, chi phí payload và API tác vụ tương tác. Kế hoạch triển khai: [SVG rendering plan](svg-rendering-plan.md).

Đây là snapshot đánh giá **baseline 0.10.1 trước implementation**. Sau khi người dùng chốt phương án, candidate [0.11 local time và SVG](time-and-svg.md) đã triển khai một phần các hạng mục; không xem những mục “chưa có” bên dưới là trạng thái candidate mới.

## 1. Kết quả đã chạy trong lượt này

Máy macOS ARM64, Node `v24.19.0`; cargo dùng dependency lock/offline. Source checkout sạch trước đánh giá. Không sửa logic engine/renderer/bindings hoặc phát hành package trong lượt này.

| Kiểm tra | Kết quả | Ý nghĩa và giới hạn |
| --- | --- | --- |
| Rust workspace tests | **108 pass** | 81 core, 9 composite contract, 9 couple contract, 6 forecast contract, 1 FFI, 1 provider, 1 cold initialization |
| `cargo fmt --check` | Pass | Format source hiện tại |
| Clippy all targets, `-D warnings` | Pass | Không có cảnh báo theo toolchain đang chạy; không thay thế runtime tests |
| Repository metadata/docs check | Pass, 325 local links | Không chạy `--artifacts`; công cụ vẫn nêu các release blockers PyPI/NuGet, wheels/RID và Windows |
| Fresh Rust CLI vs public npm 0.10.1 | **339/339 full JSON parity** | So toàn bộ JSON đã parse và trạng thái lỗi fixture; cùng source family, không phải hai engine thiên văn độc lập |
| Natal synthetic nhiều năm | **190/190 structural checks** | 19 năm × 5 vĩ độ × 2 house systems; không phải đối chứng vị trí thiên thể độc lập |
| UI degree formatter | **Tái hiện lỗi** | `degreeInSign(29.9999)` trả `30° 00′` trong khi caller vẫn có thể dùng sign cũ |

Năm synthetic: 1800, 1801, 1850, 1899, 1900, 1950, 1969, 1972, 1999, 2000, 2020, 2026, 2050, 2099, 2100, 2200, 2300, 2398, 2399. Mỗi ca dùng 15/6 12:00 UTC, latitude −65/−45/0/45/65 và longitude tổng hợp, với Placidus/Whole Sign. Kiểm tra 10 bodies, 12 houses, 4 angles, body/cusp longitude hữu hạn trong [0,360), house assignment 1–12. Không dùng dữ liệu sinh cá nhân.

Quan sát từng example bằng `calculateJson` một lần:

| Example | JSON bytes | Thời gian raw call một lần (ms) |
| --- | ---: | ---: |
| natal | 7,213 | 0.14 |
| natal-domains | 2,145,132 | 89.90 |
| couple | 2,128,806 | 52.95 |
| composite | 2,223,299 | 97.10 |
| forecast-day | 307,833 | 14.28 |
| forecast-month | 4,322,590 | 172.68 |
| forecast-year | 1,838,985 | 248.63 |

Đây là **quan sát cục bộ, không phải benchmark/p95/SLA**; không bao gồm parse JSON của caller, network hoặc render. Các fixtures có phạm vi/selection khác nhau, nên không suy ra forecast-year luôn nhẹ hơn forecast-month. Dữ liệu đo máy đọc được: [review observations](reviews/2026-10-17-observations.json).

## 2. Điểm mạnh đã có

- 10 operations có contract JSON chung; Node có TypeScript declarations tương đối đầy đủ. Python/.NET/Rust/C dùng cùng native engine.
- UTC/location validation, metadata, warnings/errors và các giới hạn provider được thể hiện; không âm thầm đổi hệ nhà để làm request thành công.
- Có kiểm tra invariants, evidence references, custom/disabled aspects, composite và forecast; có nguồn reference Swiss và độc lập Skyfield/JPL.
- Event scans có budgets, giới hạn refinement và không biến budget exhaustion thành thành công một phần.
- Native ownership qua `astro_free_string`; schemas, package checks, checksum/provenance và final npm consumer gates đã có.
- Local HTTP sample đã có readiness/status và giới hạn in-flight mặc định 8 (cho phép 1–32), trả `429 RUNTIME_BUSY`. Không kết luận toàn bộ hệ thống queue không giới hạn.

## 3. Các việc ưu tiên

P1 = nên làm trước SVG/Payload release tiếp theo. P2 = phát triển sau baseline, trước khi mở rộng phân phối đáng kể. Đây là mức ưu tiên sản phẩm/kỹ thuật, không phải mức độ lỗ hổng bảo mật.

### P1 — Xuất SVG vẫn chưa là tính năng SDK hoàn chỉnh

**Bằng chứng:** [wheel.mjs](../examples/frontend/ui/wheel.mjs) tạo DOM qua `document.createElementNS`, dùng CSS variables/classes ngoài SVG và glyph Unicode từ [data.mjs](../examples/frontend/ui/data.mjs). Không có public `renderChartSvg` standalone trong SDK hiện hành.

**Ảnh hưởng:** lấy SVG trong UI chưa đảm bảo mở độc lập, font nhất quán, in ấn/export hoặc chạy từ Python/.NET. Bộ 12 zodiac brand đã có ở workspace, nhưng cần bundle vào renderer và bổ sung glyph hành tinh/font policy.

**Việc cần làm:** pure render module + validated adapter + layout + self-contained assets/text + diagnostics. Giữ renderer tách phép tính; không bê DOM implementation sang mọi binding. Gate theo [kế hoạch SVG](svg-rendering-plan.md).

### P1 — Lỗi nhãn độ ở sát ranh cung

**Bằng chứng:** [data.mjs](../examples/frontend/ui/data.mjs), hàm `degreeInSign`, làm tròn tổng phút trước khi tách độ/phút. `29.9999` và `29.999` cho `30° 00′`.

**Ảnh hưởng:** nhãn có thể nói 30° của cung cũ; đây là lỗi presentation, không phải bằng chứng core tính sai longitude.

**Việc cần làm:** formatter nhận longitude đầy đủ và carry sign khi làm tròn, hoặc policy truncate công bố rõ. Test 0°, 29°59′ gần carry, 359°59′ và các giá trị ngay hai phía ranh; áp dụng cùng policy ở SVG và bảng UI.

### P1 — Đối chứng thiên văn chưa phủ rộng phạm vi công bố

**Bằng chứng:** [natal-references.json](../tests/conformance/natal-references.json) có 4 natal cases (1950, 2000, 2020; chỉ một ca Whole Sign). Có baseline Swiss/Moshier và độc lập Skyfield 1.49/JPL DE421, cùng TT và apparent geocentric ecliptic-of-date. Temporal references tập trung 2026: 50 Moon phases và 4 eclipses.

**Ảnh hưởng:** contract/parity/invariants tốt vẫn chưa chứng minh mọi ngày trong 1800–2399 và mọi location có cùng độ chính xác. 190 ca synthetic mới chỉ bổ sung structural coverage.

**Việc cần làm:** reference grid phân tầng theo năm, mùa, leap/calendar boundaries, hai phía 0°, longitude và vĩ độ cao; thêm temporal references nhiều năm. Lưu provider/data hash, time/frame policy và tolerance từng nhóm. [JPL DE421 không phủ toàn dải 1800–2399](https://ssd.jpl.nasa.gov/planets/eph_export.html): muốn kiểm chứng ngoài coverage phải chọn ephemeris độc lập có phạm vi phù hợp, không extrapolate. So web cũ chỉ sau khi khóa cùng inputs/rules. [JPL Horizons manual](https://ssd.jpl.nasa.gov/horizons/manual.html) mô tả vì sao timescale/reference frame phải được đồng bộ.

### P1 — Artifact trong checkout và tài liệu chưa đồng bộ version

**Bằng chứng:** manifests/source ở 0.10.1 nhưng native addon hiện có tại `bindings/node/native/darwin-arm64/astro.node` báo engineVersion 0.10.0. Nhiều docs còn nêu 0.10.0 hoặc trạng thái planned của các mẫu đã tồn tại. **Public npm 0.10.1 được dùng kiểm chứng trả version đúng**; không kết luận package public bị sai.

**Ảnh hưởng:** thử từ source checkout có thể vô tình dùng binary cũ, khiến review/web metadata và kết quả khó truy nguyên.

**Việc cần làm:** clean build, kiểm tra version/capability ở artifact thực và host handshake; build fingerprint/checksum manifest; thống nhất docs hiện hành và tách lịch sử. `check-repository.py` metadata-only không đủ thay cho kiểm tra artifact.

### P1 cho Payload — Chi phí payload cần được đo và giảm đúng chỗ

**Bằng chứng:** các example full reports ở bảng trên khoảng 2.1–2.2 MB; forecast-month khoảng 4.32 MB. `query` hiện vẫn có đường tính natal đầy đủ rồi selection; không mặc nhiên tiết kiệm CPU tương ứng số trường trả về.

**Ảnh hưởng:** parse/serialize, memory, network và cache sẽ đáng kể khi dùng tương tác hoặc nhiều chart. Payload nặng không chứng minh engine chậm trong mọi use case.

**Việc cần làm:** đo cold/warm p50/p95, RSS, serialization/parse và output size theo profile trước. Thiết kế projection/lean mode tương thích ngược, chỉ bỏ phần lặp khi vẫn giữ evidence/reference closure; dùng full mode khi thực sự cần. Compression hiện có là công cụ lựa chọn, không thay benchmark; estimated tokens không phải số token của tokenizer thực.

### P1 cho tác vụ dài / P2 cho natal — Async và cancellation ở cấp SDK

**Bằng chứng:** Node/Python/.NET chủ yếu là synchronous APIs. [bridge.cjs](../examples/frontend/bridge.cjs) đưa công việc vào một worker; [local-http.cjs](../examples/frontend/local-http.cjs) đã chặn số request như trên. Chưa có packaged async API với queue policy, deadline và cooperative cancellation cho scan đang chạy.

**Ảnh hưởng:** UI vẫn cần tự dựng host/worker, request đã không còn cần có thể tiếp tục tiêu tài nguyên. Đóng worker không phải cam kết portable về việc ngắt ngay native call đang chạy.

**Việc cần làm:** chuẩn hóa adapter async, bounded queue, busy response, request IDs/deadlines và hủy tác vụ đang chờ. Hủy scan đang chạy cần token/checkpoints trong Rust hoặc process isolation có semantics rõ. Phân biệt progress với kết quả hợp lệ; không trả partial success khi vượt budget.

### P2 — CI, toolchain và kiểm thử mở rộng

**Bằng chứng:** [neutral-engine workflow](../.github/workflows/neutral-engine.yml) có cargo tests, platform builds và Node consumer matrix; chưa có steps fmt/Clippy, benchmark/fuzz harness liên tục hoặc job MSRV 1.83 tường minh. Node metadata cho phép >=18 và matrix có 18/24; cần policy runtime support được duy trì. Có invariant tests hiện hành, không phải hoàn toàn không có property-like coverage.

**Việc cần làm:** thêm fmt/Clippy gate, declared-MSRV và supported-runtime matrix; randomized/property/fuzz harness cho parser/FFI/angle boundaries, benchmark regression cho scan và payload. Audit/update dependencies có kiểm soát, chạy lại numeric parity. Chưa kiểm tra vulnerability database để kết luận dependency có lỗ hổng; không đề nghị nâng mọi thứ lên latest mù quáng.

### P2 — Phân phối chưa đủ rộng để gọi là mọi nền tảng

Snapshot registry/tarball đã kiểm tra trong phiên làm việc ngày 17/10: npm `@7mlabs/astrology` 0.10.1 có macOS ARM64 và Linux x64; Linux cần glibc 2.38+ và `libgcc_s.so.1`. Bản npm public được kiểm chứng cục bộ trong review này trên macOS ARM64. Không chạy lại consumer Linux/Windows/Python/.NET trong lượt review này.

PyPI/NuGet và packages UI/MCP riêng chưa có bản public đã xác minh; Rust crates nguồn có `publish = false`. Crate công khai trùng tên `astro-core` thuộc dự án khác, không dùng làm bằng chứng SDK 7mlabs đã lên crates.io. Windows, macOS Intel, Linux ARM, musl và browser WASM chưa có binary/build được công bố trong matrix hiện tại.

**Việc cần làm:** ra từng nền tảng với fresh-install CI; Python cần portable wheels, .NET cần runtime assets theo RID; xác minh tên crate riêng. Không chỉ upload artifact local rồi ghi “online”. Phát hành rộng sau khi SVG/contract/Payload baseline ổn định; không dùng một lần pass macOS để suy cho mọi binding.

### P2 — Ergonomics và capability discovery

Node đã có typed declarations; Python thiên về dict, .NET thiên về JsonElement/generic API. Cần cân nhắc typed DTOs/helpers theo nhu cầu tích hợp, không tạo nhiều model lệch contract. Thêm capability/version discovery dùng được từ package để app biết renderer/chart types/platform có hỗ trợ; tận dụng status/operations đã có trong local host.

## 4. Giới hạn có chủ đích, chưa phải lỗi

- Gregorian UTC 1800–2399; Sun–Pluto; tropical apparent geocentric; Placidus và Whole Sign.
- Chuyển IANA timezone/DST từ giờ sinh sang UTC là trách nhiệm caller. Payload phải xử lý theo **thời điểm sinh**, không lấy offset hiện tại.
- Placidus có thể không tính được tại vĩ độ cao; giữ lỗi rõ, không âm thầm đổi sang Whole Sign.
- Composite là symbolic midpoint; không có speed/Rx/birth timestamp vật lý. Eclipse maxima toàn cầu không đồng nghĩa local visibility.
- Sidereal, nodes, asteroids, fixed stars, topocentric, progressions/returns/Davison và WASM là scope mới. Chỉ ưu tiên khi có use case và reference tests; không bắt buộc làm tất cả trước SVG.

Renderer UI hiện chỉ vẽ body aspects tối đa 40 nét, có counts và bảng đầy đủ; đó là lựa chọn hiển thị, không phải core đánh mất aspects. Renderer xuất bản mới cần policy coverage rõ hơn. Label spreading hiện có không đủ bằng chứng cho mọi cụm nhiều vòng; đây là rủi ro layout cần kiểm thử, chưa phải lỗi định vị core đã xác nhận.

## 5. Thận trọng khi dùng source chart cũ

Source legacy đã xác định ở repo `matrix-destiny`, `frontend/src/app/astro/product/NatalChartVisualizer.tsx`. Không thuộc engine hiện hành. Adapter cũ có cấu trúc object khác payload arrays mới, tự tính aspects với orbs riêng và có dùng `absolutePositionNormalized || degree` (giá trị 0 có thể bị coi như thiếu). Luồng location cũ lấy current UTC offset, không đủ làm chuẩn lịch sử giờ sinh.

Dùng làm tham chiếu hình thức/luồng; không port các hành vi này sang renderer mới và không quy chúng thành lỗi của core SDK hiện tại.

## 6. Thứ tự nâng cấp đề nghị

1. **Baseline nhỏ:** formatter, artifact/version handshake và docs hiện hành; bổ sung reference cases có giá trị trước khi đổi phép tính.
2. **SVG natal basic/advanced:** module tách riêng, brand vector packs, deterministic standalone output, custom pack và layout tests; tích hợp local Payload sớm.
3. **Payload thực:** profiles/output projection có số đo, async host, queue/deadline/cancellation và xử lý timezone đúng.
4. **Mở rộng:** couple/composite/transit SVG; sau đó từng runtime/platform đã qua final-package install gates.
5. **Tính toán mới:** xếp theo nhu cầu sản phẩm và khả năng đối chứng, không viết lại engine chỉ để mở rộng danh sách tính năng.

## 7. Lệnh tái chạy các kiểm tra nguồn

Từ root SDK (target build giữ ngoài checkout):

```sh
cargo test --manifest-path neutral-engine/Cargo.toml --locked --offline --target-dir /tmp/7mlabs-sdk-review-target
cargo fmt --manifest-path neutral-engine/Cargo.toml --all -- --check
cargo clippy --manifest-path neutral-engine/Cargo.toml --locked --offline --all-targets --target-dir /tmp/7mlabs-sdk-review-target -- -D warnings
cargo build --manifest-path examples/rust/Cargo.toml --locked --offline --target-dir /tmp/7mlabs-sdk-review-target
node --input-type=module -e "import {degreeInSign} from './examples/frontend/ui/data.mjs'; console.log(degreeInSign(29.9999))"
```

Parity probe dùng public npm 0.10.1 đã fetch/verify integrity và CLI Rust vừa build, lặp toàn bộ 339 fixtures trong `tests/conformance/cases.json`, `deepStrictEqual` decoded JSON và expected error status. Probe synthetic theo grid ở mục 1. Không chạy lại mọi release toolchain, browser export, load/concurrency stress, fuzz hoặc astronomical references mới trong lượt này; chúng là gates của kế hoạch tiếp theo.
