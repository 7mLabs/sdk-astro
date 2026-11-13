# GitHub công khai và phân phối package

Repo công khai là [7mLabs/sdk-astro](https://github.com/7mLabs/sdk-astro). npm `@7mlabs/astrology`, PyPI `sevenmlabs-astrology` và NuGet `SevenMLabs.Astrology` dùng cùng version mục tiêu **1.1.3**. Đây là bản đóng gói giấy phép, không thêm thuật toán hay target native. Trạng thái phát hành, checksum và kiểm tra cài mới của đợt này được ghi riêng trong [hồ sơ 1.1.3](release-1.1.3.md). Không dùng bằng chứng của bản cũ để suy ra đã phát hành hoặc kiểm chứng 1.1.3. Rust/C tích hợp từ source; UI renderer có vòng version riêng. [Thiết lập GitHub](github-setup.md).

## Phạm vi quyền bổ sung và gói 1.1.3

Bản phân phối native công khai tiếp tục theo `AGPL-3.0-only`. Phần đóng góp gốc thuộc 7mlabs được xác định trong [kiểm kê](../LICENSE-SCOPE.json) còn có [quyền MIT riêng](../LICENSE-7MLABS-MIT) theo [phạm vi chính xác](../LICENSE-SCOPE.md). Quyền này không cấp MIT cho Swiss, dữ liệu/font/phụ thuộc bên thứ ba hoặc toàn bộ binary kết hợp. Người dùng Professional phải có quyền Swiss phù hợp cho chính cách dùng của họ; chưa xác nhận các binary dựng sẵn hiện tại tự động dùng được theo Professional. Xem [tiếng Việt](license.md) / [English](license.en.md).

1.1.3 là lần đầu các gói registry và corresponding-source archive chứa `LICENSE-7MLABS-MIT`, `LICENSE-SCOPE.md`, `LICENSE-SCOPE.json` cùng `NOTICE` cập nhật. Các archive 1.1.2 đã phát hành không bị ghi đè hoặc đóng gói lại. Quyền MIT đã cấp cho các revision nằm trong phạm vi vẫn tồn tại; việc đưa tệp cấp quyền vào gói mới không chuyển quyền Swiss. Các mốc, source snapshot và bằng chứng cũ trong phần còn lại giữ ngữ cảnh lịch sử của chúng.

## Điều kiện trước khi public

Repo mới không import ứng dụng/history legacy. Bản phân phối engine native công khai dùng `AGPL-3.0-only`, giữ toàn văn LICENSE/NOTICE và provenance của third-party. Phần đóng góp thuộc 7mlabs được nhận diện trong revision có tệp cấp quyền còn có quyền MIT bổ sung theo phạm vi nêu trên. Renderer SVG native thuộc bản phân phối kết hợp này; font và mã UI/renderer legacy có phạm vi và license riêng. Source npm giữ `private: true`; assembler chỉ bỏ guard trong tarball public đã qua release gate. Cargo giữ `publish = false`. Workflow candidate chỉ build/kiểm thử; publication được tách khỏi bước này. Chỉ upload đúng artifact cuối đã qua gate; không suy ra PyPI/NuGet đã được publish từ một run build thành công.

Swiss Ephemeris có license kép AGPL hoặc Professional theo [tài liệu upstream](https://www.astro.com/swisseph-download/doc/swephprg.pdf); bản phân phối native công khai chọn nhánh AGPL. Quyền MIT riêng cho phần 7mlabs không cấp thay quyền Professional của Swiss. Provider được vendor theo commit đã pin, có bản sửa `sweph.c:swi_fopen` được ghi chính xác trong provenance, `NOTICE` và diff thuộc kiểm kê. Các notices của upstream được giữ đi kèm. Phần mềm tích hợp engine cần tuân thủ các nghĩa vụ AGPL tương ứng; xem [license.md](license.md).

## Tổ chức public repository

Public repo nên bao gồm core, bindings, examples, schema, docs, tests và workflows. Không commit build cache, virtualenv, node_modules, secrets, logs, dữ liệu sinh cá nhân hoặc toàn bộ publish binaries legacy. Binary release qua registry/GitHub Releases kèm checksum và platform manifest.

Checkout mới chỉ chứa `neutral-engine/`, `bindings/`, `schemas/`, `tests/`, `examples/`, `scripts/`, `docs/` và `.github/`. Không import lịch sử Git hoặc ứng dụng Python/.NET/sandbox cũ. Docs legacy chỉ lưu thông tin lịch sử. Giữ vendor licenses và third-party notices; bản scan theo pattern không thay thế việc xác nhận provenance.

## Package naming và version

Ba tên package npm/PyPI/NuGet được đồng bộ theo version **1.1.3**; trạng thái publication và bằng chứng mỗi registry phải được ghi trong hồ sơ phát hành của chính version đó. Core và các SDK native dùng cùng package version; ABI/schema giữ vòng version riêng. Rust crates giữ `publish = false`. UI renderer giữ version độc lập. Version artifact public không được tái sử dụng cho bytes khác.

## Cài và cập nhật thuận tiện

Mỗi runtime có một package; engine native được đóng gói hoặc chọn tự động theo OS/CPU. SDK và core cùng version. Bản npm đầu dùng một tarball chứa hai addon macOS ARM64 và Linux x64, loader chọn đúng binary; chưa cần optional platform packages. Có thể tách platform packages khi matrix lớn hơn. Python cần platform wheels; NuGet cần một nupkg chứa runtime assets của các nền tảng được hỗ trợ. Consumer cần runtime và package; Rust/C source build cần compiler.

```sh
npm install @7mlabs/astrology
```

Cập nhật tường minh bằng `npm install @7mlabs/astrology@latest`. Pin bằng `npm install --save-exact @7mlabs/astrology@1.1.3` và commit lockfile. Kênh ổn định dùng `latest`; nền tảng và libc requirements nằm trong [hướng dẫn Node.js](node-release.md).

Candidate build một platform mỗi lần, tạo tgz/nupkg cùng tên giữa hai job. npm assembler đã ghép hai candidate vào một tarball duy nhất trước release; không publish riêng hai candidate cùng package/version. NuGet ghép hai RID osx-arm64/linux-x64 trong một nupkg. Từ 1.1.2, Python có wheel macOS ARM64 và wheel `manylinux_2_38_x86_64` cho Linux x64; giữ yêu cầu Python 3.10+, glibc 2.38+ và `libgcc_s.so.1`. Ubuntu 24.04 là baseline kiểm thử Linux; Windows, macOS Intel, Linux ARM64 và Alpine/musl không nằm trong matrix này. 1.1.3 giữ nguyên phạm vi platform, vẫn phải kiểm tra cài sạch từ chính các archive cuối.

## CI và phát hành

CI candidate build/test macOS ARM64 và Linux x64, assemble npm tarball rồi fresh-install tarball cuối trên Node 18/24 ở mỗi target; kiểm tra wheel và NuGet trên Python 3.10/3.13 ở hai target. Workflow candidate không publish registry. Gate giấy phép kiểm tra đúng phạm vi, hash, bản vá Swiss và các tệp cấp quyền/notice trong archive; metadata của toàn gói tiếp tục là `AGPL-3.0-only`. Báo cáo consumer phải tham chiếu checksum của chính artifact sẽ được upload.

## Bằng chứng lịch sử (không thay thế hồ sơ 1.1.3)

Bằng chứng lịch sử npm alpha `0.10.0-alpha.1`: [run 37096125672](https://github.com/7mlabs/sdk-astro/actions/runs/37096125672) đã qua tại commit `a17850e01507485312e5cb584ed1eb82a786fcbc`: 108 tests Rust, 339 parity cases giữa năm ngôn ngữ và bốn final npm consumer jobs. SHA256 tarball alpha đã publish: `d8dcfdbaea4e66070a75f9e5ef91d0e6302b33995b364dce40b6a1e28eba9748`. Registry integrity/tag và fresh registry install trên macOS ARM64 đã được kiểm tra. Bằng chứng Linux giới hạn ở Ubuntu 24.04 x64 và không xác nhận tính portable của wheel trên các distro khác.

Bản stable `0.10.0`: [run 37098509933](https://github.com/7mlabs/sdk-astro/actions/runs/37098509933), source commit `4322f811a7a0956645e6936d63465ef438379899`, tarball SHA256 `23e71995eb71119acbce39219edcb252d158aac63a3ddcb9b1210a5fe6d8755a`. Cả bảy jobs build/kiểm chứng qua, gồm bốn final consumer jobs Node 18/24 trên macOS ARM64/Linux x64, và publish bằng npm OIDC thành công. Run gốc báo lỗi do kiểm tra registry sau publish hết thời gian chờ metadata lan truyền; xác minh thủ công sau đó đã kiểm tra identity/license, `latest = 0.10.0`, integrity/checksum và fresh bare install trên macOS ARM64. Bằng chứng từng bước ở [Node release](node-release.md). Workflow `npm-release.yml` tách quyền publish, chạy lại gates rồi chọn `latest` cho stable, `alpha` cho prerelease và kiểm tra registry checksum/dist-tag. Trusted publisher đã cấu hình cho `7mlabs/sdk-astro`, workflow `npm-release.yml`, environment `npm-release`; bản alpha đầu dùng tài khoản npm có 2FA. Publish OIDC đã được kiểm chứng bằng lần phát hành stable `0.10.0`; trạng thái lỗi của bước registry ban đầu được giữ rõ trong bằng chứng. Bản tiếp theo phải tăng version rồi dùng tag tương ứng. Các thông tin publisher ở đoạn lịch sử này mô tả thời điểm 0.10.0; trạng thái publication 1.1.3 theo hồ sơ riêng. Không có gói crates.io độc lập; xem [hướng dẫn setup](github-setup.md).

## Không duy trì server

Bạn duy trì source, issue/PR, build matrix và releases. Registry/GitHub chứa source và artifacts. Người dùng tính trong process của họ; không có server uptime, database hoặc account của engine. MCP cũng chạy local. Cần bảo trì dependency và dữ liệu khi upstream đổi, không phải vận hành backend.
