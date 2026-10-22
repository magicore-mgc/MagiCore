# CI operation scripts

This folder owns focused shell/PowerShell operations invoked by GitHub Actions. Workflow YAML retains triggers, jobs, setup actions, OS matrices, conditions and step order; scripts own long commands and their local failure handling.

Thư mục này sở hữu các thao tác shell/PowerShell nhỏ được GitHub Actions gọi. Workflow YAML giữ trigger, job, action setup, ma trận OS, điều kiện và thứ tự step; script giữ các lệnh dài cùng xử lý lỗi cục bộ.

The `stress/` subtree owns custom runtime/toolchain setup, complete runtime version gates, release CLI preparation and substantial Go/App/Web stress lifecycle commands. The identical Linux Swift signing-key import shared by stress and lifecycle lives in `toolchains/swift/`. `.github/workflows/stress-matrix.yml` retains its OS matrix, pinned setup actions, step environment, timeouts, product step ordering and evidence artifact contract.

`stress/` sở hữu setup runtime/toolchain tự quản lý, gate version đầy đủ, chuẩn bị CLI release và các lệnh stress lifecycle dài cho Go/App/Web. Import khóa Swift Linux giống hệt giữa stress/lifecycle được dùng chung trong `toolchains/swift/`. `.github/workflows/stress-matrix.yml` vẫn giữ ma trận OS, setup action ghim, environment từng step, timeout, thứ tự product step và contract artifact evidence.

The `lifecycle/` subtree owns workflow-specific Swift preservation/installation and App Linux prerequisites. Keep its release-binary-distinct Swift installer separate; shared signing-key setup has its own toolchain README.

`lifecycle/` sở hữu thao tác Swift preservation/cài đặt và prerequisite App Linux riêng workflow. Giữ installer Swift riêng vì contract khác release-binary; setup nhập signing key dùng chung có README toolchain riêng.

Script contract tests are added during the later test phase. Never treat a static source review or a green unrelated job as evidence that a script ran on its target OS.
