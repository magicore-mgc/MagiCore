//! Runtime identity and immutable policy profiles.
//! Định danh runtime và profile policy bất biến.

mod runtime;

pub use runtime::Runtime;
pub(super) use runtime::{dangerous_flags, dangerous_permissions};
