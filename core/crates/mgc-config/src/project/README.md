# Project configuration owners

EN: `mod.rs` preserves the public `mgc_config::project::*` facade. `project.rs` wires the established identity, detection, filesystem, security, policy and core-setting owners. `project_config/` owns the serialized `ProjectConfig` schema, constructor and script-grammar accessor; `project_config_io/` owns the existing load/save methods; `lock_schema/` owns the shared lock write-schema selection. Existing child-owner folders stay at their current paths and retain their parent-level access through the implementation module.

VI: `mod.rs` giữ facade công khai `mgc_config::project::*`. `project.rs` nối các owner identity, detection, filesystem, security, policy và core settings hiện có. `project_config/` sở hữu schema `ProjectConfig`, constructor và truy cập script grammar; `project_config_io/` sở hữu method load/save hiện tại; `lock_schema/` sở hữu lựa chọn schema lock ghi dùng chung. Các folder owner cũ giữ nguyên đường dẫn và vẫn truy cập parent qua implementation module.

## Contract and limits

The split preserves serialized field names/defaults, public methods and reexports, lock-schema defaults and diagnostics, and existing error behavior. It does not add validation, migration, framework support or a new error taxonomy. Existing integration tests and all 166 source-mapped caller files stay at their current paths; runtime checks remain pending the whole-source phase.

Việc tách giữ nguyên tên/default field tuần tự hóa, method và reexport công khai, default/diagnostic schema lock, cùng hành vi lỗi hiện có. Không bổ sung validation, migration, hỗ trợ framework hay error taxonomy mới. Integration test hiện tại và toàn bộ 166 file caller đã map được giữ nguyên path; kiểm tra runtime chờ giai đoạn source toàn repo.
