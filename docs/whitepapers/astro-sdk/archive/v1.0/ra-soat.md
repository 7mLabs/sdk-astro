# Kết quả rà soát paper Astro SDK

Ngày: 22/10/2026. Phiên bản paper: 1.0.

## Trạng thái nội dung
Bản thảo đã rà soát trong phạm vi nghiên cứu trường hợp kỹ thuật. Các số trong abstract, bảng và kết luận đối chiếu với evidence/results.json; nguồn lịch sử 339 trường hợp được tách khỏi 132 test chạy mới. Corpus 17 payload đều mang phiên bản nguồn cũ và được nêu rõ. Hai mẫu tăng kích thước được báo cáo. Đối chiếu Skyfield là dùng fixture, không được trình bày như chạy oracle độc lập. Mọi mục tham khảo [1]–[10] được dùng trong thân bài.
Không chứng minh tính mới thuật toán, chi phí/hiệu năng production, tiết kiệm token, độ đúng toàn miền ngày, hay hiệu lực dự báo chiêm tinh. Chưa có tác giả cá nhân xác nhận trách nhiệm, thông tin tài trợ/lợi ích hoặc phản biện độc lập. Không đánh dấu sẵn sàng nộp arXiv; submission không nằm trong yêu cầu.

## Bản xuất
- Paper Astro SDK 7MLabs.docx: nội dung chỉnh sửa được, 77 paragraph và 2 bảng native Word.
- Paper Astro SDK 7MLabs.pdf: 6 trang, xuất trực tiếp từ cùng DOCX qua bundled LibreOffice và render_docx.py.
- Runtime bundle 26.1017.31632, Python/docx và LibreOffice của Codex; không dùng Word desktop.
- Đã mở ảnh riêng từng trang 1–6 của bản cuối tại /tmp/astro-paper-final/page-N.png. Chữ Việt đọc được, bảng không cắt hàng, caption gắn bảng, tiêu đề và References có hierarchy. Không có lỗi chặn đọc quan sát được.
- Đã sửa border xanh kế thừa dưới title, font theme ở heading và giãn từ quá mức do hash dài. Bản cuối dùng Times New Roman, title 14 pt, body 11 pt, References/bảng 10 pt; A4 một cột, lề 1 inch.
- Đối chiếu toàn bộ text theo đúng thứ tự paragraph/cell giữa DOCX và PDF đạt: 18.069 ký tự sau bỏ whitespace và số trang. Không chỉ kiểm tra sự có mặt của từng đoạn. Hash hai file và kết quả máy trong evidence/export-check.json.
- PDF kiểm tra chính là bản bàn giao; dùng chung bộ ảnh để đánh giá layout Word đã chuyển đổi và PDF. Chưa mở bằng Microsoft Word desktop; pagination có thể đổi ở renderer khác.

## Tài nguyên hỗ trợ
whitepaper.md là nguồn nội dung, build_paper.py tạo Word; evidence/analyze.py và results.json lưu cách đo và kết quả. check_exports.py kiểm tra parity. Hồ sơ chọn nguồn/bảng luận điểm trong ho-so-nghien-cuu.md. Không sửa source sản phẩm, không commit/push/publish.
