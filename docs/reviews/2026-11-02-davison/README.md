# Đối chiếu Davison — 02/11/2026

Trạng thái: **đạt trên bộ mẫu này**, chưa phải chấp thuận phát hành. Không sửa mã tính Davison trong lượt đối chiếu này.

## Bộ mẫu và phương pháp

16 cặp dữ liệu tổng hợp × 2 biến thể (uncorrected/corrected) × 2 hệ nhà (Placidus/wholeSign) = 64 lá số. Có trường hợp đồng nhất, qua đường đổi ngày, năm nhuận/thế kỷ, ranh giới UTC, mốc 1900–2050 và 10 cặp giả ngẫu nhiên với seed 20261102. Các lá số chia sẻ cùng cặp đầu vào nên không phải 64 quan sát thống kê độc lập.

Định nghĩa phương pháp dựa trên [Astrodienst](https://www.astro.com/cgi/h.cgi?f=gch&h=gch65&lang=e). Mã tham chiếu tự ghép Davison, dùng Newton để tìm thời gian corrected, khác thuật toán chia đôi của SDK. Thư viện pyswisseph cài riêng vẫn dùng cùng Swiss Ephemeris nên phép so thời gian/hệ nhà/tốc độ kiểm tra triển khai và tích hợp, không phải mô hình thiên văn hoàn toàn độc lập.

Lớp kiểm độc lập dùng Skyfield và dữ liệu [NASA/JPL DE440s](https://ssd.jpl.nasa.gov/ftp/eph/planets/bsp/de440s.bsp): 10 thiên thể, tọa độ biểu kiến nhìn từ tâm Trái Đất trong hệ hoàng đạo của ngày, tại TT do SDK công bố. Sao Hỏa đến Diêm Vương dùng tâm khối lượng hệ; khác biệt với tâm hành tinh có thể đóng góp sai lệch. Phép so này không kiểm chứng độc lập chuyển UTC sang TT. MC/Asc được tính riêng bằng thời gian sao và độ nghiêng hoàng đạo Skyfield tại UT1 do SDK công bố.

## Kết quả

Tổng **3.648 phép so, 0 vượt ngưỡng**. Ngưỡng kỹ thuật là tiêu chí cho lượt kiểm này, không phải cam kết sai số toàn miền SDK.

| Phép so | Số phép | Sai lệch lớn nhất | Ngưỡng |
|---|---:|---:|---:|
| Thời gian suy ra (giây) | 64 | 0.0002413988113 | 0.005 |
| Kinh độ so thư viện Swiss riêng (độ) | 640 | 3.967281259e-08 | 1e-05 |
| Tốc độ so Swiss riêng (độ/ngày) | 640 | 6.810825752e-07 | 1e-06 |
| Đỉnh nhà so Swiss riêng (độ) | 768 | 1.1354918e-06 | 1e-05 |
| Kinh độ so JPL (giây cung) | 640 | 1.414642248 | 36 |
| Vĩ độ so JPL (giây cung) | 640 | 1.403694943 | 36 |
| MC/Asc so Skyfield (giây cung) | 128 | 0.001305442129 | 3.6 |
| Tọa độ trung bình (độ) | 128 | 1.421085472e-14 | 1e-12 |

## Lưu vết và giới hạn

[Protocol](protocol.json), [kết quả](results/summary.json), [mã chạy](run.py), [phiên bản thư viện](requirements.txt). Đầu vào, phản hồi SDK, kết quả tham chiếu và từng phép so nằm trong thư mục `results`; các tệp NDJSON được nén gzip không mất dữ liệu.

Giữ nguyên hai lượt dừng do lỗi công cụ kiểm: lượt đầu so tọa độ bằng đẳng thức nhị phân nên dừng vì chênh lệch làm tròn ~1e-14 độ; bổ sung ngưỡng tọa độ 1e-12 độ được ghi rõ trong protocol. Mọi ngưỡng thời gian/góc/hành tinh/tốc độ giữ nguyên. Lượt hai hoàn tất tính nhưng dừng khi ghi kiểu boolean của NumPy; sửa ghi dữ liệu rồi chạy lại toàn bộ 64 lá số. Hai thư mục `results-*-aborted` và protocol/script ban đầu được giữ lại.

Chưa phải so trực tiếp số liệu lá số xuất từ Astro.com hoặc một sản phẩm Davison khác. Chưa chứng minh đồng nhất quy ước thời gian giữa mọi phần mềm. Không có người kiểm toán độc lập; cùng trợ lý triển khai và viết kiểm chứng. Chưa bao phủ toàn miền 1800–2399, hệ nhà khác hoặc ma trận gói/nền tảng phát hành. Gói công khai 1.0.2 không thay đổi.

## Chạy lại

```sh
python3 -m venv /tmp/davison-reference-env
/tmp/davison-reference-env/bin/python -m pip install -r docs/reviews/2026-11-02-davison/requirements.txt
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 /tmp/davison-reference-env/bin/python docs/reviews/2026-11-02-davison/run.py \
  --library neutral-engine/target/debug/libastro_engine.dylib \
  --kernel /path/to/de440s.bsp --output /tmp/davison-new-results
python3 docs/reviews/2026-11-02-davison/verify.py
```

`run.py` yêu cầu thư mục kết quả mới; không ghi đè bằng chứng. Build thư viện từ mã nguồn hiện tại trước khi chạy lại. `verify.py` kiểm tra thống kê/hash của bộ kết quả lưu ở đây, không chạy lại mô hình tham chiếu.
