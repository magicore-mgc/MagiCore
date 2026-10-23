//! Scaffold cache identity validation.
//! Validate identity cache scaffold.

use anyhow::Result;

use crate::scaffold::spec::ScaffoldSpec;

pub(crate) fn validate_identity(spec: &ScaffoldSpec) -> Result<()> {
    let name = &spec.normalized_name;
    if name.is_empty()
        || name.contains(['\\', ':'])
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|part| part.is_empty() || part.ends_with(['.', ' ']))
    {
        return Err(super::super::super::error::template_layer_identity_invalid());
    }
    Ok(())
}
