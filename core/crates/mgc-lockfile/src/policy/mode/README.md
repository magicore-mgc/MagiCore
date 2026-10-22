# Lock policy mode / Chế độ lock policy

Defines `LockPolicyMode::{Off, Warn, Require}`, case-insensitive parsing, and the CI-aware environment default. It owns no resolution precedence; `policy/policy.rs` handles flag/env/project selection. No local error type is needed because unknown text is represented by `Option`.

Khai báo mode, parser không phân biệt hoa thường và mặc định CI. Không sở hữu precedence; phần đó thuộc `policy.rs`. Giá trị không hợp lệ trả `Option`, không cần error riêng.
