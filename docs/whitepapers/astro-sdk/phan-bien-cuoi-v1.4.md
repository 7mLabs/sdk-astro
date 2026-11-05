# Rà soát kỹ thuật cuối bản 1.4

Ngày 22/10/2026. Phạm vi: bản thảo, nguồn LaTeX, Word, dữ liệu thô và mã tại commit cd768d0192cd7e6f7aff266b8ebdb3f8685eeb39. Đây là lượt rà soát của cùng trợ lý, không phải phản biện khoa học độc lập hay chứng minh bằng proof assistant.

## Những điểm đã sửa

| Vị trí | Vấn đề và tác động | Sửa trong bản 1.4 |
| --- | --- | --- |
| 3.2–3.3 | “JSON hợp lệ” chưa chỉ rõ miền không trùng tên trường; độc giả có thể hiểu bộ đo bao phủ mọi JSON | Giới hạn miền X theo bộ đo, phân biệt quy tắc từ chối trùng tên của nghiên cứu với khuyến nghị SHOULD trong RFC 8259 mục 4 |
| 3.3, công thức 1–2 | Chưa nêu tường minh tính xác định và miền thành công của cả chuỗi, phiên bản giải mã, tính bắc cầu | Nêu E có đầu ra, D trả dữ liệu trên các biểu diễn đó, cùng phiên bản; thêm bước suy ra x ≡ y. Mệnh đề chỉ là điều kiện cần |
| 3.4, công thức 4–5 | Thiếu n > 0 và giải thích Σ; bước từ trọng số tới cận min–max quá nhanh | Định nghĩa n nguyên dương và chỉ số; giải thích cộng trọng số và cộng bất đẳng thức |
| Hình 2 | R được định nghĩa là con số phần trăm nhưng hình lại viết R ≈ 42,27%, không thống nhất ký hiệu | Viết R ≈ 42,27 và diễn giải riêng “giảm khoảng 42,27%” |
| 3.5, công thức 6 | Có thể nhầm chênh lệch kinh độ với khoảng cách trên thiên cầu | Yêu cầu cùng thiên thể, thời điểm, hệ tọa độ; nêu khoảng cách đầy đủ còn phụ thuộc vĩ độ |
| 4.3–4.4, hình 3 | Mô tả compact dễ bị đọc thành chứng nhận mọi đầu vào; cờ Moshier chưa giới hạn nhánh | Ghi đây là thiết kế được kiểm tra trên 17 mẫu; nêu giới hạn đầu vào; xác định nhánh natal mười thiên thể. Phạm vi thực tế đọc từ coverage.complete/omitted |
| 5.2–5.3 | Baseline byte và ý nghĩa Swiss bằng nhau cần nhắc lại ngay nơi kết quả | Nhắc mốc native và độ chính xác biểu diễn của số tham chiếu đã lưu |
| Bố cục | Trang cuối LaTeX quá ít nội dung, đoạn lệnh giãn từ, caption bảng có thể tách khỏi bảng | Điều chỉnh giãn dòng References, căn trái đoạn lệnh, dự trữ chỗ cho caption và đầu bảng; giữ cỡ References 10 pt |

## Kiểm tra chứng minh

(1) là thuộc tính cần kiểm tra, không phải định lý đã chứng minh cho SDK. (2) suy ra từ E(x)=E(y), D xác định, tính đối xứng và bắc cầu của ≡; kết luận là hai lớp dữ liệu không tương đương không thể có cùng mã nếu khôi phục đúng. Không dùng kết luận làm giả thiết cho triển khai.

(3) là định nghĩa mức giảm với B>0, C≥0; R âm khi dữ liệu tăng, R≤100. (4)–(5) đúng do triệt tiêu B_i và dùng chung mẫu tổng B_j>0. Trung vị không phải trung bình có trọng số. Ví dụ 45% so với 25% được ghi là minh họa, không phải phép đo Astro.

