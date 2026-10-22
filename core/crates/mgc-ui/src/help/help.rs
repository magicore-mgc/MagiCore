//! Build and print help text — dựng và in văn bản trợ giúp.
//! Keep command labels, profile branches, colors and section order stable.
//! Giữ nguyên nhãn lệnh, nhánh profile, màu sắc và thứ tự section.

use colored::Colorize;

use super::sections::{
    append_common_section, append_create_section, append_options_section, append_per_core_section,
    append_single_core_section,
};

/// Build colored MagiCore help text.
/// Dựng văn bản help MagiCore có màu theo danh sách core được hỗ trợ.
pub fn custom_help_text(available_cores: &[(&str, &str)]) -> String {
    let has_web = available_cores
        .iter()
        .any(|(short_name, _)| *short_name == "web");
    let single_core = available_cores.len() == 1;
    let single_core_web = single_core && has_web;
    let build_profile = if single_core {
        let core = available_cores
            .first()
            .map(|(core, _)| *core)
            .unwrap_or("unknown");
        format!("single-core ({core})")
    } else {
        let cores = available_cores
            .iter()
            .map(|(core, _)| *core)
            .collect::<Vec<_>>()
            .join(", ");
        format!("multi-core ({cores})")
    };

    let mut out = String::new();
    out.push('\n');
    out.push_str(&format!(
        "  {}  {}\n",
        "".bold(),
        "MagiCore - Universal Package Manager".white().bold()
    ));
    out.push_str(&format!(
        "  {}\n",
        "Usage: mgc [OPTIONS] [COMMAND]".cyan().italic()
    ));
    out.push_str(&format!(
        "  {} {}\n",
        "Build:".white().bold(),
        build_profile.white()
    ));
    out.push('\n');

    append_common_section(&mut out, available_cores, single_core);
    if single_core_web {
        append_single_core_section(&mut out);
    }
    append_create_section(&mut out, available_cores, single_core_web);
    if !single_core_web {
        append_per_core_section(&mut out, available_cores);
    }
    append_options_section(&mut out, available_cores);
    out
}

/// Print colored MagiCore help to stdout.
/// In nội dung help đã dựng ra stdout mà không đổi format.
pub fn print_custom_help(available_cores: &[(&str, &str)]) {
    print!("{}", custom_help_text(available_cores));
}

#[cfg(test)]
#[path = "test/help.rs"]
mod tests;
