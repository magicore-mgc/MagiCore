//! Immutable keg store: create, verify, list and inspect library kegs.
//! Store keg bất biến: tạo, xác minh, liệt kê và đọc keg library.
//!
//! A keg is materialized in a staging directory next to its final path, sealed
//! with write protection and a fingerprint, then renamed into place. Publishing never
//! overwrites an existing keg, so the previous version of a library is never
//! destroyed by installing a new one (rollback stays possible).
//! Keg được materialize trong thư mục staging cạnh đích, seal (write-protect
//! cộng dấu vân tay), rồi rename vào chỗ. Publish không bao giờ ghi đè keg đã
//! có, nên cài version mới không phá version cũ (vẫn rollback được).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::install::keg::error::{KegError, KegResult};
use crate::install::keg::immutability::{KegFingerprint, remove_keg, seal_keg, verify_keg};
use crate::install::keg::layout::{KegLayout, validate_library_name, validate_version};

/// Record stored inside the keg but excluded from the sealed fingerprint.
/// Bản ghi lưu trong keg nhưng không nằm trong dấu vân tay đã seal.
pub const KEG_RECORD_FILE: &str = ".mgc-keg.json";

/// Sequencing for staging directories (process-local, monotonic).
/// Bộ đếm thư mục staging (mỗi tiến trình, tăng đơn điệu).
static NEXT_STAGING: AtomicU64 = AtomicU64::new(0);

/// Metadata of one installed keg.
/// Metadata của một keg đã cài.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KegRecord {
    /// Library name.
    /// Tên library.
    pub name: String,
    /// Exact semantic version installed.
    /// Version semver chính xác đã cài.
    pub version: String,
    /// Tree digest of the sealed contents (record file excluded).
    /// Digest cây của nội dung đã seal (không gồm file record).
    pub tree_digest: String,
    /// Number of sealed files.
    /// Số file đã seal.
    pub file_count: u64,
    /// Install time in RFC 3339 (UTC).
    /// Thời điểm cài dạng RFC 3339 (UTC).
    pub installed_at: String,
}

/// Immutable keg store rooted at a resolved layout.
/// Store keg bất biến đặt trên layout đã resolve.
#[derive(Debug, Clone)]
pub struct KegStore {
    layout: KegLayout,
}

impl KegStore {
    /// Build a store over an already resolved layout.
    /// Tạo store trên layout đã resolve.
    pub fn new(layout: KegLayout) -> Self {
        Self { layout }
    }

    /// Resolve the layout from MagiCore data paths.
    /// Resolve layout từ đường dẫn dữ liệu MagiCore.
    pub fn resolve() -> KegResult<Self> {
        Ok(Self::new(KegLayout::resolve()?))
    }

    /// Whether the current process opted into the keg transfer layer.
    /// Process hiện tại có opt-in vào lớp keg không.
    ///
    /// P-6.1..P-6.3: migrating whole dependency trees into kegs changes the
    /// on-disk layout a project's toolchain reads, so the layer is additive and
    /// opt-in: without `MGC_LIB_KEG=1` no adapter path touches it and an
    /// existing project keeps the layout it already has.
    /// (P-6.1..P-6.3: chuyển cả cây dependency vào keg đổi layout trên đĩa mà
    /// toolchain của project đọc, nên lớp này là additive và opt-in: không có
    /// `MGC_LIB_KEG=1` thì không đường adapter nào chạm vào nó và project hiện
    /// có giữ nguyên layout đang dùng.)
    pub fn is_enabled_by_environment() -> bool {
        std::env::var("MGC_LIB_KEG").is_ok_and(|value| value == "1" || value == "true")
    }

    /// Borrow the underlying layout.
    /// Mượn layout bên dưới.
    pub fn layout(&self) -> &KegLayout {
        &self.layout
    }

