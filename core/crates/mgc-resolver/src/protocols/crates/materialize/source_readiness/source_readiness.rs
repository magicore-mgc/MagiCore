//! Marker is the readiness signal for a materialized Cargo registry directory source.
//! Marker là tín hiệu sẵn sàng cho directory source registry Cargo đã materialize.
//!
//! Cargo only treats an extracted `registry/src/<dir>` as a usable package when
//! the directory carries Cargo's own readiness marker. A source tree without the
//! marker is re-extracted from the `.crate` archive; the earlier
//! `.cargo-checksum.json` record and the marker could never both survive that
//! re-extraction, so a partial materialization used to fail every later build
//! with "failed to load checksum `.cargo-checksum.json` ... No such file or
//! directory".
//! Cargo chỉ coi `registry/src/<dir>` đã giải nén là package dùng được khi thư
//! mục có marker sẵn sàng của Cargo. Source thiếu marker sẽ được giải nén lại từ
//! archive `.crate`; record `.cargo-checksum.json` và marker không thể cùng tồn
//! tại qua lần giải nén lại đó, nên materialize dở dang từng làm mọi lần build
//! sau lỗi "failed to load checksum ... No such file or directory".
//!
//! The marker shape matches what Cargo itself writes after a verified
//! extraction. It carries no hash of its own, so it must only be written after
//! the archive has been fully verified and the source content checked.
//! Hình dạng marker khớp thứ Cargo tự ghi sau khi giải nén đã xác minh. Marker
//! không chứa hash riêng nên chỉ được ghi sau khi archive đã xác minh đầy đủ và
//! nội dung source đã được kiểm.

use std::path::Path;

/// Cargo's marker file is not part of the crate payload, so readers that hash
/// the source tree must exclude it exactly like `.cargo-checksum.json`.
/// File marker của Cargo không thuộc payload crate, nên bên đọc hash cây source
/// phải loại trừ nó giống hệt `.cargo-checksum.json`.
pub const NON_ARCHIVE_SOURCE_FILES: &[&str] = &[CARGO_SOURCE_MARKER];

/// Cargo's readiness marker file name inside an extracted registry source.
/// Tên file marker sẵn sàng của Cargo trong source registry đã giải nén.
pub const CARGO_SOURCE_MARKER: &str = ".cargo-ok";

/// Marker payload Cargo writes for the current registry source format version.
/// Nội dung marker Cargo ghi cho version định dạng source registry hiện hành.
pub const CARGO_SOURCE_MARKER_CONTENT: &str = "{\"v\":1}";

/// Registry `src` root holding every materialized `{name}-{version}` directory.
/// Root `src` registry chứa mọi thư mục `{name}-{version}` đã materialize.
pub fn registry_source_root(cargo_home: &Path) -> std::path::PathBuf {
    cargo_home.join("registry").join("src")
}

/// Directory name Cargo uses for one extracted package.
/// Tên thư mục Cargo dùng cho một package đã giải nén.
pub fn registry_package_dir(name: &str, version: &str) -> String {
    format!("{name}-{version}")
}

/// Report whether a registry source directory carries Cargo's readiness marker.
///
/// A missing marker, a non-regular marker (link, directory, device) or
/// unreadable content all report `false`: the caller then rebuilds the source
/// instead of letting Cargo discover the gap on its own.
/// Cho biết thư mục source registry có marker sẵn sàng của Cargo hay không.
/// Thiếu marker, marker không phải file thường (link, thư mục, device) hoặc nội
/// dung không đọc được đều trả `false`: caller dựng lại source thay vì để Cargo
/// tự phát hiện lỗ hổng.
pub fn has_cargo_source_marker(source_root: &Path) -> bool {
    let marker = source_root.join(CARGO_SOURCE_MARKER);
    match std::fs::symlink_metadata(&marker) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(_) => return false,
        Err(_) => return false,
    }
    match std::fs::read_to_string(&marker) {
        Ok(content) => content.trim() == CARGO_SOURCE_MARKER_CONTENT,
        Err(_) => false,
    }
}

/// Write Cargo's readiness marker so the source directory counts as complete.
///
/// The write is atomic (temporary file then replace) and refuses a non-regular
/// destination, so an attacker cannot substitute a link that would make a
/// later write land outside the source directory.
/// Ghi marker sẵn sàng của Cargo để thư mục source được coi là hoàn tất. Ghi
/// nguyên tử (file tạm rồi thay thế) và từ chối đích không phải file thường, nên
/// kẻ tấn công không thể thay bằng link khiến lần ghi sau rơi ra ngoài source.
pub fn write_cargo_source_marker(source_root: &Path) -> std::io::Result<()> {
    use std::io::Write as _;

    let marker = source_root.join(CARGO_SOURCE_MARKER);
    if let Ok(metadata) = std::fs::symlink_metadata(&marker)
        && !metadata.file_type().is_file()
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "refusing non-regular Cargo source marker '{}'",
                marker.display()
            ),
        ));
    }
    // A per-process counter keeps two threads materializing in one store from
    // staging into the same temporary name.
    // (Counter theo process để hai thread materialize trong cùng store không ghi
    // đè cùng một tên file tạm.)
    static NEXT_TEMP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let sequence = NEXT_TEMP.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let temp = source_root.join(format!(".cargo-ok.{}.{}.tmp", std::process::id(), sequence));
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        // O_NOFOLLOW: a planted link must never redirect the marker write.
        // (O_NOFOLLOW: link bị cắm sẵn không được chuyển hướng lần ghi marker.)
        options.mode(0o644).custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options.open(&temp)?;
    file.write_all(CARGO_SOURCE_MARKER_CONTENT.as_bytes())?;
    file.sync_all()?;
    std::fs::rename(&temp, &marker).inspect_err(|_| {
        let _ = std::fs::remove_file(&temp);
    })
}