(6) lấy cung ngắn hơn sau chuẩn hóa modulo 360°, xử lý đúng 359°/1°, trùng hướng và đối hướng. Chỉ đánh giá kinh độ, không chứng minh tọa độ đầu vào đúng. Các chứng minh giải thích cách đo, không suy dẫn thuật toán quỹ đạo của Swiss Ephemeris.

## Kiểm tra dữ liệu và trích dẫn

Chạy evidence/final_review.py từ dữ liệu thô đã lưu, không thực thi lại SDK: đối chiếu SHA-256 của 17 tệp nguồn, 34 đầu ra nén/giải nén, binary, lockfile, fixture, mã đo và log; kiểm tra lại giá trị JSON với Decimal, số byte thực tế, coverage và omitted. Tính lại từng tỷ lệ, tổng và trung vị. Đối chiếu 80 cặp kinh độ (40 mỗi nguồn, có thời điểm lặp) từ natal-0..3.json và fixture. Đếm lại 132 kiểm thử đạt từ log, không cộng các ca kiểm tra công cụ đo. Kết quả: evidence/final-review-v1.4.json.

Các giá trị chính không đổi: B=20.014.085, C=11.554.591, giảm 42,267702970…%, trung vị 40,121488373…%; 15 mẫu giảm, 2 mẫu tăng. Có 17/17 trường hợp khôi phục đúng. Đây là tái phân tích dữ liệu cũ, không phải một thí nghiệm sản phẩm mới.

Kiểm tra hai chiều: đủ nguồn [1]–[11], không có nguồn bỏ quên hoặc citation không tồn tại. Nguồn nội bộ [1,4,7,8,9,10] được đối chiếu với checkout và hồ sơ đo; không biến tài liệu phát hành thành bằng chứng đã chạy lại.

Mở lại ngày 22/10/2026 các nguồn chính thức sau và đọc phần liên quan:

- [Swiss Ephemeris Programming Interface](https://www.astro.com/swisseph/swephprg.htm), mục 3.4–3.5 và 9.2: tọa độ, cờ thực tế, lỗi và chuyển đổi thời gian; không dùng thay kiểm chứng triển khai.
- [Skyfield Positions](https://rhodesmill.org/skyfield/positions.html), Barycentric → Astrometric → Apparent: phân biệt hiệu chỉnh và hệ quy chiếu, không cung cấp số đo Astro.
- [Node-API](https://nodejs.org/api/n-api.html), Implications of ABI stability: bảo đảm ABI có giới hạn; thư viện ngoài cần bảo đảm riêng.
- [IANA Time Zones](https://www.iana.org/time-zones): lịch sử giờ địa phương và cập nhật quy tắc. Không cần đổi phiên bản phụ thuộc của SDK trong công việc này.
- [RFC 8259](https://www.rfc-editor.org/rfc/rfc8259.html), mục 3–6: loại JSON, tên trường, mảng và số. Đã sửa cách mô tả tên trùng để không biến SHOULD thành MUST.

## Giới hạn giữ nguyên

Chưa tái tạo oracle Skyfield/DE421; chưa mở rộng miền ngày, nền tảng hay dữ liệu người dùng; không đo hiệu năng hoặc token. Tác giả cá nhân chịu trách nhiệm chưa được xác nhận. Tài liệu là bản thảo kỹ thuật đã rà trong phạm vi, không tự nhận sẵn sàng xuất bản. Kiểm tra file cuối và hash nằm trong ra-soat.md cùng evidence/latex-export-check.json.

## Bản xuất cuối

PDF LaTeX 11 trang, Word 12 trang; đã xem toàn bộ. Tectonic biên dịch không cảnh báo. Đối chiếu đủ 289 đơn vị văn bản theo thứ tự, sáu công thức và ba hình. Các header bảng lặp được kiểm tra riêng và ghi rõ trong evidence/latex-export-check.json. Chưa mở bằng Word desktop. Nguồn LaTeX đã đóng gói lại cùng hình.
