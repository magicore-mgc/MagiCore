use super::error;
use anyhow::Result;

pub(in crate::commands::template) fn validate_template_identity(
    core: &str,
    name: &str,
) -> Result<()> {
    if crate::scaffold::spec::CoreKind::from_str_core(core).is_none()
        || name.is_empty()
        || name.contains(['\\', ':'])
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(error::template_layer_identity_invalid());
    }
    Ok(())
}
