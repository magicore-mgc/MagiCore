//! Compute compressed size and package digests with bounded reads.
//! Tính kích thước nén và digest package với lượt đọc có giới hạn.

use crate::tarball::PackError;
use anyhow::Result;
use base64::Engine;
use sha1::Sha1;
use sha2::{Digest, Sha512};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const HASH_BUFFER_BYTES: usize = 64 * 1024;

pub(in crate::tarball) struct ArchiveDigest {
    pub shasum: String,
    pub integrity: String,
    pub size: u64,
}

pub(in crate::tarball) fn digest_archive(input: &mut File) -> Result<ArchiveDigest> {
    input.seek(SeekFrom::Start(0))?;
    let mut sha1 = Sha1::new();
    let mut sha512 = Sha512::new();
    let mut buffer = [0_u8; HASH_BUFFER_BYTES];
    let mut size = 0_u64;
    loop {
        let count = input.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha1.update(&buffer[..count]);
        sha512.update(&buffer[..count]);
        size = size
            .checked_add(count as u64)
            .ok_or(PackError::ArchiveSizeOverflow)?;
    }

    Ok(ArchiveDigest {
        shasum: hex::encode(sha1.finalize()),
        integrity: format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(sha512.finalize())
        ),
        size,
    })
}
