# Lockfile parser API / API parser lockfile

This folder is the stable `mgc_lockfile::parser` module. `mod.rs` only wires/re-exports; parsing, bounded reads, loading, signature presence, and verification live with their behavior owners. Root crate re-exports remain in `src/lib.rs`.

Thư mục này là module ổn định `mgc_lockfile::parser`. `mod.rs` chỉ nối/export; parse, bounded read, load, kiểm tra chữ ký và verify nằm cạnh owner chức năng. Các export ở crate root vẫn ở `src/lib.rs`.

Read [ProgressReport.md](docs/ProgressReport.md) for source-only status. Existing crate integration tests remain public-consumer contracts; nearest-owner tests live under each owner `test/` and are NOT_RUN until the repository-wide source pass closes.
