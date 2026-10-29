//! Immutability enforcement and content verification for keg directories.
//! Ép và xác minh tính bất biến, nội dung cho thư mục keg.
//!
//! A keg is sealed by removing every write bit (files and directories), then
//! fingerprinted with a BLAKE3 tree digest. `verify_keg` re-walks the tree and
//! compares mode bits plus digest, so a write-protection bypass (chmod, root)
//! and a content change are both detected. Teardown thaws the tree from inside
//! so it never depends on who is allowed to write the parent directory.
//! Keg được seal bằng cách bỏ mọi bit write (file và thư mục), rồi lấy dấu vân
//! tay BLAKE3 toàn cây. `verify_keg` đi lại cây, so bit mode + digest, nên
//! phát hiện cả bypass chmod/root lẫn thay đổi nội dung.

use std::path::{Path, PathBuf};

use crate::install::keg::error::{KegError, KegResult};
use crate::install::keg::keg::KEG_RECORD_FILE;

/// Streaming buffer for keg file hashing (64 KiB).
/// Buffer đọc tuần tự khi hash file keg (64 KiB).
/// One fingerprinted node of a keg tree.
/// Một node được lấy dấu vân tay trong cây keg.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Directory {
        path: PathBuf,
        mode: u32,
    },
    File {
        path: PathBuf,
        mode: u32,
        len: u64,
        blake3: String,
    },
    Link {
        path: PathBuf,
        target: PathBuf,
    },
}

impl Node {
    fn path(&self) -> &Path {
        match self {
            Node::Directory { path, .. } | Node::File { path, .. } | Node::Link { path, .. } => {
                path
            }
        }
    }

    fn mode(&self) -> Option<u32> {
        match self {
            Node::Directory { mode, .. } | Node::File { mode, .. } => Some(*mode),
            Node::Link { .. } => None,
        }
    }
}

/// BLAKE3 tree digest of a sealed keg.
/// Digest BLAKE3 toàn cây của keg đã seal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KegFingerprint {
    /// Digest over every relative path, mode bit and file hash.
    /// Digest trên mọi đường dẫn tương đối, bit mode và hash file.
    pub tree_digest: String,
    /// Number of regular files hashed.
    /// Số file thường đã hash.
    pub file_count: u64,
}

/// Write-protect a keg tree and return its fingerprint.
/// Ép write-protect cây keg và trả dấu vân tay.
pub fn seal_keg(keg_dir: &Path) -> KegResult<KegFingerprint> {
    let root = canonical_keg_root(keg_dir)?;
    for node in collect_nodes(&root)? {
        if let Some(mode) = node.mode() {
            let read_only = mode & !0o222;
            if read_only != mode {
                set_mode(node.path(), read_only)?;
            }
        }
    }
    // Re-collect so the fingerprint describes the sealed state.
    // Thu lại để digest mô tả trạng thái đã seal.
    let sealed = collect_nodes(&root)?;
    Ok(fingerprint(&root, &sealed))
}

/// Verify a keg is still write-protected and byte-identical.
/// Xác minh keg vẫn write-protect và không đổi nội dung.
pub fn verify_keg(keg_dir: &Path, expected: &KegFingerprint) -> KegResult<()> {
    let root = canonical_keg_root(keg_dir)?;
    let nodes = collect_nodes(&root)?;
    for node in &nodes {
        if node.mode().is_some_and(|mode| mode & 0o222 != 0) {
            return Err(KegError::Tampered {
                path: root.display().to_string(),
                detail: format!("write bit present on {}", relative(&root, node.path())),
            });
        }
    }
    let actual = fingerprint(&root, &nodes);
    if actual.tree_digest != expected.tree_digest || actual.file_count != expected.file_count {
        return Err(KegError::Tampered {
            path: root.display().to_string(),
            detail: format!(
                "tree digest {} does not match sealed {}",
                actual.tree_digest, expected.tree_digest
            ),
        });
    }
    Ok(())
}

