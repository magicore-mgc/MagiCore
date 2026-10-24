//! Official initializer owner: catalog, data contract and policy.
//! Owner initializer chính thức: catalog, contract dữ liệu và policy.

#[path = "catalog/mod.rs"]
mod catalog;
#[path = "initializers.rs"]
mod implementation;
#[path = "policy/mod.rs"]
mod policy;
#[path = "types/mod.rs"]
mod types;

pub(in crate::commands::core::web::implementation) use implementation::lookup;
pub(in crate::commands::core::web::implementation) use policy::check;
pub(in crate::commands::core::web::implementation) use types::{ArgSlot, InitializerRecord, Mode};
