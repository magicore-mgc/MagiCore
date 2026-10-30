//! GATE-EXEMPT: installs a pre-built binary artifact, not a package
//! dependency — nothing resolves a registry manifest, so there is no
//! dependency resolution to route through the lane gate.
//! (GATE-EXEMPT: cài artifact binary dựng sẵn, không phải dependency package —
//! không resolve manifest registry nên không có dependency resolution để đi qua
//! lane gate.)
//!
//! `mgc install-lib` — install a signed binary into the lib keg store (E-3).
//! `mgc install-lib` — cài binary có chữ ký vào keg store của core lib (E-3).
//!
//! Flow: resolve the binary (manifest or PATH probe) → verify the Ed25519
//! signature against a pinned key → download with a byte cap → verify the
//! BLAKE3 digest from the signed document → materialise into a keg → switch
//! the active version. A failure at any step leaves the previous active
//! version untouched.
//! (Luồng: resolve binary (manifest hoặc dò PATH) → verify chữ ký Ed25519 với
//! key đã pin → tải có giới hạn byte → verify digest BLAKE3 từ tài liệu đã ký
//! → materialize vào keg → switch version active. Lỗi ở bất kỳ bước nào đều
//! không đụng version active trước đó.)

use std::path::{Path, PathBuf};

/// Process-wide lock for tests that install env-dependent fixtures: the keg
/// store is resolved from `MGC_DATA_DIR`, an environment variable.
/// (Lock toàn process cho test cài fixture phụ thuộc env: keg store được
/// resolve từ `MGC_DATA_DIR`, một biến môi trường.)
#[cfg(test)]
pub(crate) fn test_env_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

/// Environment variable that carries the pinned manifest trust key.
/// Biến môi trường chứa key tin cậy đã pin cho manifest.
pub const TRUST_KEY_ENV: &str = "MGC_LIB_TRUST_KEY";

/// What the user asked for.
/// Yêu cầu của người dùng.
#[derive(Debug)]
pub struct InstallLibRequest<'a> {
    /// Binary name, path or URL token as typed.
    /// Tên binary, đường dẫn hoặc token URL như người dùng nhập.
    pub target: &'a str,
    /// Manifest file; `None` means "find the tool on PATH".
    /// File manifest; `None` nghĩa là "tìm tool trên PATH".
    pub manifest: Option<&'a Path>,
    /// Key id in the keyring used to sign the manifest. Not used yet: signing
    /// lives in `mgc trust` and installation only verifies pinned keys, so a
    /// caller cannot install by naming a signing key.
    /// (Key id trong keyring dùng để ký manifest. Chưa dùng: việc ký nằm ở
    /// `mgc trust`, cài đặt chỉ verify key đã pin, nên caller không thể cài
    /// bằng cách chỉ định key ký.)
    #[allow(dead_code)]
    pub key_id: Option<&'a str>,
    /// Trust keys pinned by the user (base64 Ed25519 public keys).
    /// Key tin cậy do người dùng pin (public key Ed25519 base64).
    pub trust_keys: Vec<String>,
}

/// Result of a successful install.
/// Kết quả của một lần cài thành công.
#[derive(Debug)]
pub struct InstallLibOutcome {
    /// Binary name that is now active.
    /// Tên binary đang active.
    pub name: String,
    /// Version installed.
    /// Version đã cài.
    pub version: String,
    /// Keg directory holding the sealed artifact.
    /// Thư mục keg chứa artifact đã seal.
    pub keg_dir: PathBuf,
    /// True when the version was already present and only re-activated.
    /// True khi version đã có và chỉ được kích hoạt lại.
    pub reused: bool,
}

/// Execute the install.
/// Thực thi việc cài.
pub async fn execute(request: InstallLibRequest<'_>) -> anyhow::Result<InstallLibOutcome> {
    let store = mgc_lib_adapter::install::keg::KegStore::resolve()
        .map_err(|error| anyhow::anyhow!("cannot resolve the lib keg store: {error}"))?;

    let mut trust_keys = request.trust_keys.clone();
    if let Ok(from_env) = std::env::var(TRUST_KEY_ENV) {
        trust_keys.extend(
            from_env
                .split(',')
                .map(str::trim)
                .filter(|key| !key.is_empty())
                .map(str::to_string),
        );
    }

    match request.manifest {
        Some(path) => install_from_manifest(&store, path, &request, &trust_keys).await,
        None => install_from_path(&store, request.target),
    }
}

