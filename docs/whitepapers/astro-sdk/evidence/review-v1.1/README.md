# Hồ sơ thực thi lượt rà soát 1.1

Ngày nghiên cứu 22/10/2026. Mã sản phẩm tại cd768d0192cd7e6f7aff266b8ebdb3f8685eeb39. Không sửa mã sản phẩm.

## Tái thực hiện
Chạy từ root repository. Cần Rust/C toolchain và Python 3 có standard library. Hai lockfiles được giữ nguyên. Build có thể cần tải dependencies nếu cache chưa có; tính/nén trong phép đo dùng binary local.

```sh
cargo test --manifest-path neutral-engine/Cargo.toml --workspace --locked > docs/whitepapers/astro-sdk/evidence/review-v1.1/cargo-test.log 2>&1
cargo build --manifest-path examples/rust/Cargo.toml --locked > docs/whitepapers/astro-sdk/evidence/review-v1.1/build-cli.log 2>&1
python3 docs/whitepapers/astro-sdk/evidence/review-v1.1/recheck.py > docs/whitepapers/astro-sdk/evidence/review-v1.1/analysis.log
```

Chỉ chạy bước sau khi bước trước exit code 0. Nếu lưu kết quả hiện tại, sao chép thư mục hồ sơ trước: script ghi lại results.json và artifacts cùng tên. Lượt thực hiện của tài liệu có cả ba bước exit 0; 132 test passed, 0 failed, 0 ignored.

## Nội dung
- protocol.md: thay đổi phương pháp đã ghi trước khi chạy lại, sau khi biết kết quả v1.0.
- recheck.py: truyền JSON nguyên văn; Decimal và comparator có phân biệt kiểu; ca âm kiểm tra công cụ.
- results.json: môi trường, hash inputs/binary/lockfiles/script/log, số byte, kết quả so sánh, từng sai lệch body và tổng.
- *.context.json.gz và *.expanded.json.gz: nguyên đầu ra native không có newline CLI, lưu gzip mtime=0. Hash trong results.json áp dụng dữ liệu sau giải gzip.
- natal-0.json đến natal-3.json: kết quả natal mới; nguồn so sánh vẫn là tests/conformance/natal-references.json.
- normalization-and-comparison.json: so sánh với v1.0, ví dụ cách viết số và chênh lệch 44 byte. Là phân tích bổ sung từ đầu ra đã lưu; không dùng như benchmark gzip.

Không chạy Skyfield/JPL độc lập, không kiểm ma trận binding hay đo token/hiệu năng. Rust test suite có kiểm thử SVG nhưng chưa kiểm UI trên trình duyệt trong lượt này. Mỗi fixture có một lượt nén/giải nén trong lần chạy này. Các ca comparator là kiểm tra công cụ, không tính thêm vào 132 test sản phẩm.
