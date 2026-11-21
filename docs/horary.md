# Bản đồ sao câu hỏi Horary

[English](horary.en.md) · [Schema đầu vào](../schemas/horary-request.schema.json) · [Schema kết quả](../schemas/horary-response.schema.json)

Từ SDK **1.2.0**, `calculate` nhận thời điểm và địa điểm lập lá số cho câu hỏi. Người gọi chọn `quesitedHouse`, số nhà đại diện cho vấn đề được hỏi, từ 1 đến 12. Kết quả gồm các vị trí thiên văn, nhà, hành tinh đại diện và chỉ báo Void-of-Course Moon để ứng dụng tự diễn giải.

## Chọn bộ quy tắc theo mục đích

| Ứng dụng cần gì? | Chọn `ruleset` | Cách dùng kết quả |
| --- | --- | --- |
| Tái tạo lá số từ SDK 1.2.0–1.2.1 hoặc tra góc theo orb cố định | `western-traditional-v1` (mặc định) | Dùng `nextAspect` để trả lời “Mặt Trăng sẽ hoàn tất góc chính nào trước khi đổi cung?”. Giá trị `null` nghĩa là không có góc nào thỏa điều kiện trong khoảng đó. |
| Đọc lá số theo các quy tắc Lilly đã được SDK triển khai | `lilly-1647-v1` | Chọn ID này khi lập lá số để SDK áp dụng bảng orb theo hành tinh và điều kiện tiếp xúc của Mặt Trăng. [Nguồn và phạm vi](release-1.2.2.md) |

`western-traditional-v1` là tên bộ quy ước tính do 7mlabs định nghĩa. SDK không quy toàn bộ tổ hợp này cho một tác giả hay trường phái lịch sử cụ thể. Nếu đưa vào giao diện, hãy ghi **Quy ước SDK 1.2.0–1.2.1** ở phần tùy chọn tính toán. Trong danh sách trường phái, mục **William Lilly** tương ứng với `lilly-1647-v1`.

Khi đọc lại lá số đã lưu, xem `calculation.ruleset` để biết các chỉ báo mang nghĩa gì. Muốn đọc lá số cũ theo Lilly, hãy tính lại từ cùng thời điểm, địa điểm và nhà của vấn đề được hỏi với `lilly-1647-v1` và hệ nhà Regiomontanus. Giá trị `isVoidOfCourse` mô tả trạng thái Mặt Trăng theo quy ước đã chọn. Việc trả lời “có” hay “không” cho câu hỏi đời thực thuộc phần luận của ứng dụng.

## Ví dụ dùng quy ước SDK

```js
const { calculate } = require('@7mlabs/astrology');

const result = calculate({
  operation: 'horary',
  moment: {
    utc: { year: 2026, month: 11, day: 19, hour: 8, minute: 0 },
    location: { latitude: 10.8231, longitude: 106.6297 }
  },
  quesitedHouse: 7,
  ruleset: 'western-traditional-v1'
});

const { querent, quesited } = result.data.significators;
console.log(querent.rulerBodyId, quesited.rulerBodyId);
console.log(result.data.voidOfCourseMoon.nextAspect);
```

Ở ví dụ này, nhà 1 đại diện cho người hỏi, còn nhà 7 là lựa chọn của người gọi cho đối tượng được hỏi. Chủ tinh mỗi nhà là hành tinh cai quản cung tại đỉnh nhà theo bảng truyền thống. Hai vai trò có thể cùng một chủ tinh. Mặt Trăng được ghi riêng với ID `moon`.

Với đầu vào trên, đỉnh nhà 1 ở Bạch Dương nên chủ tinh là Sao Hỏa, còn đỉnh nhà 7 ở Thiên Bình nên chủ tinh là Sao Kim. Mặt Trăng đạt góc 120° với Mặt Trời lúc khoảng 01:46 UTC ngày 20/11/2026, trước khi vào Bạch Dương khoảng 05:52 UTC cùng ngày. Vì thế chỉ báo `isVoidOfCourse` có giá trị `false`.

## Tùy chọn Lilly từ bản 1.2.2

