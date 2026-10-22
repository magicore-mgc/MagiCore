# Resolved registry entries

Builds Dart package-config rows for materialized pub.dev packages. It verifies the archive manifest name against the resolved identity and requires a canonical package root before creating its URI.

`error.rs` distinguishes duplicate identities, malformed/mismatched manifests, I/O failures, and URI failures. Existing cases are under `test/registry_entries.rs`.

## Chủ sở hữu

Dựng row package-config Dart cho package pub.dev đã materialize. Owner so sánh tên manifest archive với danh tính đã resolve và yêu cầu package root canonical trước khi tạo URI.

`error.rs` tách lỗi danh tính trùng, manifest sai/lệch danh tính, I/O và URI. Các case hiện có nằm tại `test/registry_entries.rs`.
