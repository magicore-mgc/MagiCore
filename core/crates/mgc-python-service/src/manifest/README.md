# Python manifest service

This module owns the shared MGC-native `pyproject.toml` contract. `mod.rs` keeps the existing six `mgc_python_service::manifest::*` functions available while the implementation lives with its responsibility: lane ownership, dependency conversion, parsing, and safe writing.

The native lane deliberately accepts only the subset it can preserve: PEP 621 base dependencies plus simple strings in PEP 735's `dev` group. Optional extras remain inactive metadata; foreign lockfiles, tool-owned dependency sources, and unmodeled requirement semantics are not claimed as native support. The writer changes only MGC-owned fields and publishes through `mgc_adapter_base::project_file::atomic_write_regular`.

The fallible owners keep local typed errors in `dependency/error.rs`, `parse/error.rs`, and `write/error.rs`; `MgError::Anyhow` preserves their local type and TOML source chain while keeping the existing `MgResult`/`MgError` boundary and displayed diagnostic. The boolean ownership predicate remains fail-closed by contract. This refactor adds no new recovery or retry policy.

This service does not own lock generation, resolution orchestration, installation, or runtime environment creation. Those stay with the lib/AI adapters and shared resolver/install owners.

## Phạm vi

Module này sở hữu hợp đồng `pyproject.toml` native của MGC. `mod.rs` giữ nguyên sáu hàm tại `mgc_python_service::manifest::*`, còn phần triển khai nằm trong các owner tương ứng: nhận quyền quản lý, chuyển đổi dependency, đọc và ghi manifest.

Lane native chỉ nhận phần semantics mà MGC có thể bảo toàn: dependency nền PEP 621 và chuỗi đơn giản trong nhóm `dev` của PEP 735. Optional extras vẫn được giữ như metadata nhưng không tự động cài; lockfile/nguồn dependency của tool khác và cú pháp chưa mô hình hóa không được coi là hỗ trợ native. Writer chỉ sửa trường thuộc MGC và dùng `atomic_write_regular`.

Các owner có thể lỗi giữ error type cục bộ tại `dependency/error.rs`, `parse/error.rs`, `write/error.rs`; `MgError::Anyhow` giữ local type và source chain TOML, đồng thời giữ ranh giới `MgResult`/`MgError` cùng diagnostic hiển thị. Predicate ownership giữ hợp đồng boolean fail-closed. Refactor này không thêm policy retry/recovery.

Service này không tạo lockfile, không điều phối resolver, không cài package, và không tạo môi trường runtime. Những việc đó thuộc adapter Lib/AI cùng các owner resolver/install dùng chung.
