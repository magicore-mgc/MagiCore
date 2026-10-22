# Docker lifecycle CI owners

This folder decomposes the `docker-cli-smoke` job's long product-container operations by responsibility and core. The workflow keeps job identity, Ubuntu runner, timeout, pinned Flutter action, all step names/order, and the kind action. Every extracted Bash body is called with `bash` so execution does not depend on executable mode.

Thư mục này tách các thao tác container sản phẩm dài của job `docker-cli-smoke` theo trách nhiệm và core. Workflow giữ job identity, runner Ubuntu, timeout, Flutter action đã ghim, tên/thứ tự step và kind action. Mọi Bash body được gọi bằng `bash`, không phụ thuộc executable mode.

GitHub supplies `RUNNER_TEMP`, workspace `PWD`, Docker and the configured toolchain. Project/image/container/volume names and cleanup traps remain scoped to their original step. No script here has been executed in S157.
