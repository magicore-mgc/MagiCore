//! Existing metadata is reused without another network request.
//! Metadata sẵn có được dùng lại mà không tạo request mạng mới.

use super::super::{PypiInfo, PypiProtocol, load_selected_metadata};
use mgc_types::Version;

#[tokio::test]
async fn reuses_metadata_loaded_for_python_compatibility() {
    let protocol = PypiProtocol::new("http://127.0.0.1:1");
    let selected = PypiInfo {
        requires_dist: Some(vec!["dep-a>=1".to_string()]),
        requires_python: Some(">=3.9".to_string()),
    };

    let observed = load_selected_metadata(
        &protocol,
        "demo",
        &Version::parse("1.0.0").expect("test operation must succeed"),
        Some(selected),
    )
    .await
    .expect("test operation must succeed");

    assert_eq!(observed.requires_dist, Some(vec!["dep-a>=1".to_string()]));
    assert_eq!(observed.requires_python.as_deref(), Some(">=3.9"));
}
