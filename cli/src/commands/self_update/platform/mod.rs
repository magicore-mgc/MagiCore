//! Map the host and build variant to release artifact coordinates.
//! Ánh xạ hệ điều hành, kiến trúc và biến thể build thành tọa độ artifact.

mod platform;
pub(in crate::commands) use platform::detect_variant;
pub use platform::{artifact_name, host_labels};
