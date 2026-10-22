//! Streams SHA-256 verification and quarantines corrupted blob files for forensics.
//! Đọc tuần tự để xác minh SHA-256 và cách ly file blob hỏng phục vụ điều tra.

use super::super::prelude::*;

pub(in crate::storage) async fn hash_file_sha256(path: &Path) -> Result<String> {
    use sha2::Digest;

    let mut file = fs::File::open(path).await?;
    let mut hasher = sha2::Sha256::new();
    let mut buffer = vec![0; 128 * 1024];
    loop {
        let count = file.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hex::encode(hasher.finalize()))
}

/// Quarantine a corrupt registry blob (P0-6): move the tampered file into
/// `<blobs_dir>/../quarantine/` next to a forensic report. Best-effort — a
/// quarantine failure must not mask the integrity error being reported.
///
/// Cách ly blob registry hỏng (P0-6): chuyển file bị sửa vào
/// `<blobs_dir>/../quarantine/` kèm báo cáo điều tra. Best-effort — lỗi
/// cách ly không được che lỗi integrity đang báo.
pub(in crate::storage) fn quarantine_corrupt_blob(
    path: &str,
    expected: &str,
    actual: &str,
) -> Result<()> {
    let src = std::path::Path::new(path);
    // Walk up from the blob file to the STORE root (parent of blobs/):
    // <store>/blobs/oci/<repo>/<p1>/<blob> → up 4 levels = <store>.
    // The OCI blob layout is blobs/oci/<repo>/<p1>/<p2>; the local npm
    // layout is blobs/<p1>/<p2> — resolve robustly by walking until the
    // directory named "blobs" is found, then use its parent.
    //
    // Đi lên từ file blob tới GỐC STORE (cha của blobs/): layout OCI là
    // blobs/oci/<repo>/<p1>/<p2>, layout npm là blobs/<p1>/<p2> — đi lên
    // tới khi gặp thư mục tên "blobs" rồi lấy cha của nó (ổn cho cả hai
    // layout).
    let mut cur = src.parent();
    while let Some(dir) = cur {
        if dir.file_name().map(|n| n == "blobs").unwrap_or(false) {
            let store_root = dir.parent().unwrap_or(dir);
            let qdir = store_root.join("quarantine");
            let _ = std::fs::create_dir_all(&qdir);
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            let prefix: String = expected.chars().take(12).collect();
            let qfile = qdir.join(format!("corrupt-oci-{prefix}-{stamp}.blob"));
            let _ = std::fs::rename(src, &qfile);
            let _ = std::fs::write(
                qdir.join(format!("corrupt-oci-{prefix}-{stamp}.report")),
                format!(
                    "expected digest: {expected}\nactual digest:   {actual}\nsource path: {path}\nquarantined at: {stamp}\n"
                ),
            );
            return Ok(());
        }
        cur = dir.parent();
    }
    Ok(())
}
