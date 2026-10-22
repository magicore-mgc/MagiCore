# Stress CI operation owners / Owner thao tác Stress CI

This directory owns the longer setup and lifecycle commands used by `.github/workflows/stress-matrix.yml`. The workflow remains the source of truth for the three-OS matrix, pinned GitHub actions, conditions, step-level environment, order, timeouts and artifact upload. These scripts run only as steps of that workflow and receive the runner's environment and workspace cwd.

Folder này sở hữu các lệnh setup/lifecycle dài của `.github/workflows/stress-matrix.yml`. Workflow vẫn là nguồn chuẩn cho ma trận ba OS, GitHub action ghim phiên bản, điều kiện, environment từng step, thứ tự, timeout và upload artifact. Script chỉ được gọi như step của workflow, nhận environment và cwd của runner.

## Owner map / Bản đồ owner

- `toolchains/`: OS-specific verified runtime installation and version checks; exact shared tools live under `scripts/ci/toolchains/`.
- `cli/`: release build and stable binary snapshot used by the lifecycle lanes.
- `lanes/`: substantial Go, App/Dart and Web/frontend product E2E command groups.

Each child owner documents its own script inputs and supported runner OS. Keep GitHub expressions and orchestration in workflow YAML. Add `scripts/ci/stress/**` to both `push` and `pull_request` path filters when extending this tree; shared CI operations also require their shared path in those filters.

Mỗi owner con ghi rõ input và OS runner được dùng. Giữ GitHub expression và orchestration trong YAML workflow. Khi mở rộng cây này, phải theo dõi `scripts/ci/stress/**` ở cả path filter `push` và `pull_request`; thao tác CI dùng chung cũng cần path tương ứng trong filter.

Tests and workflow execution are deferred until the repository-wide production-source pass closes; current static extraction does not verify runner behavior.
