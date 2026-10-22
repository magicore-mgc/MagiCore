//! Dispatch per-language and shared manifest recognizers.
//! Kết nối bộ nhận diện manifest theo ngôn ngữ và dùng chung.
mod app;
mod cargo;
mod dotnet;
mod go;
mod infrastructure;
mod java;
mod node;
mod python;
mod shared;
pub(in crate::project::project::core_detection) use app::detect_app_core;
pub(in crate::project::project::core_detection) use cargo::detect_cargo_cores;
pub(in crate::project::project::core_detection) use dotnet::detect_dotnet_cores;
pub(in crate::project::project::core_detection) use go::detect_go_core;
pub(in crate::project::project::core_detection) use infrastructure::{
    add_shared_cloud_ci_markers, detect_infrastructure_markers,
};
pub(in crate::project::project::core_detection) use java::detect_jvm_cores;
pub(in crate::project::project::core_detection) use node::detect_node_core;
pub(in crate::project::project::core_detection) use python::detect_python_core;
pub(in crate::project::project::core_detection) use shared::signature_file_exists;
