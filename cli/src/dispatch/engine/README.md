# Dispatch engine

Owns top-level CLI orchestration, workspace execution, selector validation and command policies.

Audit-strict command classification is isolated in [`audit_policy/`](audit_policy/README.md); recursive/filter command policy is isolated in [`recursive_policy/`](recursive_policy/README.md), while recursive workspace discovery, selection, ordering, identity planning and dispatch live in [`recursive/`](recursive/README.md). Shared policy diagnostics live in [`error/`](error/README.md). The engine runner owns top-level ordering and environment mutation.

Workspace/core identity reads and explicit dependency-route validation live in [`identity/`](identity/README.md). The `engine` and `recursive` owners retain their existing call sites and delegate identity checks to that owner.

## Vietnamese

Folder này điều phối CLI, chạy workspace, xác thực core đã chọn và áp dụng policy cho lệnh. Phân loại command audit-strict nằm tại [`audit_policy/`](audit_policy/README.md); policy cho lệnh recursive/filter nằm tại [`recursive_policy/`](recursive_policy/README.md); luồng workspace đệ quy gồm khám phá, lọc, sắp thứ tự, lập identity plan và dispatch nằm tại [`recursive/`](recursive/README.md). Chẩn đoán policy dùng chung ở [`error/`](error/README.md), còn identity workspace và route dependency ở [`identity/`](identity/README.md). Engine runner chịu trách nhiệm thứ tự cấp cao và thay đổi environment.