/// Restore owner write permission inside a keg; returns entries restored.
/// Khôi phục quyền ghi owner bên trong keg; trả số entry đã khôi phục.
///
/// Restoring from inside means a directory-mode change is never required.
/// Khôi phục từ bên trong nên không bao giờ phải đổi mode thư mục cha.
pub fn thaw_keg(keg_dir: &Path) -> KegResult<usize> {
    let root = canonical_keg_root(keg_dir)?;
    let mut restored = 0usize;
    for node in collect_nodes(&root)? {
        if let Some(mode) = node.mode()
            && mode & 0o200 == 0
        {
            set_mode(node.path(), mode | 0o200)?;
            restored += 1;
        }
    }
    Ok(restored)
}

/// Remove a keg tree: thaw in place, then delete.
/// Xoá cây keg: rã băng tại chỗ rồi xoá.
pub fn remove_keg(keg_dir: &Path) -> KegResult<()> {
    if !keg_dir.exists() {
        return Ok(());
    }
    thaw_keg(keg_dir)?;
    std::fs::remove_dir_all(keg_dir)
        .map_err(|error| KegError::io("remove keg directory", keg_dir, error))
}

/// Recursively enumerate a keg tree (root included).
/// Liệt kê đệ quy cây keg (gồm root), thứ tự tất định.
fn collect_nodes(root: &Path) -> KegResult<Vec<Node>> {
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    let mut nodes = Vec::new();
    while let Some(current) = stack.pop() {
        let metadata = std::fs::symlink_metadata(&current)
            .map_err(|error| KegError::io("inspect keg entry", &current, error))?;
        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            let target = std::fs::read_link(&current)
                .map_err(|error| KegError::io("read keg symlink", &current, error))?;
            if !is_safe_link(root, &current, &target) {
                return Err(KegError::UnsafePath {
                    path: current.display().to_string(),
                    reason: "keg symlink escapes its keg or is absolute",
                });
            }
            nodes.push(Node::Link {
                path: current,
                target,
            });
        } else if file_type.is_dir() {
            nodes.push(Node::Directory {
                path: current.clone(),
                mode: mode_of(&metadata),
            });
            let children = std::fs::read_dir(&current)
                .map_err(|error| KegError::io("read keg directory", &current, error))?;
            for child in children {
                let child =
                    child.map_err(|error| KegError::io("read keg directory", &current, error))?;
                stack.push(child.path());
            }
        } else if file_type.is_file() {
            // The keg record is install metadata written after sealing; it is
            // excluded from the fingerprint so sealing stays idempotent. Editing
            // it still changes `open_keg`'s identity check.
            // (Record keg là metadata do bước cài ghi sau seal; loại khỏi dấu vân
            // tay để seal idempotent. Sửa record vẫn bị kiểm identity của open_keg.)
            if current.file_name().map(|name| name == KEG_RECORD_FILE) == Some(true) {
                continue;
            }
            nodes.push(Node::File {
                path: current.clone(),
                mode: mode_of(&metadata),
                len: metadata.len(),
                blake3: file_digest(&current)?,
            });
        } else {
            return Err(KegError::Tampered {
                path: current.display().to_string(),
                detail: "special file is not allowed inside a keg".to_string(),
            });
        }
    }
    nodes.sort_by_key(|node| relative(root, node.path()));
    Ok(nodes)
}

fn file_digest(path: &Path) -> KegResult<String> {
    // Open with no-follow and stream: a keg file is never a link and is never
    // loaded whole into memory (kegs can be large shared libraries).
    // (Mở no-follow và đọc streaming: file keg không phải link, không nạp toàn bộ.)
    let file = mgc_platform::open_regular_file_no_follow(path).map_err(|error| {
        KegError::io(
            "open keg file",
            path,
            std::io::Error::other(error.to_string()),
        )
    })?;
    // `mgc-crypto` owns the BLAKE3 dependency so every digest in the product
    // comes from one audited implementation; the sink only writes bytes.
    // (`mgc-crypto` sở hữu dependency BLAKE3 để mọi digest trong sản phẩm đến
    // từ một implementation đã audit; sink chỉ ghi byte.)
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_reader(file).map_err(|error| {
        KegError::io(
            "read keg file",
            path,
            std::io::Error::other(error.to_string()),
        )
    })?;
    Ok(digest)
}

