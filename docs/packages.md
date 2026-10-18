# Cài package và chạy source mẫu

npm `@7mlabs/astrology`, PyPI `sevenmlabs-astrology` và NuGet `SevenMLabs.Astrology` đã public **1.0.0**; cả ba đã được tải về xác minh và cài thử mới. Xem [phạm vi nền tảng và bằng chứng release](release-1.0.md). Artifacts local từ builder chỉ chứa binary của host; bản npm/NuGet cuối ghép Mac ARM64 và Linux x64.

## Build từ repo

Yêu cầu máy build: Rust 1.83+, Node 18+/npm, Python 3.10+ với setuptools/wheel, .NET 10 SDK và C compiler. Clone repo, chạy tại root:

```bash
python3 -m venv artifacts/python-build
artifacts/python-build/bin/python -m pip install setuptools wheel
artifacts/python-build/bin/python scripts/build-packages.py
artifacts/python-build/bin/python scripts/test-packages.py
```

`--offline` của builder dành cho máy đã có Cargo dependency cache. Các dependency phải được tải trước lần build đầu; runtime và việc cài local artifact không cần mạng. Không yêu cầu user bình thường cài compiler sau khi public prebuilt package.

Nếu Node/npm không nằm trên PATH, set `NODE_BIN` và `NPM_CLI` đến executable/script cụ thể. `scripts/test-packages.py` chỉ dùng tgz/wheel/nupkg vừa build, tắt registry access và tạo consumer dưới `artifacts/consumers`.

## Node.js

```bash
npm install @7mlabs/astrology
node sample.cjs
```

Cập nhật kênh ổn định tường minh bằng `npm install @7mlabs/astrology@latest`. Để pin version, dùng `npm install --save-exact @7mlabs/astrology@1.0.0` và commit lockfile; `npm ci` cài lại theo lockfile.

Package npm chứa sẵn addon cho macOS ARM64 và Linux x64/glibc 2.38+. Tarball ổn định đã qua đủ bốn consumer jobs trên hai target với Node 18/24, rồi được xác minh checksum/integrity và cài mới từ registry trên macOS ARM64; Windows, macOS Intel, Linux ARM64 và Alpine/musl chưa hỗ trợ. Consumer không cần Rust/compiler/server hoặc tải binary sau cài đặt. Xem [nền tảng và release evidence](node-release.md). Muốn thử artifact local, thay tên package trong lệnh bằng đường dẫn tuyệt đối tới `.tgz`.

```js
const { calculate } = require('@7mlabs/astrology');
const result = calculate({ operation: 'natal',
  utc: { year: 2000, month: 1, day: 1, hour: 12, minute: 0 },
  location: { latitude: 10.8231, longitude: 106.6297 }
});
console.log(result.data.placements, result.data.houses);
```

Package có TypeScript declarations. `calculate` synchronous, dùng cho natal/geometry bounded. Scans tháng/năm nên được gọi trong application worker để giữ event loop phản hồi. ES module có thể dùng default import từ CommonJS. Native package chưa chạy trong browser.

## Python

```bash
python3 -m venv .venv
.venv/bin/python -m pip install sevenmlabs-astrology==1.0.0
.venv/bin/python sample.py
```

```python
from sevenmlabs_astrology import calculate
result = calculate({"operation": "natal",
    "utc": {"year": 2000, "month": 1, "day": 1, "hour": 12, "minute": 0},
    "location": {"latitude": 10.8231, "longitude": 106.6297}
})
print(result["data"]["placements"])
```

Wheel public 1.0.0 chỉ hỗ trợ macOS ARM64, Python 3.10+; Linux Python cần build/kiểm chứng riêng. Khi cài artifact local, dùng filename wheel theo đúng host. ctypes wrapper tự giải phóng output Rust trong finally.

## .NET

Thêm SDK vào ứng dụng .NET 10 bằng `dotnet add package SevenMLabs.Astrology --version 1.0.0`. Package gồm Mac ARM64 và Linux x64/glibc 2.38+; không có Windows native binary. Sample `examples/dotnet/Example.csproj` đã có PackageReference đúng tên/version:

```bash
dotnet restore examples/dotnet/Example.csproj --source https://api.nuget.org/v3/index.json
dotnet run --project examples/dotnet/Example.csproj --no-restore
```

```csharp
using SevenMLabs.Astrology;
var result = Engine.Calculate(new {
    operation = "natal",
    utc = new { year = 2000, month = 1, day = 1, hour = 12, minute = 0 },
    location = new { latitude = 10.8231, longitude = 106.6297 }
});
Console.WriteLine(result);
```

Managed sample hiện target net10.0. NuGet native library phải xuất hiện trong output consumer qua assets theo RID. Không trỏ DllImport vào source directory hoặc cần chạy Python service.