/// Install a binary that is already present on this machine (PATH probe).
/// Cài binary đã có sẵn trên máy (dò PATH).
///
/// This is the local, offline path: the artifact is copied from the tool the
/// user already trusts, then sealed and activated. No manifest is involved, so
/// the binary is installed under its own file name.
/// (Đây là đường local/offline: artifact được copy từ tool người dùng đã tin,
/// rồi seal và kích hoạt. Không có manifest nên binary cài dưới chính tên file.)
fn install_from_path(
    store: &mgc_lib_adapter::install::keg::KegStore,
    target: &str,
) -> anyhow::Result<InstallLibOutcome> {
    let source = resolve_on_path(target)?;
    let name = source
        .file_name()
        .and_then(|file| file.to_str())
        .ok_or_else(|| anyhow::anyhow!("cannot derive a binary name from '{}'", source.display()))?
        .to_string();
    let version = version_of(&source);

    let source_for_keg = source.clone();
    // P-6: install through the keg facade so the binary is also linked onto the
    // keg bin path; sealing without a link left the artifact unreachable.
    // (P-6: cài qua facade keg để binary cũng được link vào bin path của keg;
    // seal mà không link khiến artifact không truy cập được.)
    let outcome =
        mgc_lib_adapter::install::keg::install_binary(store, &name, &version, &name, |staging| {
            copy_executable(&source_for_keg, staging.join(&name).to_path_buf()).map_err(|error| {
                mgc_lib_adapter::install::keg::KegError::Io {
                    operation: "copy binary",
                    path: staging.join(&name),
                    source: std::io::Error::other(error.to_string()),
                }
            })
        })
        .map_err(|error| anyhow::anyhow!("cannot install '{name}' into the keg store: {error}"))?;

    Ok(InstallLibOutcome {
        name,
        version,
        keg_dir: store.keg_dir(&outcome.keg.name, &outcome.keg.version)?,
        reused: outcome.reused,
    })
}

/// Install from a signed manifest.
/// Cài từ manifest có chữ ký.
async fn install_from_manifest(
    store: &mgc_lib_adapter::install::keg::KegStore,
    manifest_path: &Path,
    request: &InstallLibRequest<'_>,
    trust_keys: &[String],
) -> anyhow::Result<InstallLibOutcome> {
    // A pinned key is mandatory: an unpinned manifest is an unauthenticated
    // download (E-3).
    // (Key pin là bắt buộc: manifest không pin là tải chưa xác thực — E-3.)
    let manifest = mgc_lib_adapter::install::keg::load_manifest(manifest_path, trust_keys)
        .map_err(|error| anyhow::anyhow!("{error}"))?;
    let entry = manifest.entry.clone();

    // Download to a private temp file first; the digest check happens before
    // anything reaches the keg.
    // (Tải về file tạm riêng trước; kiểm digest xong mới vào keg.)
    let staging = tempfile::tempdir()
        .map_err(|error| anyhow::anyhow!("cannot create a download staging dir: {error}"))?;
    let download_path = staging.path().join(&entry.name);
    mgc_fetcher::download::download_with_progress(&entry.url, &download_path, None)
        .await
        .map_err(|error| anyhow::anyhow!("cannot download '{}': {error}", entry.url))?;

    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_file(&download_path)
        .map_err(|error| anyhow::anyhow!("cannot hash the downloaded artifact: {error}"))?;
    if digest.to_hex() != entry.blake3 {
        // Never install bytes that differ from the signed document.
        // (Không bao giờ cài bytes khác tài liệu đã ký.)
        anyhow::bail!(
            "downloaded artifact '{}' does not match the signed BLAKE3 digest \
             (signed {}, got {})",
            entry.name,
            entry.blake3,
            digest.to_hex()
        );
    }

    let source = download_path.clone();
    let keg_name = request
        .target
        .rsplit_once('@')
        .map(|(name, _)| name)
        .unwrap_or(&entry.name)
        .to_string();
    let source_for_keg = source.clone();
    let keg_file = entry.name.clone();
    // P-6: the manifest lane exposes the same shim as the PATH lane, so a
    // verified download is runnable without hunting for the keg path.
    // (P-6: lane manifest phơi cùng shim như lane PATH, nên bản tải đã verify
    // chạy được mà không phải đi tìm đường dẫn keg.)
    let outcome = mgc_lib_adapter::install::keg::install_binary(
        store,
        &keg_name,
        &entry.version,
        &keg_file,
        |dest| {
            copy_executable(&source_for_keg, dest.join(&keg_file).to_path_buf()).map_err(|error| {
                mgc_lib_adapter::install::keg::KegError::Io {
                    operation: "copy binary",
                    path: dest.join(&keg_file),
                    source: std::io::Error::other(error.to_string()),
                }
            })
        },
    )
    .map_err(|error| {
        anyhow::anyhow!(
            "cannot install '{}@{}' into the keg store: {error}",
            keg_name,
            entry.version
        )
    })?;

    Ok(InstallLibOutcome {
        name: keg_name,
        version: entry.version,
        keg_dir: store.keg_dir(&outcome.keg.name, &outcome.keg.version)?,
        reused: outcome.reused,
    })
}

