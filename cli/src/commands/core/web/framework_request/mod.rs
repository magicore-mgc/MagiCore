// Framework request wiring — nối parser và kiểu yêu cầu framework.
// Existing request entry points remain available at the Web facade.
use super::*;
#[path = "framework_request.rs"]
mod implementation;
pub(crate) use implementation::{FrameworkRequest, parse_framework_request};
pub(super) use implementation::{apply_framework_alias_defaults, framework_request_from_spec};
