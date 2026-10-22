# Phản biện kỹ thuật và các sửa đổi trong bản 1.1

Ngày 22/10/2026. Phạm vi: lập luận, số liệu, trích dẫn, khả năng kiểm tra lại và khả năng đọc của paper 1.0. Đã được giao sửa cả PDF và Word. Đây là lượt phản biện do cùng trợ lý thực hiện, không phải phản biện độc lập hoặc chứng nhận khoa học.

## Nhận định

Bản 1.0 có dữ liệu thực và đã nêu nhiều giới hạn quan trọng. Không tìm thấy sai số trong tổng byte, tỷ lệ giảm của 17 mẫu hoặc số 132 test. Tuy vậy, bộ đo có các điểm yếu về đối chiếu kiểu và chuyển số; tên bài dễ làm người đọc kỳ vọng bằng chứng đa ngôn ngữ rộng hơn dữ liệu thực tế; phần đọc phổ thông và hướng dẫn kiểm tra lại còn thiếu. Bản 1.1 giữ kết luận trong miền đã thử, làm chặt phương pháp và thêm các giải thích cần thiết.

## Phát hiện và cách xử lý

| Mức | Vị trí bản 1.0 | Vấn đề và căn cứ | Thay đổi bản 1.1 | Trạng thái |
|---|---|---|---|---|
| Quan trọng | Mục 3.3; evidence/analyze.py, strict(src)==strict(restored) | Equality Python có thể coi bool bằng số (True == Decimal(1)); đọc số bằng Decimal riêng chưa đủ bảo toàn kiểu JSON | Comparator đệ quy kiểm tra type trước giá trị; ca âm bool/số, null/bool, đổi số, đảo mảng, mất trường | Đã sửa công cụ và chạy lại; 17/17 vẫn đạt |
| Quan trọng | Phép gọi compressor/expander trong analyze.py | Parse/serialize Python trước native thêm một bước biến đổi số vào phép đo; không cô lập hoàn toàn core | Truyền nguyên văn JSON vào native và giữ đầu ra native làm input cho expand | Đã sửa, toàn bộ số byte theo mẫu khớp v1.0 |
| Vừa | Tiêu đề và framing | “Tính nhất quán ... đa ngôn ngữ” rộng hơn thực nghiệm chỉ chạy Rust CLI và Rust tests | Đổi tên thành “Astro SDK và khả năng bảo toàn dữ liệu thiên văn”; tách kiến trúc nhiều binding khỏi kiểm chứng các gói | Đã sửa |
| Vừa | Mục 2, đoạn Swiss | Trích dẫn [2] đúng về API Swiss nhưng đặt cuối đoạn còn chứa hành vi riêng Astro | Đặt [2] sát mô tả Swiss, thêm [7] ngay hành vi triển khai; các nguồn [7]–[8] có tên hàm xác định | Đã sửa và đọc lại nguồn liên quan |
| Vừa | Mục 3.3 và 5.2 | inputBytes chỉ dẫn metric; người đọc dễ nhầm với byte tệp gốc hoặc giảm token | Đếm lại output/expanded bytes; xác nhận nguồn lexical gọn hơn/kém native 44 byte do cách viết số ở 5 tệp; định nghĩa mốc đo và nhắc byte khác token | Đã xác minh; ghi trong kết quả và dữ liệu đi kèm |
| Vừa | Mục 5.2 | Chỉ mức giảm tổng dễ bị đọc như mức giảm điển hình | Thêm 15 giảm/2 tăng, trung vị 40,12%, miền quan sát −43,11% tới 51,03%; giải thích trọng số và không suy ra quần thể | Đã tính lại |
| Vừa | Mục 5.3 | Bốn request có một epoch lặp; “sai lệch 0” có thể bị đọc như sai số vật lý bằng 0 | Nêu 40 cặp với mỗi nguồn nhưng 30 tổ hợp epoch–body, xác định Moon ngày 29/02/2020 là cực đại, giải thích giây cung và cùng mô hình Swiss | Đã đối chiếu từng body; chưa chạy lại Skyfield |
| Vừa | Hồ sơ tái thực hiện | Thiếu đầu ra nguyên văn và hash binary/lockfile/script; khó truy lại phép đo | Lưu 34 raw outputs dạng gzip, 4 natal outputs, hash, log/suite inventory, lệnh tái lập đầy đủ | Đã bổ sung |
| Vừa | Toàn bài | Nhiều thuật ngữ xuất hiện trước giải thích; thiếu tình huống để hiểu tác động | Viết lại mở đầu, ví dụ giờ mơ hồ dựa trên test, bảng luồng xử lý, ví dụ 900 -> 1.288 byte, phụ lục thuật ngữ và ý nghĩa dùng thực tế | Đã đọc lại ngữ nghĩa, không chỉ kiểm regex |
| Vừa | Hướng nghiên cứu tiếp | Một danh sách mong muốn chưa cho biết cần đo gì | Bảng câu hỏi, phép đánh giá và điều kiện cần chốt; giữ nhãn khung sơ bộ, không gọi kế hoạch đủ thực thi | Đã bổ sung |
| Nhẹ | build_paper.py | Table Swiss dùng literal 0; table đã materialize không tự cập nhật khi sửa results | Bảng số lấy giá trị thật, có template nguồn riêng; sửa caption detection và link URL bấm được | Đã sửa công cụ xuất |

