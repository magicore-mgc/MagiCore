//! Package inclusion policy — chính sách include/exclude file package.

mod include_rules;

pub(super) use include_rules::{ALWAYS_INCLUDE, should_include};
