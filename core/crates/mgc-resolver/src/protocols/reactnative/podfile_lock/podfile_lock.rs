//! Podfile.lock values, checksum lookup, and root-pod normalization.
//! Giá trị Podfile.lock, tra checksum và chuẩn hóa tên pod gốc.

/// One versioned pod of the Podfile.lock `PODS:` closure.
/// Một pod có version thuộc bao đóng `PODS:` trong Podfile.lock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PodfileLockPod {
    pub name: String,
    pub version: String,
}

/// Parsed pod closure and its root-name to SHA-1 checksum map.
/// Bao đóng pod đã parse và map checksum SHA-1 theo tên pod gốc.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PodfileLock {
    pub pods: Vec<PodfileLockPod>,
    pub checksums: Vec<(String, String)>,
}

impl PodfileLock {
    pub fn checksum_for(&self, pod: &str) -> Option<&str> {
        self.checksums
            .iter()
            .find(|(name, _)| name == pod)
            .map(|(_, sha)| sha.as_str())
    }
}

/// Return the root pod name for a CocoaPods dependency/subspec string.
/// Trả tên pod gốc từ chuỗi dependency/subspec CocoaPods.
pub fn pod_root(name: &str) -> &str {
    name.split('/').next().unwrap_or(name)
}

#[cfg(test)]
#[path = "test/podfile_lock.rs"]
mod tests;
