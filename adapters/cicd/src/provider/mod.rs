//! CI/CD provider module wiring and stable exports.
//! Wiring module provider CI/CD và các export ổn định.

mod detection;
mod manifest_detection;
mod provider_type;

pub use detection::detect_provider;
pub(crate) use manifest_detection::manifest_is_cicd;
pub use provider_type::CicdProvider;
pub use provider_type::provider_from_str;
