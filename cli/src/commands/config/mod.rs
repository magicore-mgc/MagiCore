//! `mgc config` — read/write configuration (pnpm config parity + mgc.toml native).
//!
//! ## Nguồn cấu hình (theo thứ tự ưu tiên — cao → thấp)
//!
//! 1. `MGC_<KEY>` / `npm_config_<KEY>` — environment variables
//! 2. `mgc.toml` project (CWD hoặc parent root) — MagiCore-native
//! 3. `.npmrc` project (CWD) — npm-compat, --local
//! 4. `.npmrc` user (~/.npmrc) — npm-compat, global default
//!
//! ## Commands
//!
//! - `mgc config get <key>`           — đọc theo thứ tự ưu tiên trên
//! - `mgc config set <key> <value>`   — ghi vào .npmrc (mặc định) hoặc mgc.toml (--toml)
//! - `mgc config delete <key>`        — xóa khỏi .npmrc hoặc mgc.toml (--toml)
//! - `mgc config unset <key>`         — alias cho delete
//! - `mgc config list`                — liệt kê tất cả key từ mọi nguồn (phân biệt nguồn)

mod config;
mod implementation;

pub use config::{ConfigCmd, run};
pub use implementation::list_local_redacted;
