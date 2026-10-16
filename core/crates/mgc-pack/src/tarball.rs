//! Deterministic streaming tarball creation and hashing.
//! Đóng gói bằng bộ nhớ hữu hạn; chỉ publish output sau khi archive/hash hoàn tất.
use crate::ignore::select_files;
use anyhow::{Result, bail};
use base64::Engine;
use sha1::Sha1;
use sha2::{Digest, Sha512};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

const HASH_BUFFER_BYTES: usize = 64 * 1024;

pub struct PackResult {
    pub tarball_path: std::path::PathBuf,
    pub shasum: String,
    pub integrity: String,
    pub size: u64,
    pub unpacked_size: u64,
    pub entry_count: usize,
}

/// Đóng gói root → tarball gzip tại output_path.
/// Deterministic: mtime = 0 (epoch), entries sorted, gzip mtime = 0 (flate2 Header mtime=0 mặc định).
pub fn pack(root: &Path, output_path: &Path, prefix: &str) -> Result<PackResult> {
    let root = root.canonicalize()?;
    let parent = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let output = parent.canonicalize()?.join(
        output_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Archive output must name a file"))?,
    );
    let mut files = select_files(&root)?;
    files.retain(|relative| root.join(relative) != output);
    let mut unpacked_size: u64 = 0;
    let mut staged = tempfile::NamedTempFile::new_in(parent)?;
    let mut gz =
        flate2::write::GzEncoder::new(staged.as_file_mut(), flate2::Compression::default());
    {
        let mut builder = tar::Builder::new(&mut gz);
        builder.follow_symlinks(false);
        for rel in &files {
            let src = root.join(rel);
            let entry_name = if prefix.is_empty() {
                rel.to_string_lossy().to_string()
            } else {
                format!("{}/{}", prefix, rel.to_string_lossy())
            };
            let metadata = fs::symlink_metadata(&src)?;
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                bail!("Package input must be a regular file: {}", rel.display());
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::MetadataExt;
                const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
                if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                    bail!(
                        "Package input must not be a reparse point: {}",
                        rel.display()
                    );
                }
            }
            let mut input = File::open(&src)?;
            let size = input.metadata()?.len();
            unpacked_size = unpacked_size
                .checked_add(size)
                .ok_or_else(|| anyhow::anyhow!("Package unpacked size overflow"))?;
            let mut header = tar::Header::new_gnu();
            header.set_size(size);
            #[cfg(unix)]
            let mode = {
                use std::os::unix::fs::PermissionsExt;
                if metadata.permissions().mode() & 0o111 != 0 {
                    0o755
                } else {
                    0o644
                }
            };
            #[cfg(not(unix))]
            let mode = 0o644;
            header.set_mode(mode);
            header.set_mtime(0); // deterministic — mtime epoch
            header.set_cksum();
            {
                let mut bounded = (&mut input).take(size);
                builder.append_data(&mut header, &entry_name, &mut bounded)?;
                if bounded.limit() != 0 {
                    bail!("Package input shrank while packing: {}", rel.display());
                }
            }
            if input.read(&mut [0u8; 1])? != 0 {
                bail!("Package input grew while packing: {}", rel.display());
            }
        }
        builder.finish()?;
    }
    gz.finish()?;

    // Hash the completed staged archive with fixed-size reads, never a second full copy.
    // Băm archive đã hoàn tất qua buffer cố định; lỗi không thay output cũ.
    let mut input = File::open(staged.path())?;
    let mut sha1 = Sha1::new();
    let mut sha512 = Sha512::new();
    let mut buffer = [0u8; HASH_BUFFER_BYTES];
    let mut size = 0u64;
    loop {
        let count = input.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha1.update(&buffer[..count]);
        sha512.update(&buffer[..count]);
        size = size
            .checked_add(count as u64)
            .ok_or_else(|| anyhow::anyhow!("Archive size overflow"))?;
    }
    drop(input);
    let shasum = hex::encode(sha1.finalize());
    let integrity = format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(sha512.finalize())
    );
    staged.as_file().sync_all()?;
    staged.persist(output_path)?;

    Ok(PackResult {
        tarball_path: output_path.to_path_buf(),
        shasum,
        integrity,
        size,
        unpacked_size,
        entry_count: files.len(),
    })
}