Truyền `ruleset: 'lilly-1647-v1'` để dùng Regiomontanus và orb bằng nửa tổng orb của hai hành tinh theo cột đầu bảng Lilly. SDK trả khoảng cách tới đỉnh nhà kế tiếp nếu nằm trong 5°. Chỉ báo Mặt Trăng xét cả góc đang tách còn trong orb lẫn góc đang tiến tới trong orb, kể cả khi góc chính xác sau lúc đổi cung. Khi chỉ có góc đang tiến tới, trường `applicationWithoutSeparation` cho biết tiếp xúc đó. Ứng dụng chọn cách luận từ các dữ kiện này. [Bảng orb, quy ước và lá số đối chiếu](release-1.2.2.md)

## Chi tiết của bộ mặc định `western-traditional-v1`

- Khi bỏ `ruleset`, SDK dùng `western-traditional-v1`. ID này cũng dùng để yêu cầu tường minh đúng bộ quy tắc về sau. SDK trả lỗi nếu ID không được hỗ trợ. `calculation.ruleset` và `calculation.rulesetVersion` luôn cho biết bộ quy tắc đã chạy.
- Lá số tropical, địa tâm. `moment.houseSystem` mặc định `regiomontanus`; có thể chọn `placidus` hoặc `wholeSign`. Hệ nhà thực tế nằm ở `calculation.houseSystem`.
- `data.chart` trả bảy thiên thể từ Mặt Trời đến Sao Thổ, bốn góc trục và mười hai nhà. Năm góc chiếu giữa các thiên thể là 0°, 60°, 90°, 120°, 180°, với orb tương ứng 8°, 6°, 8°, 8°, 8°. Trường `applying` mô tả xu hướng tại thời điểm lập lá số.
- `data.voidOfCourseMoon` dùng phương pháp `exactMajorAspectBeforeNextSignIngress`. SDK tìm lần đầu Mặt Trăng đạt chính xác một trong năm góc chiếu với Mặt Trời, Sao Thủy, Sao Kim, Sao Hỏa, Sao Mộc hoặc Sao Thổ, sau thời điểm câu hỏi và trước lần Mặt Trăng đổi cung tiếp theo. Nếu có, `nextAspect` ghi hành tinh, góc và thời điểm; nếu không, `nextAspect` là `null` và `isVoidOfCourse` là `true`. `nextIngress` luôn ghi cung kế tiếp và thời điểm nhập cung. Mỗi sự kiện có UTC và ngày Julius UT1. Góc hoàn tất cách thời điểm lập lá số hoặc nhập cung không quá 1 mili giây được tính là trùng mốc và bị loại. `western-traditional-v1` là bộ chỉ báo được định nghĩa tại đây; kết luận cho câu hỏi thuộc người đọc lá số.

