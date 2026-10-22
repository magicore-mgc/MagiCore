//! Render common help commands — dựng section lệnh dùng chung.
//! Keep single-core installation and per-core installation labels distinct.
//! Giữ nhãn cài đặt single-core và từng core theo đúng nhánh cũ.

use colored::Colorize;

use super::super::command_name::per_core_name;

pub(in crate::help) fn append_common_section(
    out: &mut String,
    available_cores: &[(&str, &str)],
    single_core: bool,
) {
    out.push_str(&format!("  {}\n", "── Common ──".white().bold()));
    out.push_str(&format!(
        "    {}   {}\n",
        "init".cyan().bold(),
        "Interactive project wizard".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "dev".cyan().bold(),
        "Start the local development server".white()
    ));
    if single_core {
        out.push_str(&format!(
            "    {}   {}\n",
            "install".cyan().bold(),
            "Install all dependencies".white()
        ));
    } else {
        for (core, label) in available_cores {
            out.push_str(&format!(
                "    {}   {}\n",
                per_core_name("install", core).cyan().bold(),
                format!("Install dependencies for {}", label).white()
            ));
        }
    }
    out.push_str(&format!(
        "    {}   {}\n",
        "info".cyan().bold(),
        "Show package information".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "search".cyan().bold(),
        "Search for packages".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "outdated".cyan().bold(),
        "Check for outdated packages".white()
    ));
    out.push_str(&format!(
        "    {}   {}\n",
        "audit".cyan().bold(),
        "Audit packages for vulnerabilities".white()
    ));
    out.push('\n');
}
