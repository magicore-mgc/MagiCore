//! Template fetch, integrity, and publication diagnostics.
//! Lỗi tải, toàn vẹn và publish template.

mod error;

pub(crate) use error::{TemplateHttpStatus, TemplateTransportUnavailable};
pub use error::{
    no_web_layer_found, template_archive_layout_invalid, template_cache_rollback_failed,
    template_download_status, template_download_too_large, template_download_url_invalid,
    template_integrity_invalid, template_integrity_mismatch, template_not_published,
    template_publish_auth_required, template_publish_status, template_redirect_invalid,
    template_request_failed, template_transport_unavailable, web_templates_missing,
};
