//! Canonical TOML scalar/array writer wiring.
//! File này chỉ export helper ghi field và mảng TOML.

mod toml_fields;

pub(super) use toml_fields::{push_str_field, push_string_array};
