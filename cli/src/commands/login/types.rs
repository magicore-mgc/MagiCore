//! Login command arguments.
//! Cấu hình đầu vào của lệnh đăng nhập.

use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct LoginArgs {
    #[arg(long, help = "registry URL (default: config or npmjs)")]
    pub registry: Option<String>,
    #[arg(long, help = "username (prompt if omitted)")]
    pub username: Option<String>,
    #[arg(long, help = "read password from stdin; otherwise prompt without echo")]
    pub password_stdin: bool,
    #[arg(long, help = "write token to project .npmrc instead of ~/.npmrc")]
    pub local: bool,
}
