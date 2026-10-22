mod error;
mod http;

pub(super) use error::{HttpError, HttpResult};
pub(super) use http::{get_bytes, get_text};
