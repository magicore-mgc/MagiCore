//! Try each core's built-in adapter generator, writing files offline.
//! Thử generator adapter có sẵn của từng core, ghi file offline.
//!
//! This is the LOCAL GENERATOR path: it writes project files from the adapter's
//! own templates and never starts west/pio/godot/terraform. Callers invoke it
//! before the framework's host-tool initializer and before the registry layer,
//! so an offline machine still gets a project.
//! Đây là đường LOCAL GENERATOR: ghi file project từ template của adapter và
//! KHÔNG chạy west/pio/godot/terraform. Caller gọi TRƯỚC initializer host-tool
//! của framework và trước registry layer, để máy offline vẫn có project.

use std::path::Path;

use anyhow::Result;

/// Local-generator inputs resolved at the CLI boundary.
/// Đầu vào local generator đã phân giải tại biên CLI.
pub(crate) struct LocalScaffoldRequest<'a> {
    /// CLI core id (`iot`, `game`, `clo`).
    /// Core id CLI (`iot`, `game`, `clo`).
    pub core: &'a str,
    /// Canonical framework/engine/provider name for the core.
    /// Tên framework/engine/provider chuẩn cho core.
    pub framework: &'a str,
    /// Target directory (already validated by the create lane).
    /// Thư mục đích (đã validate ở create lane).
    pub target: &'a Path,
    /// IoT board id, when the core needs one; ignored by other cores.
    /// Board id IoT khi core cần; core khác bỏ qua.
    pub board: Option<&'a str>,
}

/// Write a project using the adapter's built-in generator.
///
/// Returns `Ok(true)` when a generator ran and files were written, `Ok(false)`
/// when the core/framework has no local generator and the caller should
/// continue with the registry layer.
/// Ghi project bằng generator có sẵn của adapter. Trả `Ok(true)` khi generator
/// đã chạy; `Ok(false)` khi core/framework không có local generator và caller
/// nên tiếp tục với registry layer.
pub(crate) async fn try_local_scaffold(request: &LocalScaffoldRequest<'_>) -> Result<bool> {
    // Fail closed on an existing destination: the adapter would merge files into
    // whatever is already there, and a create must never silently touch an
    // existing directory. The caller's registry path reports the same conflict.
    // (Fail-closed khi đích đã tồn tại: adapter sẽ hòa file vào thư mục sẵn có,
    // mà create không được lặng lẽ chạm thư mục đã có. Đường registry của
    // caller báo cùng lỗi xung đột.)
    if request.target.try_exists()? {
        return Err(crate::error::dir_already_exists(request.target));
    }

    match request.core {
        "iot" => try_scaffold_iot(request).await,
        "game" => try_scaffold_game(request).await,
        "clo" => try_scaffold_cloud(request).await,
        // Other cores own their own fallbacks; no local generator here.
        // Core khác tự lo fallback riêng; không có local generator ở đây.
        _ => Ok(false),
    }
}

#[cfg(feature = "iot")]
async fn try_scaffold_iot(request: &LocalScaffoldRequest<'_>) -> Result<bool> {
    let Some(framework) = mgc_iot_adapter::IotFramework::parse(request.framework) else {
        return Ok(false);
    };
    let Some(board) = request.board else {
        return Ok(false);
    };
    mgc_iot_adapter::scaffold::scaffold_project(
        framework,
        project_name(request),
        board,
        request.target,
    )
    .await?;
    Ok(true)
}

#[cfg(not(feature = "iot"))]
async fn try_scaffold_iot(_request: &LocalScaffoldRequest<'_>) -> Result<bool> {
    Ok(false)
}

#[cfg(feature = "game")]
async fn try_scaffold_game(request: &LocalScaffoldRequest<'_>) -> Result<bool> {
    let Some(engine) = mgc_game_adapter::GameEngine::parse(request.framework) else {
        return Ok(false);
    };
    let context = mgc_game_adapter::scaffold::TemplateContext::new(project_name(request), engine);
    mgc_game_adapter::scaffold::scaffold_project(engine, context, request.target).await?;
    Ok(true)
}

#[cfg(not(feature = "game"))]
async fn try_scaffold_game(_request: &LocalScaffoldRequest<'_>) -> Result<bool> {
    Ok(false)
}

#[cfg(feature = "clo")]
async fn try_scaffold_cloud(request: &LocalScaffoldRequest<'_>) -> Result<bool> {
    let Some(cloud) = mgc_cloud_adapter::CloudType::parse(request.framework) else {
        return Ok(false);
    };
    mgc_cloud_adapter::scaffold::scaffold_project(cloud, project_name(request), request.target)
        .await?;
    Ok(true)
}

#[cfg(not(feature = "clo"))]
async fn try_scaffold_cloud(_request: &LocalScaffoldRequest<'_>) -> Result<bool> {
    Ok(false)
}

/// Project name from the target directory's final component.
/// Tên project lấy từ thành phần cuối của thư mục đích.
fn project_name<'a>(request: &LocalScaffoldRequest<'a>) -> &'a str {
    request
        .target
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
}
