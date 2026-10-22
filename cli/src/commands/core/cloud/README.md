# Cloud core owner

This folder owns Cloud-specific project-to-adapter binding. It validates the persisted core identity using the existing `clo` identifier, loads `ProjectContext` at the detected root, and returns the selected adapter. The old `core::shared::cloud_adapter` path is kept as a compatibility re-export while package-operation handlers use this owner.

`error.rs` provides a typed boundary for identity/context failures while preserving their original diagnostics and causes. Unit-test sources are under `test/`; they remain uncompiled and unexecuted until the structural pass is complete.

# Owner của Cloud core

Folder này sở hữu việc gắn project Cloud với adapter. Nó xác thực core identity đã lưu bằng mã `clo` hiện có, tải `ProjectContext` tại root đã nhận diện và trả adapter được chọn. Đường cũ `core::shared::cloud_adapter` được giữ làm facade tương thích trong khi handler package-operation chuyển sang owner này.

`error.rs` tạo boundary typed cho lỗi identity/context và giữ nguyên diagnostic cùng nguyên nhân gốc. Source test nằm trong `test/`; chưa compile/chạy cho tới khi hoàn tất lượt cấu trúc.