## Rust và C

```bash
cargo run --manifest-path examples/rust/Cargo.toml
```

Rust sample dùng path dependency core; chưa kiểm tra cargo install từ crates.io. C sample nằm ở `examples/c/main.c`, dùng header và link native library. Script conformance compile và chạy sample này. Binary standalone hoặc publish crate sẽ là phase release sau.

## Contract và chạy sample khác

Tất cả default samples đã dùng natal request thật. Với Node/Python/.NET/Rust/C, argument đầu tiên có thể là raw JSON request để trả nguyên envelope. Xem `examples/natal-request.json`, `examples/natal-result.json`, [natal docs](natal.md) và [JSON API](api.md). Runtime không cần data files, vì Moshier/time tables được compile vào native library. Từng artifact có notices Swiss trong `third-party/`.

## Payload theo khía cạnh

`operation: "natalDomains"` gọi cùng SDK `calculate`/`Engine.Calculate`; input UTC/location không đổi. Thêm `domains`, `rulership`, `aspectPreset` hoặc custom `aspectRules`. Xem [domain contract](domains.md), `examples/natal-domains-request.json` và `examples/natal-domains-result.json`.

Samples mới dùng package đã cài: `node domains.cjs`, `python domains.py`, `.NET Example --domains`. Node/Python sample files nằm trong `examples/node/domains.cjs`, `examples/python/domains.py`; .NET `Program.cs` có option `--domains`. Rust/C nhận raw JSON như sample natal trước.

## Lá số cặp đôi

`operation: "couple"` dùng cùng API cục bộ. Mỗi `personA`/`personB` có UTC/location/houseSystem riêng; các options `domains`, `customProfiles`, `rulership`, `aspectPreset`/`aspectRules` đặt ở request gốc. Xem [couple contract](couple.md) và [request mẫu](../examples/couple-request.json).

Chạy dedicated examples từ consumer đã cài: `node couple.cjs`, `python couple.py`, `.NET Example --couple`. [Node](../examples/node/couple.cjs), [Python](../examples/python/couple.py) và [.NET](../examples/dotnet/Program.cs) dùng cùng request. Rust/C nhận raw JSON từ request file; không cần thêm package/provider riêng cho cặp đôi.

## Lá số thứ ba composite

Dùng `operation: "composite"`, `personA` và `personB` để trả C với mặc định 10 lĩnh vực/30 sections. Hoặc thêm `composite: {}` vào `couple` để tính cùng synastry. Kết quả C nằm ở `data.composite`; riêng C dùng catalog giống lá số đơn. Chi tiết options/metadata và code đủ năm ngôn ngữ trong [composite.md](composite.md).

Examples từ consumer đã cài: `node composite.cjs`, `python composite.py`, `.NET Example --composite`; dạng kết hợp: `node couple-composite.cjs`, `python couple_composite.py`, `.NET Example --coupleComposite`. Rust/C truyền raw JSON của [request composite](../examples/composite-request.json) hoặc [request kết hợp](../examples/couple-composite-request.json). Không cài thêm provider/package cho C.

## Ngày, tháng, năm và sự kiện

`forecast` nhận `birth` cùng `period` day/month/year và trả messageContext/overview/events/personal impacts từ cùng package; `events` chỉ cần period để quét sky events. Không cài thêm package hoặc duy trì server. Xem [forecast contract](forecast.md), [day request](../examples/forecast-day-request.json), [month request](../examples/forecast-month-request.json), [year natal request](../examples/forecast-year-request.json), [year sky request](../examples/events-year-request.json).

Dedicated examples: `node forecast.cjs`, `python forecast.py`, `.NET Example --forecast`. Rust/C và các default sample CLIs nhận raw JSON như trước. Native calculate synchronous; dùng worker cho scans dài trong ứng dụng có UI/event loop.

## Các registry còn lại

PyPI/NuGet 1.0.0 đã phát hành, xác minh payload và cài mới thành công. Python public wheel chỉ dành cho Mac ARM64; .NET public package cần .NET 10 và host Mac ARM64/Linux x64. Rust crates.io chưa phát hành. SDK tính cục bộ, không cần credential hoặc tải ephemeris lúc import.

## Bộ nén payload riêng

Dòng version `0.10` thêm `compressPayload`, `expandContext`, `calculateWithContext` trong Node; API tương ứng dạng snake_case trong Python và `Engine.CompressPayload/ExpandContext/CalculateWithContext` trong .NET. Rust/C dùng cùng thuật toán core. Xem [contract và ví dụ](payload-compression.md). Chạy `scripts/test-compression.py` sau kiểm thử cài package, và `scripts/check-compression-bindings.py` để xác minh helper/TypeScript trên SDK đã cài.
