//! Validate each RECORD row against the authenticated wheel and installed file.
//! Đối chiếu từng dòng RECORD với wheel xác thực và file đã cài.

use super::super::archive_index::ArchiveIndex;
use super::super::record_csv::parse_record_row;
use super::super::record_paths::{validate_record_path, verified_site_file_path};
use base64::Engine;
use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::io::Read;
use std::path::Path;

const STREAM_BUFFER_SIZE: usize = 64 * 1024;

pub(in crate::protocols::pypi::wheel_record) fn verify_record_members(
    site: &Path,
    archive: &ArchiveIndex,
    record_path: &str,
    record: &str,
) -> MgResult<BTreeSet<String>> {
    let urlsafe = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let mut expected_files = BTreeSet::new();

    for (line_index, line) in record.lines().enumerate() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        let fields = parse_record_row(line).map_err(|reason| {
            MgError::Integrity(format!(
                "malformed authenticated wheel RECORD line {}: {reason}",
                line_index + 1
            ))
        })?;
        let [relative, hash, size] = fields.as_slice() else {
            return Err(MgError::Integrity(format!(
                "malformed authenticated wheel RECORD line {}",
                line_index + 1
            )));
        };
        validate_record_path(relative)?;
        let Some(archive_member) = archive.member_bytes(relative) else {
            return Err(MgError::Integrity(format!(
                "wheel RECORD references absent member '{relative}'"
            )));
        };
        if !expected_files.insert(relative.clone()) {
            return Err(MgError::Integrity(format!(
                "wheel RECORD repeats member '{relative}'"
            )));
        }

        let target = verified_site_file_path(site, relative)?;
        let (actual_size, actual_digest) =
            compare_installed_member(&target, relative, archive_member)?;
        let is_record = relative.as_str() == record_path;
        if is_record {
            if !hash.is_empty() || !size.is_empty() {
                return Err(MgError::Integrity(
                    "wheel RECORD must not self-hash its RECORD row".to_string(),
                ));
            }
            continue;
        }

        let expected_digest = hash.strip_prefix("sha256=").ok_or_else(|| {
            MgError::Integrity(format!(
                "unsupported or missing wheel RECORD hash for '{relative}'"
            ))
        })?;
        let expected_size: u64 = size.parse().map_err(|_| {
            MgError::Integrity(format!("invalid wheel RECORD size for '{relative}'"))
        })?;
        if actual_size != expected_size {
            return Err(MgError::Integrity(format!(
                "Python runtime materialization size mismatch for '{relative}'"
            )));
        }
        let expected_digest = urlsafe.decode(expected_digest).map_err(|_| {
            MgError::Integrity(format!("invalid wheel RECORD digest for '{relative}'"))
        })?;
        if actual_digest.as_slice() != expected_digest.as_slice() {
            return Err(MgError::Integrity(format!(
                "Python runtime materialization digest mismatch for '{relative}'"
            )));
        }
    }

    Ok(expected_files)
}

fn compare_installed_member(
    target: &Path,
    relative: &str,
    archive_member: &[u8],
) -> MgResult<(u64, [u8; 32])> {
    let mut file =
        mgc_platform::open_regular_file_no_follow(target).map_err(|error| match error {
            mgc_platform::OpenRegularFileError::Io(source) => MgError::Io(source),
            other => MgError::Integrity(format!(
                "Python runtime materialization contains an unsafe file '{relative}': {other}"
            )),
        })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; STREAM_BUFFER_SIZE];
    let mut total = 0_u64;

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        let start = usize::try_from(total).map_err(|_| {
            MgError::Integrity(format!(
                "Python runtime materialization is too large for '{relative}'"
            ))
        })?;
        let end = start.checked_add(read).ok_or_else(|| {
            MgError::Integrity(format!(
                "Python runtime materialization is too large for '{relative}'"
            ))
        })?;
        let Some(expected_chunk) = archive_member.get(start..end) else {
            return Err(member_mismatch(relative));
        };
        if expected_chunk != &buffer[..read] {
            return Err(member_mismatch(relative));
        }
        hasher.update(&buffer[..read]);
        total = total
            .checked_add(read as u64)
            .ok_or_else(|| member_mismatch(relative))?;
    }

    if total != archive_member.len() as u64 {
        return Err(member_mismatch(relative));
    }
    Ok((total, hasher.finalize().into()))
}

fn member_mismatch(relative: &str) -> MgError {
    MgError::Integrity(format!(
        "Python runtime materialization differs from authenticated wheel member '{relative}'"
    ))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
