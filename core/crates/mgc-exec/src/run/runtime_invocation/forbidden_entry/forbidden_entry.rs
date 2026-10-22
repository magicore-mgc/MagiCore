//! Resolve script and Python module operands to a forbidden manager name.
//! Ánh xạ script và Python module sang tên manager bị cấm.

use crate::allowlist::FORBIDDEN_TOOLS;

pub(in crate::run::implementation) fn forbidden_entry_name(
    candidate: &str,
    python_module: bool,
) -> Option<String> {
    let normalized = crate::allowlist::normalize_script_token(candidate)?;
    if FORBIDDEN_TOOLS.contains(&normalized.as_str()) {
        return Some(normalized);
    }
    if python_module {
        return FORBIDDEN_TOOLS
            .iter()
            .find(|name| {
                candidate.eq_ignore_ascii_case(name)
                    || candidate.get(name.len()..).is_some_and(|suffix| {
                        candidate[..name.len()].eq_ignore_ascii_case(name)
                            && suffix.starts_with('.')
                    })
            })
            .map(|name| (*name).to_string());
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
