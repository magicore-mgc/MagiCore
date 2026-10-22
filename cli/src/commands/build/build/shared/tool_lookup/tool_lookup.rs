/// PATH lookup with Windows PATHEXT awareness: on top of the exact
/// name, `<tool><ext>` is accepted for every extension in PATHEXT
/// (`.BAT`/`.CMD`/`.EXE` wrappers like `flutter.bat`). PATHEXT is read
/// whenever present (Windows always sets it) so the lookup is
/// unit-testable on every OS; without it, Windows falls back to the
/// classic default set and other platforms check the exact name only.
/// (Tìm PATH có nhận biết PATHEXT Windows.)
pub(crate) fn tool_unavailable(tool: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return true;
    };
    let extensions: Vec<String> = std::env::var_os("PATHEXT")
        .and_then(|value| value.into_string().ok())
        .map(|value| {
            value
                .split(';')
                .filter(|extension| !extension.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_else(|| {
            #[cfg(windows)]
            {
                vec![
                    ".COM".to_string(),
                    ".EXE".to_string(),
                    ".BAT".to_string(),
                    ".CMD".to_string(),
                ]
            }
            #[cfg(not(windows))]
            {
                Vec::new()
            }
        });

    std::env::split_paths(&path).all(|directory| {
        let directory = directory.as_path();
        if directory.join(tool).is_file() {
            return false;
        }
        !extensions.iter().any(|extension| {
            let candidate = directory.join(format!("{tool}{extension}"));
            if candidate.is_file() {
                return true;
            }
            // Windows PATHEXT matching is case-insensitive (a `.bat`
            // shim satisfies `.BAT`) — honor that on case-sensitive
            // filesystems too, or Linux checkouts miss what Windows sees.
            // (PATHEXT Windows không phân biệt hoa thường.)
            let want = format!("{tool}{extension}").to_lowercase();
            std::fs::read_dir(directory)
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name().to_string_lossy().to_lowercase() == want)
        })
    })
}

/// Resolve the Python launcher name for this machine (P0 fix,
/// 2026-09-12): macOS ships only `python3` (no `python` shim since
/// Monterey) while setup-python CI images provide `python`. The
/// honest lifecycle matrix caught the hardcoded `python` failing on
/// macOS — build/install lanes must resolve the launcher that EXISTS,
/// never guess one.
/// Chọn tên launcher Python của máy này (P0 fix, 2026-09-12): macOS chỉ
/// có `python3` (không còn shim `python` từ Monterey) còn image CI
/// setup-python có `python`. Matrix lifecycle trung thực đã bắt được
/// hardcoded `python` fail trên macOS — lane build/install phải chọn
/// launcher TỒN TẠI, không đoán.
pub(crate) fn python_cmd() -> &'static str {
    if !tool_unavailable("python") {
        "python"
    } else if !tool_unavailable("python3") {
        "python3"
    } else {
        "python" // Neither exists — surface the honest toolchain error.
    }
}
