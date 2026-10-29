//! Mapping transport/HTTP failures onto scaffold resolve errors.
//! Map lỗi transport/HTTP sang lỗi resolve scaffold.

use super::super::error;

pub(in crate::commands::template) fn template_resolution_error(
    error: anyhow::Error,
    layer: &str,
    core: &str,
    name: &str,
    tag: &str,
) -> crate::scaffold::resolver::ScaffoldResolveError {
    use crate::scaffold::resolver::ScaffoldResolveError;
    if error.is::<error::TemplateTransportUnavailable>() {
        return ScaffoldResolveError::RegistryUnavailable {
            registry: "configured template registry".to_string(),
            offline: false,
        };
    }
    if let Some(status) = error.downcast_ref::<error::TemplateHttpStatus>() {
        if matches!(status.0, 404 | 410) {
            return ScaffoldResolveError::RequiredLayerMissing {
                layer: layer.to_string(),
                core: core.to_string(),
                template: name.to_string(),
                tag: tag.to_string(),
                attempted_sources: error.to_string(),
            };
        }
        if status.0 == mgc_types::limits::HTTP_TOO_MANY_REQUESTS
            || status.0 >= mgc_types::limits::HTTP_SERVER_ERROR_MIN
        {
            return ScaffoldResolveError::RegistryUnavailable {
                registry: format!("configured template registry (HTTP {})", status.0),
                offline: false,
            };
        }
    }
    ScaffoldResolveError::Other(error.to_string())
}
