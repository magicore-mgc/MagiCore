//! Forbidden script/module classification exports.
//! Export phân loại script/module bị cấm.

#[path = "forbidden_entry.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::forbidden_entry_name;
