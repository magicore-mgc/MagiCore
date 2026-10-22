//! Render web single-core shortcuts — dựng shortcut khi chỉ build web.
//! Keep generic add/remove/update/list commands exclusive to this profile.
//! Chỉ profile web đơn core mới hiện các lệnh add/remove/update/list chung.

use colored::Colorize;

pub(in crate::help) fn append_single_core_section(out: &mut String) {
    out.push_str(&format!("  {}\n", "── Single-core ──".white().bold()));
    out.push_str(&format!(
        "    {}   {}\n",
        "add".green().bold(),
        "Add a dependency".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "remove".green().bold(),
        "Remove a dependency".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "update".green().bold(),
        "Update packages".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "list".green().bold(),
        "List installed packages".white()
    ));
    out.push('\n');
}
