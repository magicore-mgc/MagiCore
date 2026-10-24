//! JavaScript package-manager names that policy inspects instead of rejecting outright.
//! Tên package manager JavaScript mà policy xét thay vì từ chối thẳng.
//!
//! Every name here is also in `FORBIDDEN_TOOLS`. The difference is the verdict
//! path: these are routed to the per-ecosystem dependency policy so a read-only
//! probe (`--version`, `config get`) is allowed while a graph-changing command
//! (`install`, `add`) is denied with a reason.
//! Mọi tên ở đây cũng nằm trong `FORBIDDEN_TOOLS`. Khác biệt là đường quyết
//! định: chúng đi qua policy dependency theo ecosystem để probe chỉ đọc
//! (`--version`, `config get`) được phép còn lệnh đổi graph (`install`, `add`)
//! bị từ chối kèm lý do.
pub const JS_PACKAGE_MANAGERS: &[&str] = &["npm", "npx", "pnpm", "yarn", "bun", "bunx"];
