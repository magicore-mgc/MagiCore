# Installed Metadata

Read only the requested package manifest, check its package name, parse declared executable metadata, and validate the installed semantic version.
Chỉ đọc manifest package được yêu cầu, kiểm tên package, parse khai báo executable và xác minh semantic version.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `installed_metadata.rs`: `installed_metadata.rs` owns `requested_package_bins` and `requested_package_version`; `error.rs` owns typed invalid-package-metadata failure shared with managed-tool record validation.
- `test/installed_metadata.rs`: `test/installed_metadata.rs` covers package ownership, semver and symlink refusal; `test/error.rs` checks the local typed variant.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`installed_metadata.rs sở hữu requested_package_bins và requested_package_version; error.rs giữ lỗi metadata package typed dùng chung với record validation.`

## Test and limits / Test và giới hạn

`test/installed_metadata.rs` covers package ownership, semver and symlink refusal; `test/error.rs` checks the local typed variant. Uses regular-file-safe config reading; it does not scan dependency binaries or rewrite package manifests.

test/installed_metadata.rs kiểm ownership, semver, symlink; test/error.rs kiểm variant. Đọc regular file an toàn; không quét bin dependency và không sửa manifest.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