    /// Install a keg from already-materialized content.
    /// Cài keg từ nội dung đã materialize sẵn.
    ///
    /// `materialize` receives the staging directory and writes the keg tree.
    /// Existing kegs are never replaced: the caller gets `KegExists`.
    /// `materialize` nhận thư mục staging để ghi cây keg. Keg đã có không bị
    /// ghi đè: caller nhận `KegExists`.
    pub fn create_keg<F>(&self, name: &str, version: &str, materialize: F) -> KegResult<KegRecord>
    where
        F: FnOnce(&Path) -> KegResult<()>,
    {
        validate_library_name(name)?;
        let version = validate_version(version)?;
        let keg_dir = self.layout.keg_dir(name, &version);
        if keg_dir.exists() {
            return Err(KegError::KegExists {
                name: name.to_string(),
                version: version.to_string(),
                path: keg_dir.display().to_string(),
            });
        }
        let library_root = self.layout.cellar_root().join(name);
        std::fs::create_dir_all(&library_root)
            .map_err(|error| KegError::io("create keg library directory", &library_root, error))?;

        let staging = library_root.join(staging_name(&version));
        remove_stale_staging(&staging)?;
        std::fs::create_dir(&staging)
            .map_err(|error| KegError::io("create keg staging directory", &staging, error))?;

        let staged = (|| -> KegResult<KegRecord> {
            materialize(&staging)?;
            // Seal first, then write the record: sealing makes the tree
            // read-only, so the record must be added while it is writable and
            // is deliberately excluded from the fingerprint.
            // (Seal trước rồi ghi record: seal làm cây chỉ đọc nên record phải
            // ghi lúc còn ghi được; record không nằm trong dấu vân tay.)
            let fingerprint = seal_keg(&staging)?;
            verify_keg(&staging, &fingerprint)?;
            let record = KegRecord {
                name: name.to_string(),
                version: version.to_string(),
                tree_digest: fingerprint.tree_digest,
                file_count: fingerprint.file_count,
                installed_at: chrono::Utc::now().to_rfc3339(),
            };
            // Restore owner write only on the staging root to place the record;
            // sealing is applied again below.
            // (Chỉ mở quyền ghi cho root staging để đặt record; seal lại ở dưới.)
            let _ = restore_root_write(&staging);
            write_record(&staging, &record)?;
            let refingerprint = seal_keg(&staging)?;
            if refingerprint.tree_digest != record.tree_digest
                || refingerprint.file_count != record.file_count
            {
                return Err(KegError::Tampered {
                    path: staging.display().to_string(),
                    detail: "keg digest changed while writing its record".to_string(),
                });
            }
            // Publishing must not clobber a keg created by a concurrent install.
            // (Publish không được đè keg do install đồng thời tạo.)
            if keg_dir.exists() {
                return Err(KegError::KegExists {
                    name: name.to_string(),
                    version: version.to_string(),
                    path: keg_dir.display().to_string(),
                });
            }
            std::fs::rename(&staging, &keg_dir)
                .map_err(|error| KegError::io("publish keg", &keg_dir, error))?;
            sync_dir(&library_root)?;
            Ok(record)
        })();

        if staged.is_err() {
            let _ = remove_keg(&staging);
        }
        staged
    }

    /// Verify that an installed keg is intact and return its record.
    /// Xác minh keg đã cài còn nguyên vẹn và trả record.
    pub fn open_keg(&self, name: &str, version: &str) -> KegResult<KegRecord> {
        validate_library_name(name)?;
        let version = validate_version(version)?;
        let keg_dir = self.layout.keg_dir(name, &version);
        if !keg_dir.is_dir() {
            return Err(KegError::KegNotFound {
                name: name.to_string(),
                version: version.to_string(),
                path: keg_dir.display().to_string(),
            });
        }
        let record = read_record(&keg_dir)?;
        if record.name != name || record.version != version.to_string() {
            return Err(KegError::Tampered {
                path: keg_dir.display().to_string(),
                detail: format!(
                    "record identity {}/{} does not match directory",
                    record.name, record.version
                ),
            });
        }
        verify_keg(
            &keg_dir,
            &KegFingerprint {
                tree_digest: record.tree_digest.clone(),
                file_count: record.file_count,
            },
        )?;
        Ok(record)
    }

