//! Owns the reserved Zephyr west project projection, still unwired until native lock writing exists. — Sở hữu projection project Zephyr west đang để dành, chưa nối tới khi có ghi lock native.

/// Declared Zephyr west projects from `west.yml` (`manifest.projects[].name`).
/// (Project west Zephyr khai báo từ `west.yml` (`manifest.projects[].name`).)
///
/// Reserved: same no-native-lock-writer caveat as [`declared_unity_deps`].
/// (Dành sẵn: cùng lưu ý chưa ghi lock native như [`declared_unity_deps`].)
#[allow(dead_code)]
pub fn declared_west_deps(west_yml: &str) -> Vec<String> {
    #[derive(serde::Deserialize)]
    struct WestManifest {
        #[serde(default)]
        manifest: WestBody,
    }
    #[derive(Default, serde::Deserialize)]
    struct WestBody {
        #[serde(default)]
        projects: Vec<WestProject>,
    }
    #[derive(serde::Deserialize)]
    struct WestProject {
        #[serde(default)]
        name: String,
    }
    let Ok(parsed) = serde_yaml::from_str::<WestManifest>(west_yml) else {
        return Vec::new();
    };
    let mut names: Vec<String> = parsed
        .manifest
        .projects
        .into_iter()
        .map(|p| p.name)
        .filter(|n| !n.is_empty())
        .collect();
    names.sort();
    names.dedup();
    names
}