fn fingerprint(root: &Path, nodes: &[Node]) -> KegFingerprint {
    let mut hasher = mgc_crypto::blake3_signer::Blake3StreamingHasher::new();
    let mut file_count = 0u64;
    for node in nodes {
        let relative_path = relative(root, node.path());
        // The keg root's own mode is install-controllable (the record is placed
        // there), so the digest starts below it; `verify_keg` still enforces the
        // root's write protection separately.
        // (Mode của root keg do bước cài điều khiển — nơi đặt record — nên digest
        // bắt đầu từ bên trong; `verify_keg` vẫn kiểm write-protect root riêng.)
        if relative_path.is_empty() {
            continue;
        }
        hasher.update(relative_path.as_bytes());
        hasher.update(b"\0");
        match node {
            Node::Directory { mode, .. } => {
                hasher.update(b"d");
                hasher.update(&mode.to_le_bytes());
            }
            Node::File {
                mode, len, blake3, ..
            } => {
                hasher.update(b"f");
                hasher.update(&mode.to_le_bytes());
                hasher.update(&len.to_le_bytes());
                hasher.update(blake3.as_bytes());
                file_count += 1;
            }
            Node::Link { target, .. } => {
                hasher.update(b"l");
                hasher.update(target.to_string_lossy().as_bytes());
            }
        }
        hasher.update(b"\n");
    }
    KegFingerprint {
        tree_digest: hasher.hex(),
        file_count,
    }
}

/// Reject a symlink that would let a keg read or write outside itself.
/// Từ chối symlink cho phép keg đọc/ghi ra ngoài chính nó.
fn is_safe_link(root: &Path, link: &Path, target: &Path) -> bool {
    if target.is_absolute() {
        return false;
    }
    let parent = link.parent().unwrap_or(root);
    let mut resolved = parent.to_path_buf();
    for component in target.components() {
        match component {
            std::path::Component::Normal(part) => resolved.push(part),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !resolved.pop() || !resolved.starts_with(root) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    resolved.starts_with(root)
}

fn canonical_keg_root(keg_dir: &Path) -> KegResult<PathBuf> {
    let metadata = std::fs::symlink_metadata(keg_dir)
        .map_err(|error| KegError::io("inspect keg directory", keg_dir, error))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(KegError::UnsafePath {
            path: keg_dir.display().to_string(),
            reason: "keg root must be a real directory, not a link",
        });
    }
    keg_dir
        .canonicalize()
        .map_err(|error| KegError::io("canonicalize keg directory", keg_dir, error))
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string_lossy().into_owned())
}

fn mode_of(metadata: &std::fs::Metadata) -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o777
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        // Windows has no POSIX bits; protection is read-only attribute based.
        // Windows không có bit POSIX; bảo vệ dựa trên thuộc tính read-only.
        0o755
    }
}

fn set_mode(path: &Path, mode: u32) -> KegResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
            .map_err(|error| KegError::io("set keg permissions", path, error))
    }
    #[cfg(not(unix))]
    {
        let metadata = std::fs::symlink_metadata(path)
            .map_err(|error| KegError::io("inspect keg permissions", path, error))?;
        if !metadata.file_type().is_symlink() {
            let mut permissions = metadata.permissions();
            let readonly = mode & 0o222 == 0;
            if permissions.readonly() != readonly {
                permissions.set_readonly(readonly);
                std::fs::set_permissions(path, permissions)
                    .map_err(|error| KegError::io("set keg permissions", path, error))?;
            }
        }
        Ok(())
    }
}
