# GitHub công khai và phân phối package

Repo công khai là [7mLabs/sdk-astro](https://github.com/7mLabs/sdk-astro). npm `@7mlabs/astrology`, PyPI `sevenmlabs-astrology` và NuGet `SevenMLabs.Astrology` đều đã phát hành **1.0.2**, có checksum và kiểm tra cài mới từ registry. Rust/C tích hợp từ source; UI renderer có vòng version riêng. Xem [bằng chứng 1.0.2](release-1.0.2.md) và [thiết lập GitHub](github-setup.md).

## Điều kiện trước khi public

Repo mới không import ứng dụng/history legacy. Source mới và engine packages dùng `AGPL-3.0-only`, có toàn văn LICENSE/NOTICE và provenance của third-party. Renderer độc lập giữ MIT. Source npm giữ `private: true`; assembler chỉ bỏ guard trong tarball public đã qua release gate. Cargo giữ `publish = false`. Python/NuGet vẫn có thể upload về mặt công cụ, nhưng workflow hiện không có bước publish chúng.

Swiss Ephemeris có license kép AGPL hoặc Professional theo [tài liệu upstream](https://www.astro.com/swisseph-download/doc/swephprg.pdf); dự án chọn AGPL miễn phí. Provider được vendor theo commit đã pin, có bản vá chặn file ngoài ngày 2026-10-02 và notices đi kèm. Phần mềm tích hợp engine cần tuân thủ các nghĩa vụ AGPL tương ứng; xem [license.md](license.md).

## Tổ chức public repository

Public repo nên bao gồm core, bindings, examples, schema, docs, tests và workflows. Không commit build cache, virtualenv, node_modules, secrets, logs, dữ liệu sinh cá nhân hoặc toàn bộ publish binaries legacy. Binary release qua registry/GitHub Releases kèm checksum và platform manifest.

Checkout mới chỉ chứa `neutral-engine/`, `bindings/`, `schemas/`, `tests/`, `examples/`, `scripts/`, `docs/` và `.github/`. Không import lịch sử Git hoặc ứng dụng Python/.NET/sandbox cũ. Docs legacy chỉ lưu thông tin lịch sử. Giữ vendor licenses và third-party notices; bản scan theo pattern không thay thế việc xác nhận provenance.

## Package naming và version

Ba tên package npm/PyPI/NuGet đã xác minh và phát hành cùng version **1.0.2**. Core và các SDK native dùng cùng package version; ABI/schema giữ vòng version riêng. Rust crates giữ `publish = false`. UI renderer giữ version độc lập. Version artifact public không được tái sử dụng cho bytes khác.

## Cài và cập nhật thuận tiện

Mỗi runtime có một package; engine native được đóng gói hoặc chọn tự động theo OS/CPU. SDK và core cùng version. Bản npm đầu dùng một tarball chứa hai addon macOS ARM64 và Linux x64, loader chọn đúng binary; chưa cần optional platform packages. Có thể tách platform packages khi matrix lớn hơn. Python cần platform wheels; NuGet cần một nupkg chứa runtime assets của các nền tảng được hỗ trợ. Consumer cần runtime và package; Rust/C source build cần compiler.

```sh
npm install @7mlabs/astrology
```

Cập nhật tường minh bằng `npm install @7mlabs/astrology@latest`. Pin bằng `npm install --save-exact @7mlabs/astrology@1.0.2` và commit lockfile. Kênh ổn định dùng `latest`; nền tảng và libc requirements nằm trong [hướng dẫn Node.js](node-release.md).

Candidate build một platform mỗi lần, tạo tgz/nupkg cùng tên giữa hai job. npm assembler đã ghép hai candidate vào một tarball duy nhất trước release; không publish riêng hai candidate cùng package/version. NuGet vẫn cần assembly nhiều runtime trước public. Linux wheel hiện có tag Linux thường, cần manylinux build/repair và kiểm thử môi trường sạch trước public PyPI. Windows chưa được builder hỗ trợ.

## CI và phát hành

CI candidate build/test macOS ARM64 và Linux x64, assemble npm tarball rồi fresh-install tarball cuối trên Node 18/24 ở mỗi target; workflow này không publish registry. Bằng chứng lịch sử npm alpha `0.10.0-alpha.1`: [run 37096125672](https://github.com/7mlabs/sdk-astro/actions/runs/37096125672) đã qua tại commit `a17850e01507485312e5cb584ed1eb82a786fcbc`: 108 tests Rust, 339 parity cases giữa năm ngôn ngữ và bốn final npm consumer jobs. SHA256 tarball alpha đã publish: `d8dcfdbaea4e66070a75f9e5ef91d0e6302b33995b364dce40b6a1e28eba9748`. Registry integrity/tag và fresh registry install trên macOS ARM64 đã được kiểm tra. Bằng chứng Linux giới hạn ở Ubuntu 24.04 x64 và không xác nhận tính portable của wheel trên các distro khác.

Bản stable `0.10.0`: [run 37098509933](https://github.com/7mlabs/sdk-astro/actions/runs/37098509933), source commit `4322f811a7a0956645e6936d63465ef438379899`, tarball SHA256 `23e71995eb71119acbce39219edcb252d158aac63a3ddcb9b1210a5fe6d8755a`. Cả bảy jobs build/kiểm chứng qua, gồm bốn final consumer jobs Node 18/24 trên macOS ARM64/Linux x64, và publish bằng npm OIDC thành công. Run gốc báo lỗi do kiểm tra registry sau publish hết thời gian chờ metadata lan truyền; xác minh thủ công sau đó đã kiểm tra identity/license, `latest = 0.10.0`, integrity/checksum và fresh bare install trên macOS ARM64. Bằng chứng từng bước ở [Node release](node-release.md). Workflow `npm-release.yml` tách quyền publish, chạy lại gates rồi chọn `latest` cho stable, `alpha` cho prerelease và kiểm tra registry checksum/dist-tag. Trusted publisher đã cấu hình cho `7mlabs/sdk-astro`, workflow `npm-release.yml`, environment `npm-release`; bản alpha đầu dùng tài khoản npm có 2FA. Publish OIDC đã được kiểm chứng bằng lần phát hành stable `0.10.0`; trạng thái lỗi của bước registry ban đầu được giữ rõ trong bằng chứng. Bản tiếp theo phải tăng version rồi dùng tag tương ứng. Các publisher PyPI/NuGet/crates.io được xử lý sau; xem [hướng dẫn setup](github-setup.md).

## Không duy trì server

Bạn duy trì source, issue/PR, build matrix và releases. Registry/GitHub chứa source và artifacts. Người dùng tính trong process của họ; không có server uptime, database hoặc account của engine. MCP cũng chạy local. Cần bảo trì dependency và dữ liệu khi upstream đổi, không phải vận hành backend.
