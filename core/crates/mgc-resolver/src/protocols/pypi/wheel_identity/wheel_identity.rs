use mgc_types::{MgError, MgResult};

/// Confirm wheel metadata identity before its files enter Python's import path.
/// Xác nhận METADATA khớp tên/version trong lock trước khi thêm PYTHONPATH.
pub(in crate::protocols::pypi) fn verify_wheel_identity(
    metadata: &[u8],
    expected_name: &str,
    expected_version: &str,
) -> MgResult<()> {
    let metadata = std::str::from_utf8(metadata)
        .map_err(|_| MgError::Integrity("Python wheel METADATA is not UTF-8".to_string()))?;
    let mut name = None;
    let mut version = None;
    for line in metadata.lines().take_while(|line| !line.is_empty()) {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        match key.to_ascii_lowercase().as_str() {
            "name" => {
                if name.replace(value.trim()).is_some() {
                    return Err(MgError::Integrity(
                        "Python wheel METADATA repeats Name".to_string(),
                    ));
                }
            }
            "version" if version.replace(value.trim()).is_some() => {
                return Err(MgError::Integrity(
                    "Python wheel METADATA repeats Version".to_string(),
                ));
            }
            "version" => version = Some(value.trim()),
            _ => {}
        }
    }
    let normalize = |value: &str| {
        let mut normalized = String::with_capacity(value.len());
        let mut separator = false;
        for ch in value.chars() {
            if matches!(ch, '-' | '_' | '.') {
                separator = true;
            } else {
                if separator && !normalized.is_empty() {
                    normalized.push('-');
                }
                separator = false;
                normalized.push(ch.to_ascii_lowercase());
            }
        }
        normalized
    };
    if name.is_none_or(|actual| normalize(actual) != normalize(expected_name))
        || version.is_none_or(|actual| !pep440_release_identity_matches(actual, expected_version))
    {
        return Err(MgError::Integrity(format!(
            "Python wheel METADATA identity does not match locked {}@{}",
            expected_name, expected_version
        )));
    }
    Ok(())
}

/// Compare stable PEP 440 release segments while preserving fail-closed
/// behavior for epochs, pre/post/dev releases, local labels, and malformed
/// strings that this resolver does not model yet. PEP 440 treats trailing
/// zero release segments as equivalent (for example, `26.3 == 26.3.0`).
/// So khớp release segment ổn định theo PEP 440; từ chối cú pháp chưa hỗ trợ.
pub(in crate::protocols::pypi) fn pep440_release_identity_matches(
    actual: &str,
    expected: &str,
) -> bool {
    fn release_parts(value: &str) -> Option<Vec<u64>> {
        let value = value.trim();
        let value = value
            .strip_prefix('v')
            .or_else(|| value.strip_prefix('V'))
            .unwrap_or(value);
        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte == b'.')
        {
            return None;
        }
        let mut parts = value
            .split('.')
            .map(|part| {
                if part.is_empty() {
                    return None;
                }
                part.parse::<u64>().ok()
            })
            .collect::<Option<Vec<_>>>()?;
        while parts.last() == Some(&0) && parts.len() > 1 {
            parts.pop();
        }
        Some(parts)
    }

    match (release_parts(actual), release_parts(expected)) {
        (Some(actual), Some(expected)) => actual == expected,
        _ => false,
    }
}
