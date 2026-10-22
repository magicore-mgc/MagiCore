//! Copies mismatched CAS evidence from the exact handle that was verified.
//! Sao chép bằng chứng CAS sai từ đúng handle đã được xác minh.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use tempfile::NamedTempFile;

use super::QuarantineError;

pub(in crate::cas::store::verify) fn quarantine_blob(
    root: &Path,
    source_path: &Path,
    mut source: File,
    expected: &str,
    actual: &str,
) -> Result<(), QuarantineError> {
    let directory = root.join("quarantine");
    std::fs::create_dir_all(&directory)
        .map_err(|source| io_error("create quarantine directory", &directory, source))?;

    source
        .seek(SeekFrom::Start(0))
        .map_err(|source| io_error("rewind verified source", source_path, source))?;
    let mut evidence = NamedTempFile::new_in(&directory)
        .map_err(|source| io_error("create evidence staging file", &directory, source))?;
    let mut digest = blake3::Hasher::new();
    let mut buffer = [0u8; 65_536];
    loop {
        let count = source
            .read(&mut buffer)
            .map_err(|source| io_error("read verified source", source_path, source))?;
        if count == 0 {
            break;
        }
        evidence
            .write_all(&buffer[..count])
            .map_err(|source| io_error("write evidence staging file", evidence.path(), source))?;
        digest.update(&buffer[..count]);
    }

    let captured = digest.finalize().to_hex().to_string();
    if captured != actual {
        return Err(QuarantineError::DigestMismatch {
            expected: actual.to_string(),
            actual: captured,
        });
    }
    evidence
        .as_file()
        .sync_all()
        .map_err(|source| io_error("sync evidence staging file", evidence.path(), source))?;

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let id = uuid::Uuid::new_v4().simple().to_string();
    let blob_path = directory.join(format!(
        "corrupt-{}-{}-{stamp}-{id}.blob",
        short_digest(expected),
        short_digest(actual)
    ));
    evidence
        .persist_noclobber(&blob_path)
        .map_err(|error| io_error("publish evidence blob", &blob_path, error.error))?;

    let report_path = directory.join(format!(
        "corrupt-{}-{stamp}-{id}.report",
        short_digest(expected)
    ));
    write_report(&report_path, expected, actual, source_path, stamp)?;
    Ok(())
}

fn write_report(
    report_path: &Path,
    expected: &str,
    actual: &str,
    source_path: &Path,
    stamp: u128,
) -> Result<(), QuarantineError> {
    let mut report = NamedTempFile::new_in(report_path.parent().unwrap_or(Path::new(".")))
        .map_err(|source| io_error("create report staging file", report_path, source))?;
    let contents = format!(
        "expected blake3: {expected}\n\
         actual blake3:   {actual}\n\
         source path: {}\n\
         evidence captured from verified handle: true\n\
         source retained: true\n\
         quarantined at: {stamp}\n",
        source_path.display()
    );
    report
        .write_all(contents.as_bytes())
        .map_err(|source| io_error("write quarantine report", report_path, source))?;
    report
        .as_file()
        .sync_all()
        .map_err(|source| io_error("sync quarantine report", report_path, source))?;
    report
        .persist_noclobber(report_path)
        .map_err(|error| io_error("publish quarantine report", report_path, error.error))?;
    Ok(())
}

fn short_digest(digest: &str) -> &str {
    &digest[..12.min(digest.len())]
}

fn io_error(operation: &'static str, path: &Path, source: std::io::Error) -> QuarantineError {
    QuarantineError::Io {
        operation,
        path: PathBuf::from(path),
        source,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
