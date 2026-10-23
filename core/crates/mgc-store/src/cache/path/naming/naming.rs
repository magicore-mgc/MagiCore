//! Portable-name and version-identity hashing for cache paths.
//! Hash tên portable và identity version cho đường dẫn cache.

use mgc_types::Version;

pub(crate) fn is_legacy_portable_name(name: &str) -> bool {
    if name.is_empty()
        || !name.is_ascii()
        || name.starts_with('.')
        || name.ends_with('.')
        || name.ends_with(' ')
        || name.contains('~')
        || is_windows_reserved_device_name(name)
    {
        return false;
    }

    name.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'_' | b'-' | b'.' | b'@' | b'!')
    })
}

pub(crate) fn is_windows_reserved_device_name(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name);
    if matches!(stem, "con" | "prn" | "aux" | "nul") {
        return true;
    }

    let bytes = stem.as_bytes();
    bytes.len() == 4
        && (stem.starts_with("com") || stem.starts_with("lpt"))
        && matches!(bytes[3], b'1'..=b'9')
}

pub(crate) fn is_legacy_portable_version(version: &Version) -> bool {
    let rendered = version.to_string();
    if rendered.is_empty()
        || !rendered.is_ascii()
        || rendered.ends_with('.')
        || !rendered.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'-' | b'+' | b'_')
        })
    {
        return false;
    }

    matches!(Version::parse(&rendered), Ok(parsed) if &parsed == version)
}

pub(crate) fn hash_version_identity(version: &Version) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&version.major.to_be_bytes());
    hasher.update(&version.minor.to_be_bytes());
    hasher.update(&version.patch.to_be_bytes());
    hash_optional_string(&mut hasher, version.pre.as_deref());
    hash_optional_string(&mut hasher, version.build.as_deref());
    hash_optional_number(&mut hasher, version.dev);
    hash_optional_number(&mut hasher, version.post);
    hasher.finalize().to_hex().to_string()
}

fn hash_optional_string(hasher: &mut blake3::Hasher, value: Option<&str>) {
    match value {
        Some(value) => {
            hasher.update(&[1]);
            hasher.update(&(value.len() as u128).to_be_bytes());
            hasher.update(value.as_bytes());
        }
        None => {
            hasher.update(&[0]);
        }
    }
}

fn hash_optional_number(hasher: &mut blake3::Hasher, value: Option<u64>) {
    match value {
        Some(value) => {
            hasher.update(&[1]);
            hasher.update(&value.to_be_bytes());
        }
        None => {
            hasher.update(&[0]);
        }
    }
}
