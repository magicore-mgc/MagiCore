//! Construct the CocoaPods CDN path for a named podspec version.
//! Tạo đường dẫn CDN CocoaPods cho podspec theo tên và version.

/// CocoaPods CDN URL: `{cdn}/{segment}/{name}/{version}/{name}.podspec.json`.
/// URL CDN CocoaPods: `{cdn}/{segment}/{name}/{version}/{name}.podspec.json`.
pub fn cocoapods_spec_url(cdn: &str, name: &str, version: &str) -> String {
    let segment = name
        .chars()
        .next()
        .map(|character| {
            if character.is_ascii_digit() {
                "0-9".to_string()
            } else {
                character.to_ascii_lowercase().to_string()
            }
        })
        .unwrap_or_else(|| "0-9".to_string());
    format!(
        "{}/{}/{}/{}/{}.podspec.json",
        cdn.trim_end_matches('/'),
        segment,
        name,
        version,
        name
    )
}

#[cfg(test)]
#[path = "test/spec_url.rs"]
mod tests;
