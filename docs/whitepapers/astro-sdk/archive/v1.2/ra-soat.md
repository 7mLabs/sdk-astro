# Kết quả rà soát paper Astro SDK

Ngày: 22/10/2026. Phiên bản paper: 1.1.

## Nội dung và bằng chứng

Đã rà theo hướng phản biện kỹ thuật: kiểm tra quan hệ giữa câu hỏi, phương pháp, kết quả và kết luận; tính lại số liệu; đối chiếu nguồn chính thức và mã triển khai. Báo cáo chi tiết các điểm yếu và cách sửa nằm trong phan-bien-ky-thuat.md. Đây là lượt tự rà có AI hỗ trợ, không phải phản biện độc lập.

Chạy lại 132 kiểm thử Rust: tất cả đạt. Đo lại 17 mẫu bằng đầu vào nguyên văn và công cụ so sánh JSON phân biệt kiểu, dùng Decimal; 17 mẫu bảo toàn cấu trúc, kiểu và giá trị theo quy tắc công bố. Tổng byte giảm 42,27%, trung vị 40,12%; hai mẫu tăng kích thước được báo cáo. Số liệu đối chiếu thiên văn là từ ba thời điểm riêng biệt, so với tham chiếu lưu sẵn, chưa tạo lại Skyfield độc lập. Kết quả chính không đổi sau khi làm chặt phương pháp.

Đã bổ sung cách đọc bài, ví dụ thực tế, bảng luồng xử lý, giới hạn suy luận, hướng nghiên cứu tiếp, hướng tái thực hiện và bảng thuật ngữ. Tài liệu tham khảo [1]–[11] được dùng trong bài; nguồn Swiss được giới hạn cho API Swiss, còn khẳng định triển khai Astro dẫn mã Astro. Con số lịch sử 339 trường hợp không cộng vào 132 test chạy mới.

Không chứng minh tính mới thuật toán, hiệu năng production, tiết kiệm token, độ đúng toàn miền ngày hay hiệu lực dự báo chiêm tinh. Chưa xác nhận tác giả cá nhân chịu trách nhiệm, tài trợ/lợi ích và phản biện độc lập. Giữ trạng thái bản thảo kỹ thuật.

## Bản xuất đã kiểm tra

- Word chỉnh sửa được: 103 đoạn và 6 bảng native.
- PDF: 9 trang, xuất từ chính DOCX bàn giao qua bundled LibreOffice, runtime bundle 26.1017.31632.
- Đã xem từng ảnh trang 1–9 của bản cuối trong /tmp/astro-paper-v11-final/. Không thấy chữ bị cắt, lỗi dấu tiếng Việt, bảng tràn hoặc hàng bị chia; caption gắn với bảng. Trang cuối chứa tài liệu tham khảo cuối và phần công bố trách nhiệm.
- Times New Roman; tiêu đề 14 pt, thân bài 11 pt, bảng và tham khảo 10 pt; A4 một cột, lề 1 inch. Sửa định dạng caption và độ rộng bảng thuật ngữ sau lượt xem đầu.
- Đối chiếu văn bản theo đúng thứ tự đoạn/ô giữa DOCX và PDF đạt: 23.052 ký tự sau bỏ khoảng trắng và số trang. Kết quả và SHA-256 nằm trong evidence/export-check.json.
- PDF được kiểm tra chính là PDF bàn giao. Chưa mở bằng Microsoft Word desktop; phân trang có thể khác với renderer khác.

## Hồ sơ đi kèm

paper-source.md là bản nguồn có chỗ chèn số liệu; build_paper.py tạo whitepaper.md hoàn chỉnh và DOCX từ evidence/review-v1.1/results.json. Thư mục evidence/review-v1.1 lưu protocol, mã đo, log, đầu ra native và hướng tái thực hiện. Bản 1.0 giữ trong archive/v1.0. Không sửa mã sản phẩm, không commit, push hoặc công bố.

## Cập nhật minh họa — bản 1.2

Bản bàn giao hiện tại là 1.2, gồm 10 trang, 113 đoạn Word, 6 bảng và 3 hình nhúng. Phần rà ở trên ghi nhận lịch sử bản 1.1; kết quả xuất trong mục này thay thế số trang và số đoạn cũ.

- Hình 1: các lớp giao diện ngôn ngữ, lõi Rust, thời gian và provider. Nguồn [1, 7, 8]; đây là sơ đồ trách nhiệm, không mô tả đầy đủ dependency graph.
- Hình 2: công thức R = 100 × (B − C) / B với B > 0, ví dụ dùng tổng byte thật của 17 mẫu từ [9]. Đã tính lại phần chênh lệch và tỷ lệ còn lại 57,73%.
- Hình 3: luồng natal được đơn giản hóa, có hướng đọc, bước chuyển dữ liệu tùy chọn và điều kiện kiểm tra trạng thái lỗi. Nguồn [1, 7, 8].
- Đã xem toàn bộ ảnh trang 1–10 trong /tmp/astro-paper-v12-final; hình và caption cùng trang, chữ Việt rõ, không tràn hay chồng lấn. Bảng 2 tiếp sang trang 7 với tiêu đề cột được lặp; hàng không bị cắt.
- Văn bản Word/PDF khớp theo thứ tự: 24.256 ký tự sau chuẩn hóa khoảng trắng, bỏ số trang và đúng một header bảng lặp đã xác nhận trực quan ở trang 7. Phần loại bỏ được ghi cụ thể trong evidence/export-check.json, không bỏ tùy ý nội dung khác.
- Kiểm tra riêng ảnh nhúng bằng evidence/check_figures.py: Word chứa đúng ba PNG nguồn; PDF chứa ba ảnh ở đúng thứ tự, so sánh pixel sau đổi kích thước với ảnh nguồn. Kết quả lưu evidence/figure-check.json.
- Nội dung, bảng, công thức (1) và chú thích vẫn là văn bản Word chỉnh sửa được. Hình là ảnh nhúng; SVG cùng mã tạo hình lưu tại figures/ để chỉnh sửa và tái tạo. Chưa mở bằng Word desktop; phân trang có thể thay đổi trên renderer khác.
- Cập nhật này minh họa bằng chứng hiện có, không thêm benchmark hoặc kết quả thực nghiệm mới. Bộ 1.1 được giữ trong archive/v1.1.
