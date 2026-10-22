//! Defines the stable doctor CLI options.
//! Khai báo các tùy chọn CLI doctor hiện hành.

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum DoctorCmd {
    /// Run diagnostic check with AI-remediation hints
    // Chạy kiểm tra chẩn đoán kèm gợi ý xử lý cho AI.
    Check {
        #[arg(short, long, help = "output JSON")]
        json: bool,
        #[arg(
            long,
            help = "create a missing store directory; report the repair result"
        )]
        fix: bool,
    },
}
