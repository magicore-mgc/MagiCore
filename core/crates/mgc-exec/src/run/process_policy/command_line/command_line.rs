//! Classify a command image or forbidden token from legacy command-line text.
//! Phân loại image hoặc token bị cấm từ command-line text cũ.

use super::super::{FORBIDDEN_TOOLS, process_basename};

pub(in crate::run::implementation) fn forbidden_process_name(
    command: &str,
    command_line: &str,
    exempt: &[&str],
) -> Option<String> {
    forbidden_process_match(command, command_line, exempt).map(|(name, _, _, _)| name)
}

pub(in crate::run::implementation) fn forbidden_process_match(
    command: &str,
    command_line: &str,
    exempt: &[&str],
) -> Option<(String, &'static str, Option<String>, Option<usize>)> {
    let exempt = |name: &str| exempt.contains(&name);
    let process_name = process_basename(command);
    if FORBIDDEN_TOOLS.contains(&process_name.as_str()) && !exempt(&process_name) {
        return Some((process_name, "executable image", None, None));
    }

    command_line
        .split_whitespace()
        .enumerate()
        .find_map(|(index, token)| {
            let name = crate::allowlist::normalize_script_token(token)?;
            (FORBIDDEN_TOOLS.contains(&name.as_str()) && !exempt(&name)).then(|| {
                (
                    name,
                    "command-line token",
                    Some(process_basename(token)),
                    Some(index),
                )
            })
        })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
