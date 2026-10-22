//! Shared error facade for root and compatibility diagnostics.
//! Facade lỗi dùng chung cho lỗi root và diagnostic tương thích.

pub mod messages;

pub use messages::*;
pub(crate) use messages::{TemplateHttpStatus, TemplateTransportUnavailable};
