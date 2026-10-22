//! Validate the core and package argument in the established order.
//! Kiểm tra core và package theo thứ tự hiện hành.

use super::super::error::WebLinkError;

pub(in crate::commands::core::web::links) fn require_web_package<'a>(
    adapter_name: &str,
    command: &'static str,
    package: Option<&'a str>,
) -> std::result::Result<&'a str, WebLinkError> {
    if adapter_name != "web" {
        return Err(WebLinkError::unsupported_core(command));
    }
    package.ok_or_else(|| WebLinkError::missing_package(command))
}
