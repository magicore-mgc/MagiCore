//! infrastructure recognizer wiring.
//! Wiring bộ nhận diện infrastructure.
mod infrastructure;
pub(in crate::project::project::core_detection) use infrastructure::{
    add_shared_cloud_ci_markers, detect_infrastructure_markers,
};