Các vấn đề comparator/serialization là rủi ro phương pháp được phát hiện khi review; không có bằng chứng rằng chúng đã làm sai kết quả corpus v1.0. Không trình bày chúng như lỗi sản phẩm Astro.

## Kiểm chứng số liệu
- Cargo: 132 passed, 0 failed, 0 ignored; bộ đo comparator không cộng vào tổng.
- 17/17 roundtrip theo comparator typed; 17/17 coverageComplete và omitted rỗng.
- Input native 20.014.085; output 11.554.591; giảm 8.459.494 byte = 42,26770297018325%.
- Trung vị mức giảm 40,1214883732563%; 15 mẫu nhỏ hơn, 2 mẫu lớn hơn.
- Input lexical chỉ bỏ whitespace: 20.014.129 byte; chênh 44 byte. Ví dụ 0.000001 thành 1e-6, có cùng giá trị.
- 4 requests/3 epochs, 10 bodies/request. Max Skyfield stored-reference 0,00012131835345 độ ở Moon ngày 29/02/2020; xấp xỉ 0,437 giây cung.
- Không thay tuyên bố 339 historical conformance bằng “đã chạy mới”. Số này không phải tổng test của nghiên cứu.

## Nguồn đã kiểm tra lại
- Swiss Programming Interface: Ephemeris flags, Position and Speed, Error handling; phần chuyển đổi thời gian đã đọc trong lượt biên soạn. Nguồn hỗ trợ API, không hỗ trợ hiệu quả riêng của Astro.
- Skyfield Positions: Barycentric / Astrometric / Apparent. Không suy metadata fixture từ tài liệu Skyfield hiện hành; metadata lấy từ fixture.
- Node-API: Implications of ABI stability, URL hiện hành. URL latest-v24.x không mở được bằng công cụ nên không dùng làm nguồn; bản cuối dẫn URL đã đọc.
- IANA Time Zones: mô tả cập nhật và lịch sử quy tắc.
- RFC 8259, T. Bray, 2017: values/objects/arrays/numbers; dùng để phân biệt kiểu và quy tắc JSON, không viện RFC làm bảo đảm compressor.
- Source local: provider, time, FFI, compression, natal validator, test initialization và log các test.

## Giới hạn còn mở
Không có reviewer độc lập; chưa xác nhận tác giả cá nhân, tài trợ và lợi ích liên quan. Chưa có benchmark latency/memory/token, đánh giá LLM, tái tạo oracle Skyfield hoặc ma trận binding mới. Chưa thể suy hiệu lực chiêm tinh từ các kiểm tra phần mềm. Không có quyết định công bố/submission trong phạm vi lần này.

Bản 1.0 được giữ tại archive/v1.0 để đối chiếu. Bản chính PDF/Word được cập nhật tại cùng đường dẫn và mang phiên bản 1.1. Kết quả kiểm tra file cuối nằm trong ra-soat.md.
