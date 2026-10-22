# Release binary E2E script owners

This folder owns the operation scripts called by `.github/workflows/release-binary-e2e.yml`. The workflow remains the GitHub entrypoint and owns triggers, OS matrix, pinned setup actions/toolchains, step order and conditional execution. Scripts hold one archive, toolchain, or product-lifecycle operation each.

Thư mục này sở hữu các script thao tác được gọi bởi `.github/workflows/release-binary-e2e.yml`. Workflow vẫn là entrypoint GitHub, quản lý trigger, ma trận OS, action/toolchain ghim phiên bản, thứ tự và điều kiện step. Script chỉ giữ từng thao tác archive, toolchain hoặc lifecycle sản phẩm.

## Owners / Chủ sở hữu

- `archive/`: packages the release-shaped binary archive and installs it into the isolated prefix.
- `archive/`: đóng gói archive theo định dạng release và cài vào prefix cô lập.
- `lifecycle/`: native MGC create/add/install/test/build/audit, lock migration, cache reuse and output proof lanes.
- `lifecycle/`: các lane create/add/install/test/build/audit bằng MGC, chuyển lockfile, tái sử dụng cache và xác minh output.
- `toolchain/`: verifies Swift signing keys and installs the pinned signed Windows toolchain.
- `toolchain/`: xác minh khóa ký Swift và cài toolchain Windows có chữ ký đã ghim.
- `web/`: coordinates the official initializer, provenance, Web lifecycle, process probes and warm/offline reinstall using one temporary project.
- `web/`: điều phối initializer chính thức, provenance, lifecycle Web, probe process và cài lại cache/offline trên cùng một dự án tạm.

## Contracts / Hợp đồng

- Workflow Bash steps invoke files with `bash`; file executable bits are not required. PowerShell steps run under `pwsh`.
- Step Bash của workflow gọi file bằng `bash`, không phụ thuộc executable bit. Step PowerShell chạy dưới `pwsh`.
- `RUNNER_OS`, `HOME`, `GITHUB_ENV` and `GITHUB_PATH` come from GitHub Actions. Archive scripts additionally consume `HOST_TRIPLE`/`STAGE_DIR` through the existing GitHub environment hand-off.
- `RUNNER_OS`, `HOME`, `GITHUB_ENV` và `GITHUB_PATH` do GitHub Actions cấp. Script archive nhận thêm `HOST_TRIPLE`/`STAGE_DIR` qua cơ chế truyền môi trường hiện có của GitHub.
- The Web coordinator exports `MGCBIN` and one `WORK` directory to all ordered child phases. Every phase re-enters the same `$WORK/test-web` folder where needed.
- Coordinator Web export `MGCBIN` và cùng một thư mục `WORK` cho các pha con theo thứ tự. Mỗi pha chuyển lại vào `$WORK/test-web` khi cần.
- Commands must propagate failure. Scripts retain strict audit exit handling, online creator/provenance assertions, HTTP probes, process cleanup, cache proof and offline install behavior.
- Lệnh phải truyền lỗi ra ngoài. Script giữ nguyên xử lý exit audit strict, assertion creator/provenance online, HTTP probe, dọn process, chứng minh cache và hành vi cài offline.
- Test owners belong next to these scripts in a later verification phase. No empty test folder is created during source decomposition.
- Test owner sẽ được đặt gần script ở pha xác minh sau. Không tạo folder test rỗng trong pha tách source.

## Verification status / Trạng thái xác minh

S156 đang rà tĩnh sơ đồ owner và wiring lệnh/môi trường. Chưa chạy script, shell parser, YAML parser, actionlint, GitHub Actions hay product E2E; README này không xác nhận runtime hoạt động.
