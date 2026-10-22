//! Owns the reserved PlatformIO lib_deps projection, still unwired until native lock writing exists. — Sở hữu projection lib_deps PlatformIO đang để dành, chưa nối tới khi có ghi lock native.

/// Declared PlatformIO libraries from `platformio.ini` (`lib_deps` values,
/// first token each — owner/lib, URL or path as written).
/// (Thư viện PlatformIO khai báo từ `platformio.ini` (giá trị `lib_deps`,
/// token đầu mỗi cái).)
///
/// Reserved: same no-native-lock-writer caveat as [`declared_unity_deps`].
/// (Dành sẵn: cùng lưu ý chưa ghi lock native như [`declared_unity_deps`].)
#[allow(dead_code)]
pub fn declared_platformio_deps(platformio_ini: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_env = false;
    // Multi-line `lib_deps` values continue on following indented lines
    // until the next key or section.
    // (Giá trị `lib_deps` nhiều dòng nối ở dòng thụt đầu dòng sau, tới key
    // hoặc section tiếp theo.)
    let mut collecting = false;
    for raw_line in platformio_ini.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_env = line.starts_with("[env:");
            collecting = false;
            continue;
        }
        if !in_env {
            continue;
        }
        let mut value = if let Some(rest) = line
            .strip_prefix("lib_deps")
            .and_then(|rest| rest.trim().strip_prefix('='))
        {
            collecting = true;
            rest
        } else if collecting && !line.is_empty() && !line.contains('=') {
            line
        } else {
            if !line.is_empty() {
                collecting = false;
            }
            continue;
        };
        value = value.trim();
        for token in value.split([',', ' ', '\t']).map(str::trim) {
            let token = token.trim_end_matches('\\').trim();
            // A lone `@` only separates `owner/lib @ version` pairs.
            // (`@` đơn chỉ ngăn cặp `owner/lib @ version`.)
            if token.is_empty() || token == "@" || token.starts_with([';', '#']) {
                continue;
            }
            names.push(token.to_string());
        }
    }
    names.sort();
    names.dedup();
    names
}
