//! Owns ecosystem package scoping and package/repository name validation.
//! Sở hữu namespace package theo hệ sinh thái và kiểm tra tên package/repository.

use super::super::prelude::*;
use super::super::*;

/// Build the storage and authorization key for one ecosystem package.
/// Tạo khóa lưu trữ và phân quyền riêng cho từng hệ sinh thái.
pub fn scoped_package(protocol: &str, package: &str) -> Result<String, StatusCode> {
    match TrustedProtocol::parse(protocol)? {
        TrustedProtocol::Npm => {
            validate_package(package)?;
            Ok(package.to_owned())
        }
        TrustedProtocol::Pypi => {
            let normalized = canonical_pypi_name(package)?;
            Ok(format!("pypi:{normalized}"))
        }
        TrustedProtocol::Oci => {
            validate_oci_repository(package)?;
            Ok(format!("oci:{package}"))
        }
    }
}

/// Namespace one OCI artifact repository under a canonical MagiCore core id.
/// Đặt OCI repository dưới mã core MagiCore chuẩn để trust không chéo core.
pub fn core_scoped_oci_repository(core: &str, package: &str) -> Result<String, StatusCode> {
    let core = mgc_types::Ecosystem::from_str(core).ok_or(StatusCode::BAD_REQUEST)?;
    let repository = format!("{}/{package}", core.as_str());
    validate_oci_repository(&repository)?;
    Ok(repository)
}

/// Return the PEP 503 project key used by routes, bindings, and storage.
/// Trả về khóa project theo PEP 503 dùng thống nhất ở route, binding và storage.
pub fn canonical_pypi_name(package: &str) -> Result<String, StatusCode> {
    if package.is_empty()
        || package.len() > 214
        || !package
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_.".contains(&byte))
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut normalized = String::with_capacity(package.len());
    let mut previous_separator = false;
    for byte in package.bytes() {
        if b"-_.".contains(&byte) {
            if !previous_separator {
                normalized.push('-');
            }
            previous_separator = true;
        } else {
            normalized.push(byte.to_ascii_lowercase() as char);
            previous_separator = false;
        }
    }
    Ok(normalized)
}

fn validate_oci_repository(repository: &str) -> Result<(), StatusCode> {
    if repository.is_empty() || repository.len() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }
    for component in repository.split('/') {
        let bytes = component.as_bytes();
        if bytes.is_empty()
            || !bytes[0].is_ascii_lowercase() && !bytes[0].is_ascii_digit()
            || !bytes[bytes.len() - 1].is_ascii_lowercase()
                && !bytes[bytes.len() - 1].is_ascii_digit()
            || bytes.iter().any(|byte| {
                !byte.is_ascii_lowercase() && !byte.is_ascii_digit() && !b"._-".contains(byte)
            })
            || bytes
                .windows(2)
                .any(|pair| b"._-".contains(&pair[0]) && b"._-".contains(&pair[1]))
        {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    Ok(())
}

fn validate_package(package: &str) -> Result<(), StatusCode> {
    if package.len() > 214 || package.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let valid_part = |part: &str| {
        !part.is_empty()
            && part != "."
            && part != ".."
            && part
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
    };
    let valid = if let Some(scoped) = package.strip_prefix('@') {
        let Some((scope, name)) = scoped.split_once('/') else {
            return Err(StatusCode::BAD_REQUEST);
        };
        !name.contains('/') && valid_part(scope) && valid_part(name)
    } else {
        !package.contains('/') && valid_part(package)
    };
    if !valid {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}

pub(in crate::trusted::trusted) fn validate_repository(repository: &str) -> Result<(), StatusCode> {
    let mut parts = repository.split('/');
    let valid_part = |part: &str| {
        !part.is_empty()
            && part != "."
            && part != ".."
            && part
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-_.".contains(&byte))
    };
    if let (Some(owner), Some(name), None) = (parts.next(), parts.next(), parts.next())
        && valid_part(owner)
        && valid_part(name)
    {
        return Ok(());
    }
    Err(StatusCode::BAD_REQUEST)
}
