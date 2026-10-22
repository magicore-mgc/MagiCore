//! CI/CD provider name parser module wiring.
//! Wiring module parser tên provider CI/CD.

mod provider_name;

pub(super) use provider_name::provider_from_str;
