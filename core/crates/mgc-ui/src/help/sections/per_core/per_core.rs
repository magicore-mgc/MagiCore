//! Render commands scoped to each core — dựng lệnh có phạm vi từng core.
//! Preserve the established command sequence and labels.
//! Giữ nguyên thứ tự lệnh và nhãn hiện hành.

use colored::Colorize;

pub(in crate::help) fn append_per_core_section(out: &mut String, available_cores: &[(&str, &str)]) {
    out.push_str(&format!("  {}\n", "── Per-core Commands ──".white().bold()));
    for (core, label) in available_cores {
        out.push_str(&format!(
            "    {}  {}\n",
            super::super::command_name::per_core_name("install", core)
                .cyan()
                .bold(),
            format!("Install dependencies for {}", label).white()
        ));
        out.push_str(&format!(
            "    {}  {}\n",
            super::super::command_name::per_core_name("add", core)
                .green()
                .bold(),
            format!("Add dependency for {}", label).white()
        ));
        out.push_str(&format!(
            "    {}  {}\n",
            super::super::command_name::per_core_name("remove", core)
                .red()
                .bold(),
            format!("Remove dependency for {}", label).white()
        ));
        out.push_str(&format!(
            "    {}  {}\n",
            super::super::command_name::per_core_name("list", core)
                .blue()
                .bold(),
            format!("List packages for {}", label).white()
        ));
        out.push_str(&format!(
            "    {}  {}\n",
            super::super::command_name::per_core_name("update", core)
                .magenta()
                .bold(),
            format!("Update packages for {}", label).white()
        ));
    }
    out.push('\n');
}
