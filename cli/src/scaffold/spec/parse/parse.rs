//! Parse `<name>[@<ref>]` into the shared scaffold request model.
//! Parse `<name>[@<ref>]` thành model yêu cầu scaffold dùng chung.

use anyhow::Result;

use super::super::normalize_name::normalize_template_name;
use super::super::{CoreKind, ScaffoldRef, ScaffoldSpec, ScaffoldSpecError};

/// Parse a scaffold request using the existing reference heuristic.
/// Parse yêu cầu scaffold theo heuristic reference hiện có.
pub fn parse_scaffold_spec(core: CoreKind, input: &str) -> Result<ScaffoldSpec> {
    let (name, reference) = if let Some(position) = input.find('@') {
        let (name, suffix) = input.split_at(position);
        (name, Some(&suffix[1..]))
    } else {
        (input, None)
    };

    if name.is_empty() {
        return Err(ScaffoldSpecError::TemplateNameEmpty.into_anyhow());
    }

    let requested_ref = match reference {
        None | Some("") => ScaffoldRef::Default,
        Some(value) => {
            if value.chars().next().unwrap_or('a').is_ascii_digit() {
                ScaffoldRef::Version(value.to_string())
            } else if value.starts_with('^') || value.starts_with('~') {
                ScaffoldRef::Range(value.to_string())
            } else {
                ScaffoldRef::DistTag(value.to_string())
            }
        }
    };

    requested_ref.validate()?;
    let normalized_name = normalize_template_name(name);

    Ok(ScaffoldSpec {
        core,
        name: name.to_string(),
        normalized_name,
        requested_ref,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
