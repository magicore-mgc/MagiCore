//! Render project-creation help — dựng section tạo dự án.
//! Keep the special web command and named per-core alternatives intact.
//! Giữ lệnh create riêng cho web và biến thể có tên từng core.

use colored::Colorize;

use super::super::command_name::per_core_name;

pub(in crate::help) fn append_create_section(
    out: &mut String,
    available_cores: &[(&str, &str)],
    single_core_web: bool,
) {
    out.push_str(&format!("  {}\n", "── Create ──".white().bold()));
    if single_core_web {
        out.push_str(&format!(
            "    {}   {}\n",
            "create".yellow().bold(),
            "Scaffold a new web project".white()
        ));
    } else {
        for (core, label) in available_cores {
            out.push_str(&format!(
                "    {}   {}\n",
                per_core_name("create", core).yellow().bold(),
                format!("Scaffold {}", label).white()
            ));
        }
    }
    out.push('\n');
}