/// Locate a binary on `PATH` without spawning anything.
/// Tìm binary trên `PATH` mà không spawn gì.
fn resolve_on_path(target: &str) -> anyhow::Result<PathBuf> {
    let candidate = Path::new(target);
    if candidate.is_file() {
        return Ok(absolute(candidate));
    }
    if candidate.components().count() > 1 {
        anyhow::bail!("'{target}' does not exist; pass an existing path or a command name on PATH");
    }
    let path = std::env::var_os("PATH")
        .ok_or_else(|| anyhow::anyhow!("PATH is not set; cannot locate '{target}'"))?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(target);
        if candidate.is_file() {
            return Ok(absolute(&candidate));
        }
    }
    anyhow::bail!("'{target}' is not on PATH and is not an existing file")
}

fn absolute(path: &Path) -> PathBuf {
    std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf())
}

/// Version reported by the tool, or `0.0.0` when it reports nothing.
/// Version do tool báo, hoặc `0.0.0` khi tool không báo gì.
///
/// Version discovery spawns the tool with `--version`, which is a read-only
/// probe already allowed by the exec allowlist for third-party tools.
/// (Dò version spawn tool với `--version`, probe chỉ đọc đã nằm trong allowlist
/// exec cho tool bên thứ ba.)
fn version_of(binary: &Path) -> String {
    let options = mgc_exec::prelude::ExecOptions {
        timeout: Some(std::time::Duration::from_secs(10)),
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::FrameworkInitializer),
        ..Default::default()
    };
    match mgc_exec::prelude::run(
        binary.to_string_lossy().as_ref(),
        &["--version".to_string()],
        &options,
    ) {
        Ok(output) if output.exit_code == 0 => parse_version(&output.stdout_tail),
        _ => "0.0.0".to_string(),
    }
}

/// First semver-looking token in a version banner.
/// Token giống semver đầu tiên trong banner version.
fn parse_version(banner: &str) -> String {
    banner
        .split(|character: char| !(character.is_ascii_digit() || character == '.'))
        .find(|token| {
            token.matches('.').count() == 2 && token.starts_with(|c: char| c.is_ascii_digit())
        })
        .unwrap_or("0.0.0")
        .to_string()
}

/// Copy an executable into the keg staging directory.
/// Copy executable vào thư mục staging của keg.
fn copy_executable(source: &Path, destination: PathBuf) -> anyhow::Result<()> {
    std::fs::copy(source, &destination).map_err(|error| {
        anyhow::anyhow!("cannot copy binary from '{}': {error}", source.display())
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&destination, std::fs::Permissions::from_mode(0o755)).map_err(
            |error| {
                anyhow::anyhow!(
                    "cannot make '{}' executable: {error}",
                    destination.display()
                )
            },
        )?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/library.rs"]
mod test;
