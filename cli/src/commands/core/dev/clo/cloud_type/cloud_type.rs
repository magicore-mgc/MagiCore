use std::path::Path;

/// Cloud type từ mgc.toml `[cloud] type` hoặc manifest probe — dùng cho dev/deploy.
pub fn cloud_type(root: &Path) -> anyhow::Result<String> {
    // adapter_for is Result<Option<_>> now (P0-4): `?` carries the typed
    // registry-URL error, Ok(None) still means "no cloud project here".
    // (adapter_for là Result<Option<_>> (P0-4): `?` mang lỗi registry-URL
    // typed, Ok(None) vẫn nghĩa là "không phải project cloud".)
    let adapter = mgc_cloud_adapter::adapter_for(root)?
        .ok_or_else(|| crate::error::no_framework_detected("cloud", root))?;
    Ok(adapter.cloud_type().to_string())
}