    /// List every installed keg of one library, newest version last.
    /// Liệt kê mọi keg của một library, version mới nhất ở cuối.
    pub fn list_kegs(&self, name: &str) -> KegResult<Vec<KegRecord>> {
        validate_library_name(name)?;
        let library_root = self.layout.cellar_root().join(name);
        let entries = match std::fs::read_dir(&library_root) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => {
                return Err(KegError::io(
                    "read keg library directory",
                    &library_root,
                    error,
                ));
            }
        };
        let mut versions = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|error| {
                KegError::io("read keg library directory", &library_root, error)
            })?;
            if !entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false) {
                continue;
            }
            let candidate = entry.file_name().to_string_lossy().into_owned();
            // Staging directories are not installed kegs.
            // Thư mục staging không phải keg đã cài.
            if candidate.starts_with(STAGING_PREFIX) {
                continue;
            }
            if let Ok(version) = Version::parse(&candidate) {
                versions.push((version, candidate));
            }
        }
        versions.sort_by(|left, right| left.0.cmp(&right.0));
        let mut records = Vec::with_capacity(versions.len());
        for (_, raw) in versions {
            records.push(read_record(&self.layout.keg_dir(
                name,
                &Version::parse(&raw).map_err(|_| KegError::InvalidVersion(raw.clone()))?,
            ))?);
        }
        Ok(records)
    }

    /// Remove an installed keg explicitly (the only path that deletes one).
    /// Xoá một keg đã cài một cách tường minh (đường duy nhất được phép xoá).
    pub fn remove_keg(&self, name: &str, version: &str) -> KegResult<()> {
        validate_library_name(name)?;
        let version = validate_version(version)?;
        remove_keg(&self.layout.keg_dir(name, &version))
    }

    /// Final directory of a keg (no creation, no verification).
    /// Thư mục cuối của keg (không tạo, không xác minh).
    pub fn keg_dir(&self, name: &str, version: &str) -> KegResult<PathBuf> {
        validate_library_name(name)?;
        let version = validate_version(version)?;
        Ok(self.layout.keg_dir(name, &version))
    }
}

/// Write a keg record next to sealed content (used by tests and re-sealing).
/// Ghi record keg cạnh nội dung đã seal (dùng cho test và seal lại).
pub fn write_record(keg_dir: &Path, record: &KegRecord) -> KegResult<()> {
    let path = keg_dir.join(KEG_RECORD_FILE);
    let encoded = serde_json::to_vec_pretty(record)
        .map_err(|error| KegError::Layout(format!("encode keg record: {error}")))?;
    std::fs::write(&path, encoded).map_err(|error| KegError::io("write keg record", &path, error))
}

/// Read a keg record from an unsealed/readable keg directory.
/// Đọc record keg từ thư mục keg đọc được.
pub fn read_record(keg_dir: &Path) -> KegResult<KegRecord> {
    let path = keg_dir.join(KEG_RECORD_FILE);
    let bytes =
        std::fs::read(&path).map_err(|error| KegError::io("read keg record", &path, error))?;
    let record = serde_json::from_slice(&bytes).map_err(|error| {
        KegError::Layout(format!("parse keg record '{}': {error}", path.display()))
    })?;
    Ok(record)
}

/// Staging directory prefix (never treated as a keg).
/// Tiền tố thư mục staging (không bao giờ coi là keg).
pub const STAGING_PREFIX: &str = ".staging-";

fn staging_name(version: &Version) -> String {
    let sequence = NEXT_STAGING.fetch_add(1, Ordering::Relaxed);
    format!(
        "{STAGING_PREFIX}{version}.{}.{sequence}",
        std::process::id()
    )
}

/// Remove a leftover staging directory from an aborted previous run.
/// Xoá staging còn sót từ lần chạy trước bị huỷ.
fn remove_stale_staging(staging: &Path) -> KegResult<()> {
    if staging.exists() {
        remove_keg(staging)?;
    }
    Ok(())
}

/// Restore owner write on the staging root only (record placement).
/// Chỉ mở quyền ghi cho root staging (để đặt record).
fn restore_root_write(staging: &Path) -> KegResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::symlink_metadata(staging)
            .map_err(|error| KegError::io("inspect keg staging directory", staging, error))?;
        let mode = metadata.permissions().mode() & 0o777;
        std::fs::set_permissions(staging, std::fs::Permissions::from_mode(mode | 0o200))
            .map_err(|error| KegError::io("restore keg staging write", staging, error))
    }
    #[cfg(not(unix))]
    {
        let _ = staging;
        Ok(())
    }
}

/// Persist the directory entry created by a rename.
/// Đồng bộ entry thư mục sau rename.
fn sync_dir(directory: &Path) -> KegResult<()> {
    #[cfg(unix)]
    {
        std::fs::File::open(directory)
            .and_then(|handle| handle.sync_all())
            .map_err(|error| KegError::io("sync keg directory", directory, error))
    }
    #[cfg(not(unix))]
    {
        let _ = directory;
        Ok(())
    }
}
