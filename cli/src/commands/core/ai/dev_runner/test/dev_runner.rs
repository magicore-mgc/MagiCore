//! Tests for the AI dev runner's pre-spawn lifecycle gate.
//! Test gate lifecycle trước spawn của runner dev AI.

use super::super::run_ai_dev_at;

#[tokio::test]
async fn unsupported_python_lane_is_refused_before_dev_execution() {
    let project = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write supported Python manifest");
    std::fs::write(project.path().join("requirements.txt"), "requests==2.0\n")
        .expect("write legacy requirements");

    let error = run_ai_dev_at(project.path()).await.unwrap_err();
    assert!(
        error
            .to_string()
            .contains("No external package manager was invoked")
    );
}
