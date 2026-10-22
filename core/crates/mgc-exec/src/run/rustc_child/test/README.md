# Rust toolchain child tests

Tests the trust boundary between normal Cargo/rustdoc compiler children and package-mutating Cargo commands. The moved contract test intentionally exercises the parent process matcher as well as this module's classifiers.

Kiểm tra ranh giới tin cậy giữa compiler child bình thường và lệnh Cargo sửa package; test contract gọi cả matcher của module cha và classifier tại owner này.

`mod.rs` only includes `rustc_child.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `rustc_child.rs` để giữ nguyên scope cha của test module.
