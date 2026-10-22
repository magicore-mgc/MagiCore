//! Shared OSV package and response types.
//! Kiểu package và response OSV dùng chung.

mod types;
pub use types::OsvPin;
pub(crate) use types::{OsvQueryResponse, OsvVuln};
