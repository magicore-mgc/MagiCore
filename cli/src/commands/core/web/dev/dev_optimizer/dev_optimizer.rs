//! Apply the optimizer profile to the in-process dev compiler.
//! Áp profile optimizer cho compiler dev chạy trong process.
//!
//! `NODE_OPTIONS` means something only to a Node process. The Web dev server
//! compiles in-process through esbuild-rs, so the knobs that matter here are
//! the compiler's own: which target to emit, whether to keep names, and the
//! JSX runtime. Reading the same profile keeps dev and build consistent — a
//! dev server that ignored the profile would contradict the build it is meant
//! to preview.
//! `NODE_OPTIONS` chỉ có nghĩa với process Node. Dev server Web biên dịch
//! trong process qua esbuild-rs, nên các nút ở đây là của compiler: target
//! phát ra, có giữ tên không, JSX runtime nào. Đọc cùng một profile giữ dev và
//! build nhất quán — dev server bỏ qua profile sẽ mâu thuẫn với bản build mà
//! nó dùng để xem trước.

use std::path::Path;

use mgc_types::MgResult;

/// Compiler settings the dev server derives from the optimizer profile.
/// Cấu hình compiler mà dev server suy ra từ profile optimizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DevCompilerProfile {
    /// ECMAScript target the compiler emits for.
    /// Target ECMAScript mà compiler phát ra.
    pub target: String,
    /// Whether identifiers are kept (useful while debugging).
    /// Có giữ tên định danh không (hữu ích khi debug).
    pub keep_names: bool,
    /// Set when the profile could not be read, carrying the reason so the dev
    /// banner can show it instead of silently using defaults.
    /// Đặt khi không đọc được profile, kèm lý do để banner dev hiện thay vì
    /// lặng lẽ dùng mặc định.
    pub profile_note: Option<String>,
}

impl Default for DevCompilerProfile {
    fn default() -> Self {
        Self {
            target: "es2020".to_string(),
            keep_names: true,
            profile_note: None,
        }
    }
}

/// Read the optimizer profile and derive the compiler settings.
///
/// A missing profile is not an error: `mgc dev` works on a project that never
/// ran `mgc optimizer`. The note is what keeps that visible.
/// Đọc profile optimizer và suy ra cấu hình compiler. Thiếu profile không phải
/// lỗi: `mgc dev` vẫn chạy trên project chưa từng chạy `mgc optimizer`. Ghi chú
/// giữ cho điều đó hiển thị rõ.
pub(crate) fn load(root: &Path) -> MgResult<DevCompilerProfile> {
    let profile_path = root.join(".mgc-optimizer").join("profile.json");
    if !profile_path.is_file() {
        return Ok(DevCompilerProfile {
            profile_note: Some(
                "no optimizer profile; run `mgc optimizer` to derive one".to_string(),
            ),
            ..DevCompilerProfile::default()
        });
    }

    let raw = std::fs::read_to_string(&profile_path)?;
    let value: serde_json::Value = serde_json::from_str(&raw)?;

    // `profile` is the hardware class the optimizer computed; it decides the
    // compile target because a low-memory machine should not receive the
    // newest syntax the browser cannot run anyway.
    // `profile` là lớp phần cứng optimizer tính; nó quyết định target biên dịch
    // vì máy ít RAM không nên nhận cú pháp mới nhất mà trình duyệt cũng không
    // chạy được.
    let hardware_profile = value
        .get("profile")
        .and_then(|p| p.as_str())
        .unwrap_or("Balanced");
    let target = match hardware_profile {
        "HighPerformance" => "esnext",
        "LowPower" => "es2019",
        _ => "es2020",
    };

    Ok(DevCompilerProfile {
        target: target.to_string(),
        keep_names: true,
        profile_note: Some(format!(
            "optimizer profile '{hardware_profile}' → target {target}"
        )),
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
