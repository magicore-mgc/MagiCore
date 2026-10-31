//! PyPI protocol ownership tree.
//! Cây owner protocol PyPI.

mod age_gate;
mod marker_evaluation;
mod materialization;
mod metadata;
mod pep440;
mod pep508;
mod platform_compatibility;
mod pypi;
mod python_compatibility;
mod request;
mod requirements_lock;
mod resolution;
mod wheel_identity;
mod wheel_record;
mod wheel_selection;

pub(in crate::protocols::pypi) use age_gate::{AgeGateExclusion, PypiAgeGate};
pub(in crate::protocols::pypi) use marker_evaluation::marker_applies;
#[cfg(test)]
pub(in crate::protocols::pypi) use metadata::PypiDigests;
pub(in crate::protocols::pypi) use metadata::{PypiFile, PypiInfo, PypiJson};
pub(in crate::protocols::pypi) use pep440::pep440_matches;
pub(in crate::protocols::pypi) use pep508::{parse_pep508, requires_requested_extras};
pub(in crate::protocols::pypi) use platform_compatibility::{
    host_platform_version, platform_matches_host, platform_tag_matches_target,
};
pub use pypi::PypiProtocol;
pub(in crate::protocols::pypi) use python_compatibility::{
    InterpreterAbiFlavor, abi_compatible_for_interpreter, consumer_python, requires_python_allows,
};
pub(in crate::protocols::pypi) use wheel_identity::verify_wheel_identity;
pub(in crate::protocols::pypi) use wheel_selection::{select_file, wheel_tags};
