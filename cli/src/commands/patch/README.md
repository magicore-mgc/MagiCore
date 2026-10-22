# Patch command owners / Owner lệnh patch

This facade preserves `crate::commands::patch::{run, PatchArgs, PatchCmd}`. `patch.rs` resolves the project and serializes mutations; add/remove/list/verify each live in a focused owner. The existing project-write lock, `.magicore/patches` store, integrity policy, and not-yet-supported patch application behavior are preserved.

Facade này giữ nguyên `crate::commands::patch::{run, PatchArgs, PatchCmd}`. `patch.rs` tìm project và tuần tự hóa mutation; add/remove/list/verify nằm ở owner riêng. Giữ project-write lock, store `.magicore/patches`, policy integrity và hành vi patch application chưa được hỗ trợ.

`types.rs` owns Clap command types; `config.rs` owns the existing size limit; `test/` contains the patch-specific regression suite formerly linked from the shared commands test folder. Errors continue through `anyhow` and the existing `crate::error` helpers; no empty error module was added.

`types.rs` sở hữu type Clap; `config.rs` sở hữu giới hạn size hiện có; `test/` chứa regression test riêng của patch, trước đây được nối từ thư mục test commands dùng chung. Error tiếp tục đi qua `anyhow` và helper `crate::error`; không tạo error module rỗng.
