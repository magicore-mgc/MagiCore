# Lifecycle matrix setup owners / Owner setup lifecycle matrix

This tree owns workflow-specific preparation for `.github/workflows/lifecycle-matrix.yml`. Workflow YAML retains the three-OS matrix, pinned setup actions, conditions, step order, environment and evidence gate. Shared toolchain operations live under `scripts/ci/toolchains/` only when their verification contract is identical.

Cây này sở hữu phần chuẩn bị riêng cho `.github/workflows/lifecycle-matrix.yml`. YAML workflow giữ ma trận ba OS, setup action ghim phiên bản, điều kiện, thứ tự step, environment và evidence gate. Thao tác toolchain dùng chung chỉ nằm ở `scripts/ci/toolchains/` khi contract xác minh giống hệt.
