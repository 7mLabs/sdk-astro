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
