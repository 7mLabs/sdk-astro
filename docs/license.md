# Giấy phép Astro: bản phân phối và quyền MIT cho phần 7mlabs

[English](license.en.md) · [Phạm vi cấp quyền](../LICENSE-SCOPE.md) · [Kiểm kê phạm vi](../LICENSE-SCOPE.json)

## Cần hiểu nhanh

**Bản phân phối native công khai dùng `AGPL-3.0-only`. Phần đóng góp gốc thuộc bản quyền 7mlabs và được xác định trong kiểm kê còn có lựa chọn MIT riêng.** MIT này không biến toàn bộ SDK, Swiss Ephemeris hoặc dữ liệu đi kèm thành MIT.

- **Phần 7mlabs được cấp quyền:** dùng theo [MIT chuẩn](../LICENSE-7MLABS-MIT) và [phạm vi chính xác](../LICENSE-SCOPE.md). Khi dùng lựa chọn này, không có hạn chế AGPL riêng từ 7mlabs đối với đúng phần đóng góp được cấp quyền.
- **Swiss Ephemeris:** giữ giấy phép kép của upstream: AGPL hoặc Swiss Ephemeris Professional. MIT của 7mlabs không cấp thay quyền của Swiss.
- **Phần mềm kết hợp với Swiss:** phải có quyền phù hợp cho tất cả thành phần. Giấy phép MIT của một phần không tự miễn AGPL cho cả binary kết hợp.

**SDK 1.1.3** là lần đầu bộ tệp `LICENSE-7MLABS-MIT`, `LICENSE-SCOPE.md`, `LICENSE-SCOPE.json` và `NOTICE` cập nhật được đóng kèm đồng bộ trong các gói registry cùng corresponding source. Metadata của toàn gói vẫn là `AGPL-3.0-only`. Xem [hồ sơ phát hành 1.1.3](release-1.1.3.md) để phân biệt kiểm tra dự kiến và bằng chứng đã chốt.

Các gói npm/PyPI/NuGet **1.1.2 đã phát hành giữ nguyên bytes, nội dung giấy phép và bằng chứng phát hành**. Quyền MIT được mô tả ở đây là quyền bổ sung cho phần đóng góp được xác định trong revision source có các tệp cấp quyền; không phải tuyên bố các archive 1.1.2 đã được đóng gói lại hay trở thành MIT.

## Bản đồ giấy phép

| Thành phần | Quyền áp dụng |
| --- | --- |
| Phần code gốc thuộc 7mlabs, được xác định trong `LICENSE-SCOPE.json` | Có quyền MIT bổ sung theo `LICENSE-7MLABS-MIT`; phạm vi chính xác trong `LICENSE-SCOPE.md` |
| Phần bổ sung/sửa đổi gốc của 7mlabs trong source Swiss, được xác định bởi diff trong kiểm kê | MIT chỉ cho đúng phần đóng góp thuộc 7mlabs; không cấp MIT cho cả tệp chứa code Swiss |
| Swiss vendored và dữ liệu Chiron nhúng | Điều khoản Swiss; bản phân phối native công khai chọn nhánh AGPL |
| Toàn bộ binary native kết hợp trong bản phân phối công khai | `AGPL-3.0-only`; không có lựa chọn MIT vô điều kiện cho toàn bộ binary |
| Renderer UI độc lập `examples/frontend/ui` | [MIT riêng](../examples/frontend/ui/LICENSE) |
| Thư viện Rust, dữ liệu timezone, font và tài sản bên thứ ba | Giấy phép riêng được giữ trong notices/inventory; xem [NOTICE](../NOTICE) |
| Tên, nhãn hiệu và tài sản thương hiệu | Không tự được cấp quyền bởi MIT cho code; xem phạm vi và notices tương ứng |

Kiểm kê xác định revision, tệp và hash để phân biệt phần được cấp quyền với các thành phần bị loại trừ. Trong tệp Swiss đã sửa, các dòng gốc, dòng bị xóa và phần ngữ cảnh trong diff không trở thành code MIT của 7mlabs. Điều khoản của Swiss đối với toàn bộ tệp và việc sửa/phân phối vẫn áp dụng.

## Hai cách dùng Swiss

### Theo nhánh AGPL

AGPL cho phép cả sử dụng miễn phí lẫn thương mại. Khi phân phối covered work, phải giữ notices và cung cấp corresponding source theo các mục 4–6. Khi sửa chương trình rồi cho người dùng tương tác qua mạng, mục 13 yêu cầu cung cấp cách nhận corresponding source cho những người dùng đó. Phải đánh giá nghĩa vụ của phần mềm kết hợp với engine theo phạm vi áp dụng của AGPL.

Source cần tương ứng với bản thực tế đang phân phối hoặc phục vụ người dùng, gồm các sửa đổi và scripts build cần thiết. Chỉ trỏ về repo upstream không tự đáp ứng nghĩa vụ cho bản đã thay đổi.

