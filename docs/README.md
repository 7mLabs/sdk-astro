<a href="../README.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs"></a>

# Tài liệu Astro SDK 1.1.2

[README tiếng Việt](../README.md) · [English README](../README.en.md) · [Tài liệu web](https://7mlabs.com/astro/sdk/docs/1.1.2/overview/)

## Bắt đầu từ đây

| Bạn cần | Tiếng Việt | English |
| --- | --- | --- |
| Biết chức năng, công thức và payload | [Chức năng và công thức](features-and-formulas.vi.md) | [Features and formulas](features-and-formulas.md) |
| Biết cấu hình, mặc định và giới hạn | [Quy tắc tính](rules.vi.md) | [Calculation rules](rules.md) |
| Xem nguồn và phạm vi đã kiểm chứng | [Nguồn và kiểm chứng](formula-verification.vi.md) | [Sources and verification](formula-verification.md) |

## Contract kỹ thuật

- [API](api.md), [natal](natal.md), [domains](domains.md), [custom profiles](profiles.md), [facts/reports](individual-reports.md).
- [Couple](couple.md), [composite](composite.md), [Davison](davison.md), [events/forecast](forecast.md).
- [Metadata payload](payload-metadata.md), [query](query.md), [đổi giờ và SVG](time-and-svg.md), [nén context](payload-compression.md).
- [Node](../bindings/node/README.md), [Python](../bindings/python/README.md), [.NET](../bindings/dotnet/README.md), [kiến trúc](architecture.md), [tích hợp](integrations.md), [kiểm thử](testing.md).

## Ví dụ và phát hành

[Request natal](examples/natal-request.json) · [Response thật 1.1.2](examples/natal-response.json) · [Geometry](examples/geometry-request.json) · [Báo cáo và checksum 1.1.2](release-1.1.2.md) · [Lịch sử thay đổi](../CHANGELOG.md).

Các tệp release/review mô tả đúng thời điểm của chúng. Các số alpha/phiên bản cũ trong ghi chú lịch sử không xác định version SDK hiện hành. Các contract dùng lại trong 1.1.2 không có nghĩa mọi API đều được thêm mới trong bản này. Các bộ quy tắc riêng giữ ID/version trong payload; tài liệu không tự nâng chúng.
