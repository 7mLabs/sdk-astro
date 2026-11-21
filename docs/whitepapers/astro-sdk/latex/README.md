# Paper Astro SDK — 7MLabs, bản 1.4

PDF được biên dịch trực tiếp từ `main.tex`, dùng LaTeX và công thức amsmath. Ba hình nguồn nằm trong `images/`. Không cần font thương mại; tài liệu dùng TeX Gyre Termes.

## Biên dịch

Trong thư mục này, chạy `tectonic main.tex`. Đã kiểm tra bằng Tectonic trên macOS. Lần đầu có thể cần kết nối mạng để tải gói TeX. Có thể dùng XeLaTeX với các gói tương ứng, nhưng luồng đó chưa được kiểm thử trong lần bàn giao này.

## Đồng bộ và bằng chứng

Trong repository, `paper-source.md`, `build_paper.py`, `equations.py` và `build_latex.py` tạo hai bản từ cùng nội dung. Chạy build_paper.py trước build_latex.py rồi biên dịch main.tex. Word giữ công thức Office Math chỉnh sửa được. PDF và Word có thể khác số trang.

Gói ZIP này chứa nguồn dàn trang và hình để biên dịch; hồ sơ thực nghiệm đầy đủ nằm riêng trong `docs/whitepapers/astro-sdk/evidence/review-v1.1/` của bộ bàn giao repository. Các phép chứng minh có giả thiết được nêu trong bài; không phải chứng minh triển khai SDK đúng với mọi đầu vào. Bản thảo kỹ thuật chưa qua phản biện khoa học độc lập.
