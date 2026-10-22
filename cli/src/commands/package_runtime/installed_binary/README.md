# Installed Binary

Verify installed package identity, package-owned manifest metadata, declared target containment, platform shim existence, and project containment before returning a runnable record.
Xác minh identity package, metadata manifest, containment target, shim theo OS và project containment trước khi trả record được chạy.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `installed_binary.rs`: `installed_binary.rs` owns `VerifiedBinary` and `verify_installed_binary`; `error.rs` owns typed missing-bin, outside-project, and target-mismatch failures. `mod.rs` exports only the verifier and error factories.
- `test/installed_binary.rs`: `test/installed_binary.rs` covers owned targets and symlink escapes; `test/error.rs` checks diagnostic type/message contracts.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`installed_binary.rs giữ VerifiedBinary và verify_installed_binary; error.rs giữ lỗi bin thiếu/outside project/target mismatch. mod.rs export verifier và factory.`

## Test and limits / Test và giới hạn

`test/installed_binary.rs` covers owned targets and symlink escapes; `test/error.rs` checks diagnostic type/message contracts. A successful verifier does not prove real process execution; the dlx production E2E remains separate.

test/installed_binary.rs kiểm target thuộc package và symlink escape; test/error.rs kiểm type/message. Verifier thành công chưa chứng minh chạy process thật; dlx E2E là lớp riêng.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
