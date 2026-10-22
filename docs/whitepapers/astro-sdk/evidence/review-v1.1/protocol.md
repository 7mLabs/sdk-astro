# Đề cương rà soát v1.1

Ngày 22/10/2026, trước phép chạy bổ sung. Không đăng ký trước; đã biết kết quả v1.0.

1. Giữ nguyên commit mã sản phẩm và toàn bộ 17 fixture, không chọn lại theo kết quả.
2. Sửa công cụ đo: truyền nguyên JSON nguồn đến native, không parse/serialize qua float Python; so sánh đệ quy có phân biệt bool và number; Decimal không dung sai; từ chối duplicate keys và NaN/Infinity. Kiểm tra comparator bằng ca âm trước corpus.
3. Giữ log Cargo mới, kiểm tra exit status, tập hợp riêng test suites thay vì chỉ cộng các dòng ok tùy ý. Hash binary, lockfiles, script, fixture tham chiếu và từng payload.
4. Lưu byte output native và expanded dưới gzip để tránh chiếm nhiều dung lượng. Gzip chỉ dùng lưu artifact, không phải baseline nghiên cứu.
5. Kiểm tra số inputBytes bằng độ dài raw JSON đã loại whitespace ngoài chuỗi; nếu khác vì canonical number/escaping thì báo khác biệt, không âm thầm đổi metric.
6. Đối chiếu từng body, lưu độ lệch đầy đủ và tách ba epoch riêng; không gọi lại Skyfield nên tiếp tục ghi rõ oracle lưu sẵn.
7. Tính lại tổng, median theo mẫu, min/max, số mẫu tăng/giảm; không kiểm định ý nghĩa thống kê trên corpus tiện dụng.
8. Rà nguồn chính thức và vị trí mã. Sửa trích dẫn [2] không dùng tài liệu Swiss để chứng minh triển khai riêng Astro.
9. Biên tập theo ba lớp đọc: vấn đề dễ hiểu, kết quả chính, phụ lục tái lập/thuật ngữ. Thêm bảng luồng dữ liệu và giải thích ý nghĩa con số. Không thêm claim hiệu năng/chất lượng LLM chưa đo.
