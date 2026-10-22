//! Scaffold request types and parsing API.
//! Kiểu dữ liệu và API phân tích yêu cầu scaffold.

mod artifact_name;
mod core_kind;
mod error;
mod normalize_name;
mod parse;
mod scaffold_ref;
mod spec;

pub use artifact_name::artifact_name;
pub use core_kind::CoreKind;
pub use parse::parse_scaffold_spec;
pub use scaffold_ref::ScaffoldRef;
pub use spec::ScaffoldSpec;

pub(in crate::scaffold::spec) use error::ScaffoldSpecError;
