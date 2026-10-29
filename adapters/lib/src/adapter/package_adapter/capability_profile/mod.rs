//! Capability claims for language-specific Lib lanes.
//! Claim capability cho từng lane ngôn ngữ Lib.

mod capability_profile;

pub(in crate::adapter) use capability_profile::select_capabilities;
