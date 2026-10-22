//! Owns the reserved Gradle projection; it remains unwired pending verified lock shape. — Sở hữu projection Gradle đang để dành; chưa nối khi chưa xác minh dạng lock.

/// Extract `group:artifact` coordinates from Gradle `implementation`-family
/// declarations (`implementation("g:a:v")`, `implementation 'g:a'`).
/// (Trích tọa độ `group:artifact` từ khai báo họ `implementation` của
/// Gradle (`implementation("g:a:v")`, `implementation 'g:a'`).)
///
/// Reserved: same lock-shape caveat as [`declared_maven_deps`].
/// (Dành sẵn: cùng lưu ý dạng tên lock như [`declared_maven_deps`].)
#[allow(dead_code)]
pub fn declared_gradle_deps(build_gradle: &str) -> Vec<String> {
    const CONFIGS: &[&str] = &[
        "implementation",
        "api",
        "compileOnly",
        "runtimeOnly",
        "testImplementation",
        "testRuntimeOnly",
        "androidTestImplementation",
        "debugImplementation",
        "releaseImplementation",
    ];
    let mut names = Vec::new();
    for line in build_gradle.lines() {
        let line = line.trim().trim_end_matches(';').trim();
        let Some((config, rest)) = line.split_once([' ', '(', '\t']) else {
            continue;
        };
        if !CONFIGS.contains(&config.trim()) {
            continue;
        }
        let rest = rest.trim().trim_start_matches('(').trim();
        let quoted = rest
            .strip_prefix('"')
            .and_then(|s| s.split('"').next())
            .or_else(|| rest.strip_prefix('\'').and_then(|s| s.split('\'').next()));
        if let Some(coordinate) = quoted {
            let coordinate = coordinate.trim();
            if coordinate.contains(':') && !coordinate.is_empty() {
                names.push(coordinate.to_string());
            }
        }
    }
    names.sort();
    names.dedup();
    names
}
