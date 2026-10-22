//! Render shared help options — dựng các tùy chọn help dùng chung.
//! Keep the core list and global help/version flags in their existing section.
//! Giữ danh sách core cùng cờ help/version ở đúng section hiện tại.

use colored::Colorize;

pub(in crate::help) fn append_options_section(out: &mut String, available_cores: &[(&str, &str)]) {
    out.push_str(&format!("  {}\n", "── Options ──".white().bold()));
    let cores_list = available_cores
        .iter()
        .map(|(short_name, _)| *short_name)
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&format!(
        "    {}   {}\n",
        "--core <CORE>".cyan().bold(),
        format!("Target core ({})", cores_list).white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "-h, --help".cyan().bold(),
        "Print help (per-command)".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "-V, --version".cyan().bold(),
        "Print version".white()
    ));
    out.push('\n');
}
