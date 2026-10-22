//! Manifest signature recognition owners.
//! Các owner nhận diện chữ ký manifest.
mod signatures;
pub(super) use signatures::{
    add_shared_cloud_ci_markers, detect_app_core, detect_cargo_cores, detect_dotnet_cores,
    detect_go_core, detect_infrastructure_markers, detect_jvm_cores, detect_node_core,
    detect_python_core, signature_file_exists,
};
