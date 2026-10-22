# Lockfile policy owners / Chủ sở hữu policy lockfile

This folder keeps the stable `mgc_lockfile::policy::*` API and splits trust-policy responsibilities by invariant. `mod.rs` only declares and re-exports owners; each behavior lives in its same-name implementation file.

- `policy.rs`: CLI/environment/project precedence and a bounded no-follow `mgc.toml` read.
- `mode/`: policy values and environment default.
- `report/`: public structural verification report.
- `digest/`: strict BLAKE3 digest decoding shared by verification and signing.
- `verification/`: v4 root pins, digest and signature math.
- `enforcement/`: trust decisions applied after structural verification.
- `install_integrity/`: SRI shape gate and native SHA-256 representation.
- `file_verification/`: bounded on-disk parse → math → policy flow.
- `signing/`: canonical payload digest signing.

No local error type is needed: operations retain the crate-level `LockfileError` / `LockfileResult` contract. Preserve CI's non-downgradable `require`, symlink/reparse rejection, input bounds, failure order, public signatures and existing diagnostics. Direct integration coverage remains under `tests/policy/` with unchanged root Cargo target files; tests have not run.

Thư mục này giữ API ổn định `mgc_lockfile::policy::*` và chia trách nhiệm theo invariant. `mod.rs` chỉ khai báo/export; implementation ở file cùng tên. Không tạo error giả: dùng contract lỗi chung của crate. Mọi quyết định CI, giới hạn/no-follow, thứ tự lỗi và diagnostic hiện tại phải giữ nguyên. Test integration ở `tests/policy/`; chưa chạy.
