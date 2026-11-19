# Bản đồ sao câu hỏi Horary

[English](horary.en.md) · [Schema đầu vào](../schemas/horary-request.schema.json) · [Schema kết quả](../schemas/horary-response.schema.json)

Từ SDK **1.2.0**, `calculate` nhận thời điểm và địa điểm lập lá số cho câu hỏi. Người gọi chọn `quesitedHouse`, số nhà đại diện cho vấn đề được hỏi, từ 1 đến 12. Kết quả gồm các vị trí thiên văn, nhà, hành tinh đại diện và chỉ báo Void-of-Course Moon để ứng dụng tự diễn giải.

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

## Bộ quy tắc

- Khi bỏ `ruleset`, SDK dùng `western-traditional-v1`. ID này cũng dùng để yêu cầu tường minh đúng bộ quy tắc về sau. SDK trả lỗi nếu ID không được hỗ trợ. `calculation.ruleset` và `calculation.rulesetVersion` luôn cho biết bộ quy tắc đã chạy.
- Lá số tropical, địa tâm. `moment.houseSystem` mặc định `regiomontanus`; có thể chọn `placidus` hoặc `wholeSign`. Hệ nhà thực tế nằm ở `calculation.houseSystem`.
- `data.chart` trả bảy thiên thể từ Mặt Trời đến Sao Thổ, bốn góc trục và mười hai nhà. Năm góc chiếu giữa các thiên thể là 0°, 60°, 90°, 120°, 180°, với orb tương ứng 8°, 6°, 8°, 8°, 8°. Trường `applying` mô tả xu hướng tại thời điểm lập lá số.
- `data.voidOfCourseMoon` dùng phương pháp `exactMajorAspectBeforeNextSignIngress`. SDK tìm lần đầu Mặt Trăng đạt chính xác một trong năm góc chiếu với Mặt Trời, Sao Thủy, Sao Kim, Sao Hỏa, Sao Mộc hoặc Sao Thổ, sau thời điểm câu hỏi và trước lần Mặt Trăng đổi cung tiếp theo. Nếu có, `nextAspect` ghi hành tinh, góc và thời điểm; nếu không, `nextAspect` là `null` và `isVoidOfCourse` là `true`. `nextIngress` luôn ghi cung kế tiếp và thời điểm nhập cung. Mỗi sự kiện có UTC và ngày Julius UT1. Góc hoàn tất cách thời điểm lập lá số hoặc nhập cung không quá 1 mili giây được tính là trùng mốc và bị loại. `western-traditional-v1` là bộ chỉ báo được định nghĩa tại đây; kết luận cho câu hỏi thuộc người đọc lá số.

Ở vùng cực, có lúc các nhà Regiomontanus từ 1 đến 12 đi theo chiều kinh độ giảm. SDK giữ nguyên đỉnh nhà do Swiss Ephemeris tính và dùng chiều đó để gán hành tinh, xác định chủ tinh và vẽ lá số. [Swiss Ephemeris, mục 6.4](https://www.astro.com/swisseph/swisseph.htm)

`calculation.houseOrder` trả `decreasingLongitude` cho trường hợp này và `increasingLongitude` cho chiều thông thường. Mảng `houseCusps` luôn xếp theo số nhà 1–12. Một hành tinh nằm đúng đỉnh nhà được tính vào nhà bắt đầu tại đỉnh đó. Trong mô hình hiển thị, `width` là độ rộng dương và `signedWidth` là độ rộng có dấu theo chiều kinh độ. Nếu các đỉnh trùng nhau đến mức không xác định được đủ 12 nhà riêng biệt, SDK trả lỗi tính toán. Đầu vào yêu cầu vĩ độ lớn hơn −90° và nhỏ hơn 90°. [Ví dụ Longyearbyen](../examples/horary-polar-request.json)

Nếu đầu vào là giờ địa phương, dùng `calculateFromLocal` với `moment.local`, `moment.location` và một trong hai trường `moment.timeZone` hoặc `moment.utcOffsetSeconds`. Kết quả ghi cách đổi giờ ở `calculation.timeResolution.moment`. [Quy tắc giờ địa phương](time-and-svg.md)

`renderChartSvg(result)` vẽ lá số từ chính kết quả trên. Để làm nổi bật nhà 7 trong lá số câu hỏi (lớp `Q`), dùng `renderChartSvg(result, { chart: 'horary', focus: { house: 7, layer: 'Q' } })`. `buildChartModel(result)` trả dữ liệu cho giao diện tùy chỉnh. [Mẫu đầu vào JSON](../examples/horary-request.json)

Để đối chiếu các cách hiểu khác nhau về Void-of-Course, xem [Deborah Houlding, The Moon as the transmitter of influences](https://www.skyscript.co.uk/moon2.html).
