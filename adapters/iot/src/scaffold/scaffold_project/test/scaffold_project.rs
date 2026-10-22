#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Board validation must reject unsupported combinations before creating files.
//! Xác thực board phải từ chối tổ hợp không hỗ trợ trước khi tạo file.

use crate::framework::IotFramework;
use crate::scaffold::scaffold_project;
use crate::scaffold::test::support::tmp;

#[tokio::test]
async fn scaffold_rejects_unknown_board_before_creating_project_files() {
    let temp = tmp();
    let project = temp.path().join("firmware");
    let error = scaffold_project(
        IotFramework::Esp32Rust,
        "firmware",
        "unknown-board",
        &project,
    )
    .await
    .expect_err("unknown boards must be rejected");

    assert!(matches!(error, mgc_types::MgError::Unsupported { .. }));
    assert!(
        !project.exists(),
        "rejection must happen before filesystem writes"
    );
}

#[tokio::test]
async fn scaffold_rejects_board_from_another_framework() {
    let temp = tmp();
    let project = temp.path().join("firmware");
    let error = scaffold_project(
        IotFramework::Esp32Rust,
        "firmware",
        "nrf52dk_nrf52832",
        &project,
    )
    .await
    .expect_err("framework mismatch must be rejected");

    assert!(matches!(error, mgc_types::MgError::Unsupported { .. }));
    assert!(
        !project.exists(),
        "rejection must happen before filesystem writes"
    );
}
