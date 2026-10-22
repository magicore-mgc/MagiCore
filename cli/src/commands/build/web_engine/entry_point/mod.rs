#[path = "error.rs"]
mod error;
#[path = "entry_point.rs"]
mod implementation;

pub(crate) use implementation::find_entry_point;
