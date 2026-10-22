# Work Root

Build separate dlx and initializer roots under `MGC_DATA_DIR/cache/web` and reject symlinks, special files, and non-canonical directory components.
Tạo root riêng cho dlx và initializer dưới `MGC_DATA_DIR/cache/web`; từ chối symlink, special file và component không canonical.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `work_root.rs`: `work_root.rs` owns `PackageRunPurpose`, path components, prefixes, and path validation; `error.rs` owns typed unsafe-cache-path failure reused by runner workspace.
- `test/work_root.rs`: `test/work_root.rs` covers configured root, purpose separation, files and symlink chains; `test/error.rs` checks typed path context.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`work_root.rs giữ PackageRunPurpose, component/prefix và kiểm path; error.rs giữ lỗi cache unsafe typed dùng lại bởi runner workspace.`

## Test and limits / Test và giới hạn

`test/work_root.rs` covers configured root, purpose separation, files and symlink chains; `test/error.rs` checks typed path context. The code does not prune old roots; cache retention and garbage collection are outside this owner.

test/work_root.rs kiểm root, tách purpose, file và symlink; test/error.rs kiểm path context typed. Code không prune root cũ; retention/GC nằm ngoài owner này.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
