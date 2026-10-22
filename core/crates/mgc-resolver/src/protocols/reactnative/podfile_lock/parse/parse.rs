//! Parse the pinned pod closure and `SPEC CHECKSUMS` section.
//! Parse bao đóng pod đã ghim và section `SPEC CHECKSUMS`.

use super::super::{PodfileLock, PodfileLockPod};

/// Parse Podfile.lock while excluding nested edges and local-only pods.
/// Parse Podfile.lock, bỏ cạnh lồng và pod chỉ tồn tại cục bộ.
pub fn parse_podfile_lock(text: &str) -> PodfileLock {
    let mut lock = PodfileLock::default();
    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Section {
        None,
        Pods,
        Checksums,
    }
    let mut section = Section::None;
    for raw in text.lines() {
        if raw.trim().is_empty() {
            continue;
        }
        if !raw.starts_with(' ') && !raw.starts_with('\t') {
            section = match raw.trim() {
                "PODS:" => Section::Pods,
                "SPEC CHECKSUMS:" => Section::Checksums,
                _ => Section::None,
            };
            continue;
        }
        match section {
            Section::Pods => {
                let trimmed = raw.trim_start();
                let indent = raw.len() - trimmed.len();
                if indent != 2 || !trimmed.starts_with("- ") {
                    continue;
                }
                let body = trimmed.trim_start_matches("- ").trim();
                if let Some((name, version)) = pod_name_version(body) {
                    let root = name.split('/').next().unwrap_or(&name).to_string();
                    if !lock.pods.iter().any(|pod| pod.name == root) {
                        lock.pods.push(PodfileLockPod {
                            name: root,
                            version,
                        });
                    }
                }
            }
            Section::Checksums => {
                let trimmed = raw.trim();
                if let Some((name, sha)) = trimmed.split_once(':') {
                    let name = name.trim().to_string();
                    let sha = sha.trim().to_ascii_lowercase();
                    if !name.is_empty() && !sha.is_empty() {
                        lock.checksums.push((name, sha));
                    }
                }
            }
            Section::None => {}
        }
    }
    lock
}

fn pod_name_version(body: &str) -> Option<(String, String)> {
    let open = body.find('(')?;
    let close = body[open..].find(')')? + open;
    let version = body[open + 1..close].trim();
    if version.starts_with('=') || version.starts_with("from") || version.contains(' ') {
        return None;
    }
    let name = body[..open].trim().trim_end_matches(':').trim();
    if name.is_empty() {
        return None;
    }
    Some((name.to_string(), version.to_string()))
}

#[cfg(test)]
#[path = "test/parse.rs"]
mod tests;
