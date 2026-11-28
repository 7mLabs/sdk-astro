# Hồ sơ nghiên cứu Astro SDK

Ngày chốt nguồn: 22/10/2026. Đơn vị: 7MLabs. Ngôn ngữ: tiếng Việt.
Độc giả: kỹ sư SDK, kiến trúc sư phần mềm và người đánh giá kỹ thuật.
Thiết kế: nghiên cứu trường hợp phần mềm, phân tích mã nguồn và kiểm tra thực thi hữu hạn; tổng hợp tài liệu kỹ thuật có chọn lọc.
Revision ban đầu sạch: cd768d0192cd7e6f7aff266b8ebdb3f8685eeb39, workspace version 1.0.0.

## Đề cương trước phép đo
Đã đọc README, kiến trúc, tài liệu nén và provenance trước khi lập đề cương; đây không phải đăng ký trước nghiên cứu.
RQ1: Các ranh giới provider, thời gian và FFI công bố/giữ những hợp đồng nào?
RQ2: Nén compact bảo toàn giá trị JSON và thay đổi kích thước thế nào trên tất cả examples/*-result.json hiện có?
RQ3: Kiểm tra Rust hiện có và đối chiếu lại fixture natal cho phép kết luận tới đâu?
Chạy cargo test --workspace --locked; ghi nguyên log, môi trường. Chạy bản CLI Rust build từ source, nén/giải nén toàn bộ fixture theo glob, không loại mẫu theo kết quả. So sánh giá trị JSON, không so whitespace. Byte theo metrics native; mức giảm = (inputBytes-outputBytes)/inputBytes*100. Không đo token hoặc latency, không kiểm định thống kê trên corpus convenience.
Đối chiếu longitude natal hiện tính với Swiss và Skyfield được lưu trong natal-references.json; sai lệch góc tròn = min(|a-b| mod 360,360-(|a-b| mod 360)). Không gọi đây là chạy lại Skyfield hoặc kiểm chuẩn độc lập mới.
Quy tắc kết luận: chỉ báo số thực chạy; lỗi/độ tăng kích thước được giữ. Test pass không chứng minh đúng trên mọi đầu vào hoặc xác nhận chiêm tinh.

## Nguồn và lựa chọn
Nguồn nội bộ: mã nguồn provider, core compression/natal, FFI, astro-time, manifests; README và docs chỉ dùng mô tả hoặc báo cáo lịch sử, ưu tiên mã khi mâu thuẫn.
Nguồn ngoài: mở trực tiếp tài liệu chính thức Swiss Ephemeris, Node-API, IANA và Skyfield ngày 22/10/2026. Chọn nguồn giải thích hợp đồng và phương án nền; không chọn blog quảng cáo, không thực hiện systematic review.
Giới hạn: một lượt phân tích có AI hỗ trợ, không reviewer độc lập; mẫu fixtures do dự án cung cấp; chưa xác nhận tác giả cá nhân, tài trợ, peer review hay điều kiện nộp arXiv.

## Profile trình bày
Paper một cột A4, lề 25.4 mm, serif Times New Roman 11 pt, title 14 pt, single spacing, body justify thụt 16 pt, đoạn đầu sau heading không thụt; references hanging 18 pt. Abstract ngay trang đầu, không bìa quảng cáo. Chọn profile mặc định của skill, không tuyên bố dùng template arXiv chính thức.

## Nhật ký thực hiện và kết quả
- 22/10/2026: mở nguồn ngoài trực tiếp (không dùng search snippets): Swiss Programming Interface, Node-API ABI stability, IANA Time Zones, Skyfield overview và Positions. Đọc các phần API/quy ước liên quan; không đọc toàn bộ mọi manual.
- Chạy cargo test --workspace --locked: 132 passed, 0 failed, 0 ignored; log trong evidence/cargo-test.log. Build CLI bằng examples/rust/Cargo.lock, profile dev.
- Chạy evidence/analyze.py: tất cả 17 mẫu đúng JSON roundtrip, cả 17 outputBytes khớp byte stdout. Corpus nguồn đều version 0.10.0-alpha.1; compressor hiện tại 1.0.0.
- Tổng 20.014.085 -> 11.554.591 byte, giảm 42,26770297018325%. Hai mẫu tăng kích thước được giữ đầy đủ.
- Đối chiếu kinh độ: 4 request, 3 epochs, 10 bodies/request; khớp Swiss lưu sẵn, Skyfield lưu sẵn lệch tối đa 0,00012131835345030595 độ. Không chạy lại oracle Skyfield/DE421.
- Không đổi tiêu chí lấy mẫu sau phép đo. Không chạy benchmark thời gian/token, không đo FFI parity mới. Không có subagent/reviewer độc lập.

## Bảng luận điểm và bằng chứng
| Luận điểm trong paper | Nguồn/vị trí | Loại và giới hạn | Cách diễn đạt |
|---|---|---|---|
| Một core, nhiều giao diện | astro-ffi/src/lib.rs; workspace Cargo.toml; docs/architecture.md | Phân tích mã, không đo parity mọi binary | Cùng triển khai, wrapper vẫn cần kiểm thử |
| Provider khóa trạng thái và kiểm cờ Moshier | astro-provider-swiss/src/lib.rs: PROVIDER, natal, reset_provider, EphemerisSession | Quan sát mã + test hiện có; chưa đo contention | Khóa trong từng native library |
| Hệ nhà thất bại không trả fallback thành công | provider natal sau swe_houses_ex | Failure trace suy từ mã, không fault injection mới | Trả Err khi status < 0 |
| Điểm mở rộng có dữ liệu nhúng | provider PROVENANCE.md mục Optional additional natal points | Mô tả provenance đã đọc | Offline không có nghĩa không có ephemeris data |
| Gap/fold và cảnh báo thời gian | astro-time/src/lib.rs: resolve_json | Mã và test hữu hạn | Từ chối gap, yêu cầu chọn fold |
| 132 test đạt | evidence/cargo-test.log, các dòng test result | Một lần chạy Mac ARM64 | Không đồng nghĩa bao phủ/vét cạn |
| 17/17 roundtrip và byte totals | evidence/results.json, compression | Corpus mẫu lịch sử, không production sample | Chỉ phạm vi đã thử |
| 2 mẫu phình ra | results.json query-aspects/query-between | Phản ví dụ thật với always smaller | Nén không luôn giảm byte |
| 0,0001213 độ | results.json references và natal-*.json; natal-references.json | Oracle được lưu sẵn, 3 epochs | Không gọi kiểm chuẩn độc lập mới |
| estimatedTokens là bytes | compression.rs finalize_metrics; docs/payload-compression.md | Tính chất triển khai | Không kết luận tiết kiệm token |
| 339 trường hợp binding | docs/testing.md mục SDK 1.0.0 | Báo cáo lịch sử, không chạy lại | Dự án báo cáo |

## Thuật ngữ
- fact: dữ kiện tính toán hoặc tổ chức theo policy; không là bằng chứng về tính cách.
- evidence trong payload: tham chiếu truy nguyên giữa facts; không là bằng chứng hiệu lực chiêm tinh.
- compact: biến đổi biểu diễn, giữ giá trị JSON; không giữ bytes nguồn.
- focused/budgeted: có lựa chọn phạm vi, không cam kết phục hồi phần omitted.
- TT/UT1/UTC: các thang thời gian khác nhau; không dùng thay nhau trong đối chiếu.

## Điều kiện công bố
Bản tiếng Việt theo yêu cầu, chưa có tác giả cá nhân chịu trách nhiệm. Không được giao chuẩn bị submission arXiv nên không xác nhận chính sách/điều kiện nộp hiện hành; nếu nộp cần kiểm tra riêng chính sách, tác giả, ngôn ngữ và cách công bố dữ liệu. Không tự công bố repository mới, không push hoặc gửi tài liệu ra ngoài.

## Tái thực hiện
Tại root repository revision đã ghi:
1. cargo test --manifest-path neutral-engine/Cargo.toml --workspace --locked > docs/whitepapers/astro-sdk/evidence/cargo-test.log 2>&1
2. cargo build --manifest-path examples/rust/Cargo.toml --locked
3. python3 docs/whitepapers/astro-sdk/evidence/analyze.py
Dùng Python có standard library; builder Word cần python-docx. Script phân tích đọc repo tương đối từ vị trí của nó. SHA-256 fixture ở results.json; log cargo gồm build và từng ca test. Đây là tái thực hiện do người biên soạn thực hiện, chưa là independent replication.

## Bổ sung sau phản biện kỹ thuật, bản 1.1 — 22/10/2026

Lượt rà giữ corpus ban đầu và công bố thay đổi phương pháp sau khi đã biết kết quả 1.0; không gọi là đăng ký trước hay lặp lại độc lập. Truyền nguyên văn JSON vào CLI, so sánh Decimal có phân biệt kiểu, kiểm tra công cụ đo bằng ca cố ý sai và lưu các đầu ra native. Đã chạy lại test/build/phân tích; số liệu chính không đổi. Chi tiết đầy đủ trong phan-bien-ky-thuat.md và evidence/review-v1.1/protocol.md, results.json, README.md.

Nguồn bổ sung: T. Bray (biên tập), RFC 8259, tháng 12/2017, mục 3–6, https://www.rfc-editor.org/rfc/rfc8259.html, DOI 10.17487/RFC8259; truy cập 22/10/2026. Dùng để giải thích các loại giá trị JSON, không coi lựa chọn so sánh số của nghiên cứu là yêu cầu bắt buộc của RFC. Các nguồn Swiss, Skyfield, Node-API và IANA được đối chiếu lại; dẫn mã dự án cho các khẳng định triển khai Astro. Bản nguồn 1.1 phân biệt ba thời điểm với bốn yêu cầu và báo cáo đầy đủ hai mẫu tăng kích thước.

Đã bổ sung phần dành cho người đọc phổ thông và công bố các phép đánh giá chưa thực hiện. Hồ sơ xuất cuối, kiểm tra hình thức 9 trang và parity Word/PDF nằm trong ra-soat.md và evidence/export-check.json.

## Bổ sung minh họa — bản 1.2

Theo yêu cầu giúp người đọc không chuyên, thêm sơ đồ kiến trúc, luồng natal và hình công thức giảm byte. Đọc lại docs/architecture.md, sử dụng kết quả và mã đã đối chiếu ở lượt 1.1. Hình phân biệt kiến trúc mô tả với phép đo; luồng natal không đại diện mọi operation. Công thức dùng mốc byte native đã công bố, không suy sang token. SVG/PNG và mã vẽ nằm trong figures/. Không thu thập dữ liệu hoặc chạy lại kiểm thử sản phẩm trong lượt bổ sung hình; số đo kế thừa evidence/review-v1.1 và phép tính minh họa được kiểm tra bằng evidence/check_figures.py.


## Cập nhật LaTeX và giải thích toán học — bản 1.3

Mục này thay thế thông tin bản xuất của 1.2: PDF bàn giao được biên dịch trực tiếp bằng LaTeX/Tectonic, 11 trang; Word có 12 trang khi render bằng LibreOffice. Cả hai dùng chung nội dung và danh mục sáu công thức. Bản 1.2 được lưu trong archive/v1.2.

- Bổ sung trực giác, giả thiết, ký hiệu, suy dẫn và ví dụ cho tính khôi phục dữ liệu, tỷ lệ giảm byte, trung bình có trọng số và khoảng cách góc trên vòng tròn. Chứng minh điều kiện cần về tính phân biệt đầu vào bằng phản chứng; chứng minh công thức tổng hợp bằng thế và rút gọn; giải thích khoảng cách góc bằng hai cung nối hai điểm.
- Các suy luận toán học là tự chứa dưới giả thiết được ghi rõ. Định nghĩa JSON dựa trên [11], cách đo và số liệu dựa trên [9], quy ước thiên văn dựa trên [2, 3]. Không coi chứng minh điều kiện cần là chứng minh mã Astro đúng; 17 mẫu không đại diện cho mọi JSON.
- Tính lại chênh lệch byte, tỷ lệ còn lại, phép đổi độ sang giây cung và các ví dụ minh họa bằng evidence/check_math.py; kết quả math-check-v1.3.json. Không chạy thêm benchmark sản phẩm trong lần cập nhật trình bày này.
- Đã xem tất cả 11 trang PDF và 12 trang Word render, kiểm tra dấu tiếng Việt, bảng, chú thích và sáu công thức. Sau sửa dấu gạch kép trong lệnh CLI, xem lại trang PDF 10–11. Không thấy cắt chữ hoặc chồng nội dung. Word chưa được mở bằng Microsoft Word desktop.
- evidence/check_latex_exports.py xác nhận đủ 289 đơn vị văn bản theo đúng thứ tự trong hai bản, sáu công thức tại cùng vị trí nội dung và sáu đối tượng Office Math. Công thức được đối chiếu trực quan vì bộ trích xuất PDF sắp xếp chỉ số toán học khác nhau. Kết quả và SHA-256: evidence/latex-export-check.json.
- evidence/check_figures.py xác nhận ba ảnh Word trùng nguồn và ảnh PDF trùng pixel, nằm ở trang 2, 4, 6; kết quả figure-check.json. Sơ đồ nguồn SVG vẫn nằm trong figures/.
- Nguồn LaTeX tại latex/main.tex; ZIP đi kèm chứa main.tex, README và ảnh. Font PDF dùng TeX Gyre Termes; Word giữ định dạng Times New Roman. Các khác biệt font và phân trang không thay đổi nội dung.
- Không sửa mã sản phẩm, không commit, push hoặc công bố. Giữ giới hạn dữ liệu tham chiếu đã lưu và trạng thái bản thảo chưa qua phản biện khoa học độc lập.


## Rà soát cuối bản 1.4

Bản 1.4 thay thế bản xuất 1.3; bản cũ được lưu tại archive/v1.3. Báo cáo chi tiết về vị trí lỗi, căn cứ và sửa đổi: phan-bien-cuoi-v1.4.md.

- Rà suy luận sáu công thức; bổ sung giả thiết miền đầu vào và tính xác định, n nguyên dương, cách đọc Σ, bước chứng minh cận của trung bình có trọng số. Phân biệt sai lệch kinh độ với khoảng cách góc đầy đủ; thu hẹp mô tả compact và nhánh Moshier theo bằng chứng.
- Sửa hình 2 để R là con số phần trăm như định nghĩa trong bài; giải thích 42,27% bằng lời riêng. Hình 3 phân biệt mục tiêu thiết kế với việc đã kiểm tra khả năng khôi phục.
- Tái phân tích dữ liệu thô đã lưu bằng evidence/final_review.py: 17 cặp nguồn/giải nén, số byte và hash, 80 cặp kinh độ với hai nguồn, tổng 132 ca trong log; các giá trị báo cáo không đổi. Đủ citation [1]–[11], đã mở lại năm nguồn chính thức; chi tiết trong báo cáo rà soát. Không chạy mới SDK hoặc tạo mới oracle thiên văn.
- PDF biên dịch trực tiếp bằng Tectonic: 11 trang; log evidence/latex-build-v1.4.log không có cảnh báo. Word render bằng LibreOffice: 12 trang, chưa thử Microsoft Word desktop. Đã xem tất cả trang của cả hai bản cuối, gồm công thức, hình, bảng và tham khảo. Các trang trước phần công bố không thay đổi khi rút gọn đoạn công bố cuối; đã xem lại trang cuối tương ứng sau xuất lại.
- 289 đơn vị văn bản khớp theo đúng thứ tự, sáu công thức tại cùng vị trí, sáu đối tượng Office Math chỉnh sửa được. Bộ kiểm tra chỉ loại các header bảng lặp đã xác nhận trực quan: glossary LaTeX trang 11; bảng 1 Word trang 7; bảng 5 Word trang 11. Một chỉ số j bị bộ trích xuất Word PDF đưa sai thứ tự được sửa bằng chuỗi đối chiếu cụ thể, không xóa tùy ý văn bản. Kết quả và SHA-256 lưu evidence/latex-export-check.json.
- Ba ảnh nhúng trong Word khớp nguồn; ảnh PDF ở trang 2, 4, 6 trùng pixel với nguồn, theo evidence/figure-check.json. Nguồn SVG và mã vẽ đã cập nhật đồng bộ.
- Đã xử lý caption bảng 4 tách trang, đoạn lệnh giãn từ và trang LaTeX cuối chỉ có vài dòng. References vẫn 10 pt; không rút nội dung bằng chứng để giảm trang. Phần công bố được viết gọn, giữ thông tin thiếu tác giả/tài trợ và trạng thái chưa qua phản biện độc lập.
- Nguồn đóng gói lại trong Astro SDK 7MLabs LaTeX.zip, gồm main.tex, README và ảnh; PDF chính khớp byte với latex/main.pdf. Không sửa mã sản phẩm, không commit/push/công bố.

Không còn lỗi nội dung hoặc trình bày được phát hiện cần chặn bộ bàn giao trong phạm vi rà soát này. Các giới hạn khoa học nêu trong bài vẫn còn hiệu lực; không xem đây là chứng nhận đúng toàn bộ SDK hay đủ điều kiện xuất bản.
