# Generic install command / Lệnh install tổng quát

The facade preserves `commands::install::{run, install_into_root_ws, discover_workspace_projects}`. It wires command orchestration, transaction, ownership validation, materialization, workspace discovery and lock projection. Errors continue through the existing `anyhow` and `crate::error` routes; no duplicate error type is introduced.

Facade này giữ nguyên ba đường gọi CLI, chỉ nối các owner command, transaction, ownership, materialize, workspace và lock. Lỗi tiếp tục dùng `anyhow`/`crate::error` dùng chung; không tạo error type trùng.
