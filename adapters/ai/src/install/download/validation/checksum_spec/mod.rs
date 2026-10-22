//! Checksum prefix, size and hexadecimal grammar.
//! Cú pháp prefix checksum, độ dài và ký tự hex.

mod checksum_spec;
pub(in crate::install::download) use checksum_spec::validate_checksum_spec;