### Theo nhánh Swiss Professional

Người có Professional hợp lệ có thể dựa vào quyền đó cho Swiss **trong phạm vi hợp đồng thực tế của họ**, đồng thời dùng quyền MIT cho phần 7mlabs được cấp quyền. 7mlabs không đặt thêm một hạn chế AGPL riêng cho đúng phần đóng góp đã cấp MIT. Tuy nhiên, quyền này không xác nhận hợp đồng Swiss có bao phủ một sản phẩm, dịch vụ, dữ liệu, bản sửa đổi hoặc binary cụ thể hay không.

**Chưa xác nhận rằng các binary native dựng sẵn hiện tại tự động dùng được theo Professional.** Chúng nhúng Swiss, dữ liệu Chiron và source Swiss đã sửa. Cần làm rõ với Swiss cách áp dụng hợp đồng cho tổ hợp đó trước khi quảng bá cam kết này. Tự build lại từ source cũng không thay thế việc có đủ quyền đối với Swiss và các thành phần khác.

Checklist cho người dùng Professional:

1. Kiểm tra hợp đồng Swiss đã ký còn hiệu lực và bao phủ đúng chủ thể, sản phẩm/dịch vụ và cách sử dụng dự định.
2. Xác định revision source, dữ liệu, bản sửa Swiss và binary mình định sử dụng; đối chiếu với `LICENSE-SCOPE.md` và provenance.
3. Làm rõ quyền dùng/phân phối tổ hợp cụ thể với Swiss khi hợp đồng chưa nói rõ, nhất là source đã sửa, binding/port hoặc dịch vụ API cho bên thứ ba.
4. Giữ MIT cho phần 7mlabs, các thông báo Swiss và mọi giấy phép phụ thuộc trong sản phẩm phân phối theo yêu cầu của từng giấy phép.

Hợp đồng Swiss công khai được kiểm tra ngày **12/11/2026** là bản September 2026, cập nhật 9 November 2026. Mục 5 nói về phân phối source và sửa source; mục 13 nói về binding/port và mô hình hai nhánh. Hợp đồng đã ký của mỗi người mới xác định quyền Professional cụ thể của họ; không mặc định các hợp đồng cũ có cùng điều khoản với mẫu hiện hành.

## Đóng gói và tính tương thích

`LICENSE` và metadata của gói native kết hợp tiếp tục ghi `AGPL-3.0-only`; chúng không được đổi thành `MIT` hoặc `AGPL-3.0-only OR MIT` cho cả gói. Từ 1.1.3, các tệp MIT/phạm vi và `NOTICE` cập nhật đi cùng các gói cài đặt và source archive để nhận diện quyền riêng cho phần đóng góp thuộc 7mlabs. Giữ toàn văn AGPL, `NOTICE`, Swiss notices, provenance và giấy phép phụ thuộc đi cùng các gói/source archive mới được chuẩn bị theo chính sách này.

Việc bổ sung quyền không đổi hàm, thuật toán, JSON, ABI hay cách gọi SDK; không yêu cầu người dùng sửa code. Không có cơ chế nhập hoặc kiểm tra khóa Professional trong SDK. Giấy phép là quyền sử dụng; việc chạy được một lệnh không chứng minh đã có đủ quyền.

Lịch sử phát hành và các source snapshot cũ giữ nội dung của thời điểm phát hành. Repo không import source/history ứng dụng legacy; [tài liệu legacy](legacy-source.md) chỉ mô tả lịch sử. Không suy rộng quyền MIT sang đóng góp tương lai hoặc code không nằm trong phạm vi đã cấp quyền.

## Nguồn và tệp có hiệu lực

- [Phạm vi cấp quyền 7mlabs](../LICENSE-SCOPE.md), [MIT chuẩn](../LICENSE-7MLABS-MIT), [kiểm kê](../LICENSE-SCOPE.json), [AGPL đầy đủ](../LICENSE) và [NOTICE](../NOTICE).
- [Swiss license tại commit đã pin](https://github.com/aloistr/swisseph/blob/aacf962d19d79f8bc921dbcdaacf306d85be1917/LICENSE) và [provenance của provider](../neutral-engine/crates/astro-provider-swiss/PROVENANCE.md).
- [Hướng dẫn cấp phép Swiss](https://www.astro.com/swisseph/swephinfo_e.htm) và [mẫu hợp đồng Professional hiện hành](https://www.astro.com/swisseph/secont_e_2609.pdf).
- [GNU AGPL v3, nhất là mục 5 và 7](https://www.gnu.org/licenses/agpl-3.0.html), [GNU: tương thích và cấp lại giấy phép](https://www.gnu.org/licenses/license-compatibility.en.html).

Trang này giải thích cách đọc các tệp cấp quyền. Phạm vi và quyền chính xác do các tệp giấy phép, quyền của từng chủ sở hữu và hợp đồng áp dụng quyết định; trang này không mở rộng quyền của Swiss.
