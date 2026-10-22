# Import transaction coordinator / Coordinator giao dịch import

Owns `run` and the ordered lock → journal → parse → snapshot → merge → signing → verify/rollback → report flow. The binary-level Bun/Deno import scenarios remain in `cli/tests/audit_cli_e2e.rs`; this source owner does not access or mutate the user's keyring from a unit test.

Sở hữu `run` và thứ tự lock → journal → parse → snapshot → merge → signing → verify/rollback → report. E2E import Bun/Deno vẫn ở `cli/tests/audit_cli_e2e.rs`; unit test owner không truy cập/sửa keyring người dùng.