Ở vùng cực, có lúc các nhà Regiomontanus từ 1 đến 12 đi theo chiều kinh độ giảm. SDK giữ nguyên đỉnh nhà do Swiss Ephemeris tính và dùng chiều đó để gán hành tinh, xác định chủ tinh và vẽ lá số. [Swiss Ephemeris, mục 6.4](https://www.astro.com/swisseph/swisseph.htm)

`calculation.houseOrder` trả `decreasingLongitude` cho trường hợp này và `increasingLongitude` cho chiều thông thường. Mảng `houseCusps` luôn xếp theo số nhà 1–12. Một hành tinh nằm đúng đỉnh nhà được tính vào nhà bắt đầu tại đỉnh đó. Trong mô hình hiển thị, `width` là độ rộng dương và `signedWidth` là độ rộng có dấu theo chiều kinh độ. SDK trả lỗi khi cả 12 đỉnh cùng hội tụ về hai đầu của một trục trong sai số 0,0000001°. Các nhà riêng lẻ rất hẹp vẫn được hỗ trợ. Đầu vào yêu cầu vĩ độ lớn hơn −90° và nhỏ hơn 90°. [Ví dụ Longyearbyen](../examples/horary-polar-request.json)

Nếu đầu vào là giờ địa phương, dùng `calculateFromLocal` với `moment.local`, `moment.location` và một trong hai trường `moment.timeZone` hoặc `moment.utcOffsetSeconds`. Kết quả ghi cách đổi giờ ở `calculation.timeResolution.moment`. [Quy tắc giờ địa phương](time-and-svg.md)

`renderChartSvg(result)` vẽ lá số từ chính kết quả trên. Để làm nổi bật nhà 7 trong lá số câu hỏi (lớp `Q`), dùng `renderChartSvg(result, { chart: 'horary', focus: { house: 7, layer: 'Q' } })`. `buildChartModel(result)` trả dữ liệu cho giao diện tùy chỉnh. [Mẫu đầu vào JSON](../examples/horary-request.json)

Để đối chiếu các cách hiểu khác nhau về Void-of-Course, xem [Deborah Houlding, The Moon as the transmitter of influences](https://www.skyscript.co.uk/moon2.html).

## Hướng phát triển sau 1.2.2

### Quy ước của SDK · `western-traditional-v1`

Bộ này phục vụ hai việc: tái tạo kết quả đã lưu và cung cấp một phép tra cứu độc lập cho ứng dụng dùng orb cố định. Ví dụ, lịch chiêm tinh có thể hiển thị góc chính tiếp theo của Mặt Trăng trước lúc đổi cung, cùng thời điểm chính xác. Ứng dụng đó có thể dùng bộ mặc định ngay cả khi người dùng chưa từng lập lá số bằng bản cũ.

Hướng nâng cấp đề xuất là sửa các lỗi sai với đặc tả, cải thiện tốc độ và kiểm thử ở ranh giới đổi cung. Nếu có nhu cầu xem toàn bộ các góc trước khi đổi cung, có thể thêm một truy vấn tùy chọn. Đổi ngưỡng orb hay đổi định nghĩa vô hành là thay đổi phương pháp và cần ID quy tắc mới. Nếu sau này quyết định ngừng hỗ trợ, cần chỉ rõ cách đọc kết quả đã lưu và cách chuyển các lời gọi đang bỏ trống `ruleset` trước khi gỡ ID.

### William Lilly · `lilly-1647-v1`

**Đã phát hành trong 1.2.2.** SDK lập lá số Regiomontanus, xác định chủ tinh cho nhà của người hỏi và nhà của đối tượng do ứng dụng chọn, xét năm góc chính theo orb của từng cặp hành tinh, trả khoảng cách đến đỉnh nhà kế tiếp trong 5° và tính chỉ báo Mặt Trăng theo quy ước đã công bố. [Nguồn, các lựa chọn khi chuyển quy tắc thành phép tính và ca đối chiếu](release-1.2.2.md)

| Mốc đề xuất | Dữ kiện mà ứng dụng sẽ nhận | Cách kiểm tra trước khi phát hành |
| --- | --- | --- |
| 1.3.0 · Phẩm chất, tiếp nhận và nhà phái sinh | Phẩm chất hành tinh theo cung, độ và ngày/đêm khi quy tắc yêu cầu; hành tinh nào tiếp nhận hành tinh nào, theo phẩm chất nào, có tiếp nhận lẫn nhau hay không. Với nhà phái sinh, ứng dụng chọn vai trò và nhà gốc; SDK đếm nhà, trả chủ tinh và chỉ rõ khi hai vai trò trùng chủ tinh. | Đối chiếu bảng phẩm chất trang 104 của Lilly; thử hai phía của ranh giới độ, lá số ngày/đêm, tiếp nhận một chiều/hai chiều và trường hợp đếm nhà qua nhà 12. [Bản chụp bảng của Lilly](https://www.skyscript.co.uk/ca_downloads.html) |
| 1.4.0 · Diễn biến và sự hoàn tất góc (*perfection*) | Với các hành tinh đại diện đã chọn, trả các góc vừa hoàn tất và thứ tự tiếp xúc sắp tới trong khoảng khảo sát. Từ chuỗi đó, nhận diện góc hoàn tất trực tiếp, truyền sáng (*translation*), tụ sáng (*collection*) và trường hợp bị ngăn hoặc gián đoạn (*prohibition*, *frustration*, *refranation*) theo từng điều kiện đã đối chiếu. | Mỗi cơ chế cần nguồn, ví dụ thỏa điều kiện và ví dụ gần giống nhưng thiếu điều kiện. Kiểm tra đổi cung, đổi chiều và trường hợp hết khoảng khảo sát khi góc còn đang tiến tới. [William Lilly, Christian Astrology](https://lib.wildla.in/books/Christian%20Astrology%201-2%20by%20William%20Lilly.pdf) |

Hai mốc trên là đề xuất cho phần dữ kiện tính được của hệ Lilly. Khi ra mắt một chỉ báo, tài liệu cần cho biết căn cứ, lựa chọn của SDK ở điểm có nhiều cách hiểu và dữ liệu nào cho phép ứng dụng kiểm tra kết quả. Ứng dụng chọn đối tượng của câu hỏi và cân nhắc các chỉ báo để đưa ra lời luận.
