# Package file selection

This owner selects regular package inputs from a project root while preserving npm-compatible ignore-file precedence. The stable public entry is `mgc_pack::ignore::select_files`.

The coordinator in `ignore.rs` walks the root, applies the policy from `include_rules/`, and uses the builder configuration in `walk_config/`. It returns relative `PathBuf` values in sorted order and keeps the existing `anyhow::Result<Vec<PathBuf>>` boundary. It does not create archives, rewrite manifests, or access the registry.

Behavioral coverage is one Cargo integration target, registered from `test/ignore.rs`. Its assertions are grouped under the nearest owners: `include_rules/test/` checks every forced include/exclude entry, `walk_config/test/` checks npm precedence and git fallback, and `test/ignore.rs` checks sorted selection output. The target name is `test_ignore`.

READMEVN: Owner này chọn file package theo quy tắc ignore tương thích npm. API công khai vẫn là `mgc_pack::ignore::select_files`; coordinator, include/exclude policy và cấu hình walker được tách theo trách nhiệm. Test tích hợp gần owner bao phủ precedence, file bắt buộc, đường dẫn loại trừ và thứ tự kết quả.
