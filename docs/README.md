<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs"></a>

# Tài liệu Astro SDK 1.1.3

[README tiếng Việt](../README.md) · [English README](../README.en.md) · [Tài liệu web](https://7mlabs.com/astro/sdk/docs/1.1.3/overview/)

## Bắt đầu từ đây

| Bạn cần | Tiếng Việt | English |
| --- | --- | --- |
| Biết chức năng, công thức và payload | [Chức năng và công thức](features-and-formulas.vi.md) | [Features and formulas](features-and-formulas.md) |
| Biết cấu hình, mặc định và giới hạn | [Quy tắc tính](rules.vi.md) | [Calculation rules](rules.md) |
| Xem nguồn và phạm vi đã kiểm chứng | [Nguồn và kiểm chứng](formula-verification.vi.md) | [Sources and verification](formula-verification.md) |
| Phân biệt bản phân phối AGPL, quyền MIT phần 7mlabs và Swiss Professional | [Giấy phép và phạm vi](license.md) | [Licensing and scope](license.en.md) |

## Contract kỹ thuật

- [API](api.md), [natal](natal.md), [domains](domains.md), [custom profiles](profiles.md), [facts/reports](individual-reports.md).
- [Couple](couple.md), [composite](composite.md), [Davison](davison.md), [events/forecast](forecast.md).
- [Metadata payload](payload-metadata.md), [query](query.md), [đổi giờ và SVG](time-and-svg.md), [nén context](payload-compression.md).
- [Node](../bindings/node/README.md), [Python](../bindings/python/README.md), [.NET](../bindings/dotnet/README.md), [kiến trúc](architecture.md), [tích hợp](integrations.md), [kiểm thử](testing.md).

## Ví dụ và phát hành

[Request natal](examples/natal-request.json) · [Response thật 1.1.3](examples/natal-response.json) · [Geometry](examples/geometry-request.json) · [Hồ sơ phát hành 1.1.3](release-1.1.3.md) · [Báo cáo và checksum lịch sử 1.1.2](release-1.1.2.md) · [Lịch sử thay đổi](../CHANGELOG.md).

Các tệp release/review mô tả đúng thời điểm của chúng. Các số alpha/phiên bản cũ trong ghi chú lịch sử không xác định version SDK hiện hành. Các contract dùng lại trong 1.1.3 không có nghĩa mọi API đều được thêm mới trong bản này. Các bộ quy tắc riêng giữ ID/version trong payload; tài liệu không tự nâng chúng.

## Giấy phép theo thành phần

1.1.3 là lần đầu gói registry đóng kèm bộ tệp MIT/phạm vi và `NOTICE` cập nhật. Bản phân phối native công khai giữ `AGPL-3.0-only`. Các phần đóng góp được kiểm kê còn có [MIT cho phần đóng góp thuộc 7mlabs](../LICENSE-7MLABS-MIT), với [phạm vi](../LICENSE-SCOPE.md) và [kiểm kê](../LICENSE-SCOPE.json) cụ thể. Swiss và các phụ thuộc giữ quyền riêng; không có tuyên bố cả binary là MIT hoặc mọi binary dựng sẵn tự động dùng được theo Professional. Các archive 1.1.2 đã phát hành giữ nguyên. Đọc [hướng dẫn tiếng Việt](license.md) hoặc [English](license.en.md) trước khi chọn cách tích hợp.
