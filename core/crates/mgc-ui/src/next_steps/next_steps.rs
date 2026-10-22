//! Project follow-up output — output hướng dẫn sau khi tạo project.
//! The command suggestions are shared by production output and nearest tests.
//! Gợi ý lệnh được dùng chung bởi output production và test gần nhất.

use crate::quiet::is_quiet;
use crate::{hint, success};
use console::style;

pub(super) fn suggested_commands(project_name: &str) -> [String; 3] {
    [
        format!("  cd {}", project_name),
        "  mgc install    # Install dependencies".to_string(),
        "  mgc dev        # Start development".to_string(),
    ]
}

/// Print next steps after project creation — in hướng dẫn sau khi tạo project.
/// Quiet mode suppresses the full panel — quiet sẽ ẩn toàn bộ bảng.
pub fn print_next_steps(project_name: &str) {
    if is_quiet() {
        return;
    }
    println!();
    println!(
        "  {}",
        style("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓").green()
    );
    println!(
        "  {}",
        style("┃            All done!                   ┃")
            .green()
            .bold()
    );
    println!(
        "  {}",
        style("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛").green()
    );
    println!();
    success("Project created successfully!");
    println!();

    let commands = suggested_commands(project_name);
    for command in &commands {
        hint(command);
    }
}
