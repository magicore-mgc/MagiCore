//! Escapes TOML basic strings without locale or platform dependence.
//! File này chỉ nối và export owner toml_escaping.

mod toml_escaping;

pub(super) use toml_escaping::escape_toml_string;
