//! mgc-platform/paths.rs — Standard paths (RULE §12: no hardcoded paths)
//! (Đường dẫn chuẩn: store/cache/logs/locks/dlx/advisories/patches/quarantine/registry + .magicore/lock-signatures.json)

use std::path::{Path, PathBuf};

pub const MGC_DATA_DIR_ENV: &str = "MGC_DATA_DIR";

/// Resolve the visible root for MagiCore-managed stores and caches.
/// Tìm root dễ thấy cho store/cache do MagiCore quản lý.
/// `MGC_DATA_DIR` overrides the default `~/MagiCore-SystemData` location.
/// `MGC_DATA_DIR` ghi đè vị trí mặc định `~/MagiCore-SystemData`.
pub fn mgc_data_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    if let Some(raw) = std::env::var_os(MGC_DATA_DIR_ENV).filter(|value| !value.is_empty()) {
        let path = PathBuf::from(raw);
        return if path.is_absolute() {
            path
        } else {
            home.join(path)
        };
    }
    home.join("MagiCore-SystemData")
}

/// Persistent roots for user-installed command-line tools and their shims.
/// Root bền vững cho CLI tool của user và shim nằm trong PATH.
pub fn global_tools_dir() -> PathBuf {
    mgc_data_dir().join("tools")
}

pub fn global_bin_dir() -> PathBuf {
    mgc_data_dir().join("bin")
}

/// Persistently add a managed executable directory to the current user's PATH.
/// Ghi PATH bền vững cho shell hiện tại; terminal mới sẽ nhận thay đổi.
pub fn ensure_global_bin_on_path() -> anyhow::Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("cannot resolve user home directory"))?;
    let shell = std::env::var("SHELL").unwrap_or_default();
    ensure_bin_on_path(&home, &shell, &global_bin_dir())
}

/// Add `bin` to the selected user's shell profile (or Windows user PATH).
/// Cho phép test và CLI cập nhật đúng profile theo shell/OS người dùng.
pub fn ensure_bin_on_path(home: &Path, shell: &str, bin: &Path) -> anyhow::Result<PathBuf> {
    let metadata = std::fs::symlink_metadata(bin)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        anyhow::bail!(
            "managed executable path '{}' must be a real directory",
            bin.display()
        );
    }
    let bin = bin.canonicalize()?;
    #[cfg(windows)]
    {
        ensure_windows_user_path(&bin)?;
        return Ok(bin);
    }
    #[cfg(not(windows))]
    {
        let shell_name = Path::new(shell)
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        let (profile, fish) = match shell_name {
            "fish" => {
                let directory = home.join(".config/fish/conf.d");
                ensure_directory_chain(home, &directory)?;
                (directory.join("magicore.fish"), true)
            }
            "zsh" => (home.join(".zprofile"), false),
            "bash" => {
                let login = home.join(".bash_profile");
                let profile = if std::fs::symlink_metadata(&login).is_ok() {
                    login
                } else {
                    home.join(".profile")
                };
                (profile, false)
            }
            _ => (home.join(".profile"), false),
        };
        ensure_profile_entry(&profile, &bin, fish)?;
        Ok(bin)
    }
}

#[cfg(not(windows))]
fn ensure_profile_entry(profile: &Path, bin: &Path, fish: bool) -> anyhow::Result<()> {
    use std::io::Read;

    const MAX_PROFILE_BYTES: u64 = 1024 * 1024;
    let metadata = match std::fs::symlink_metadata(profile) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            anyhow::bail!(
                "shell profile '{}' must not be a symlink",
                profile.display()
            )
        }
        Ok(metadata) if !metadata.is_file() => {
            anyhow::bail!(
                "shell profile '{}' must be a regular file",
                profile.display()
            )
        }
        Ok(metadata) => Some(metadata),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    let mut current = String::new();
    if let Some(metadata) = &metadata {
        if metadata.len() > MAX_PROFILE_BYTES {
            anyhow::bail!(
                "shell profile '{}' exceeds the 1 MiB safety limit",
                profile.display()
            );
        }
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        options
            .open(profile)?
            .take(MAX_PROFILE_BYTES + 1)
            .read_to_string(&mut current)?;
        if current.len() as u64 > MAX_PROFILE_BYTES {
            anyhow::bail!(
                "shell profile '{}' exceeds the 1 MiB safety limit",
                profile.display()
            );
        }
    }
    let desired = managed_path_profile_block(bin, fish)?;
    let updated = replace_managed_profile_block(&current, &desired)?;
    if updated == current {
        return Ok(());
    }
    atomic_replace_profile(profile, updated.as_bytes(), metadata.as_ref())
}

#[cfg(not(windows))]
fn managed_path_profile_block(bin: &Path, fish: bool) -> anyhow::Result<String> {
    let path = bin
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("managed executable path is not valid UTF-8"))?;
    if path.chars().any(char::is_control) {
        anyhow::bail!("managed executable path contains control characters");
    }
    if fish {
        let quoted = path.replace('\\', "\\\\").replace('\'', "\\'");
        Ok(format!(
            "# >>> magicore-managed-tool-bin >>>\nif not contains -- '{quoted}' $PATH\n    set -gx PATH '{quoted}' $PATH\nend\n# <<< magicore-managed-tool-bin <<<"
        ))
    } else {
        let quoted = path.replace('\'', "'\\''");
        Ok(format!(
            "# >>> magicore-managed-tool-bin >>>\ncase \":$PATH:\" in\n  *:'{quoted}':*) ;;\n  *) export PATH='{quoted}':\"$PATH\" ;;\nesac\n# <<< magicore-managed-tool-bin <<<"
        ))
    }
}

#[cfg(not(windows))]
fn replace_managed_profile_block(current: &str, desired: &str) -> anyhow::Result<String> {
    const START: &str = "# >>> magicore-managed-tool-bin >>>";
    const END: &str = "# <<< magicore-managed-tool-bin <<<";
    let starts = current
        .match_indices(START)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let ends = current
        .match_indices(END)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    match (starts.as_slice(), ends.as_slice()) {
        ([], []) => {
            let mut updated = current.to_string();
            if !updated.is_empty() && !updated.ends_with('\n') {
                updated.push('\n');
            }
            if !updated.is_empty() {
                updated.push('\n');
            }
            updated.push_str(desired);
            updated.push('\n');
            Ok(updated)
        }
        ([start], [end]) if *end >= *start => {
            let end = current[*end..]
                .find('\n')
                .map(|offset| *end + offset + 1)
                .unwrap_or(current.len());
            let mut updated = String::with_capacity(current.len() + desired.len());
            updated.push_str(&current[..*start]);
            updated.push_str(desired);
            updated.push('\n');
            updated.push_str(&current[end..]);
            Ok(updated)
        }
        _ => anyhow::bail!("shell profile contains malformed or duplicate MagiCore PATH markers"),
    }
}

#[cfg(not(windows))]
fn ensure_directory_chain(home: &Path, directory: &Path) -> anyhow::Result<()> {
    let relative = directory
        .strip_prefix(home)
        .map_err(|_| anyhow::anyhow!("shell config directory must be inside the home directory"))?;
    let mut current = home.to_path_buf();
    for component in relative.components() {
        current.push(component);
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                anyhow::bail!("shell config directory '{}' is unsafe", current.display())
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir(&current)?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&current, std::fs::Permissions::from_mode(0o700))?;
                }
            }
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn atomic_replace_profile(
    profile: &Path,
    content: &[u8],
    prior: Option<&std::fs::Metadata>,
) -> anyhow::Result<()> {
    use std::io::Write;

    let parent = profile
        .parent()
        .ok_or_else(|| anyhow::anyhow!("shell profile has no parent directory"))?;
    let temp = parent.join(format!(".magicore-path-{}.tmp", uuid_from_pid()));
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        let mode = prior
            .map(|metadata| metadata.mode() & 0o777)
            .unwrap_or(0o600);
        options.mode(mode).custom_flags(libc::O_NOFOLLOW);
    }
    let result = (|| -> anyhow::Result<()> {
        let mut file = options.open(&temp)?;
        file.write_all(content)?;
        file.sync_all()?;
        replace_profile_file(&temp, profile)?;
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}

#[cfg(not(windows))]
fn replace_profile_file(temp: &Path, profile: &Path) -> anyhow::Result<()> {
    std::fs::rename(temp, profile)?;
    Ok(())
}

#[cfg(windows)]
fn ensure_windows_user_path(bin: &Path) -> anyhow::Result<()> {
    const SCRIPT: &str = r#"$ErrorActionPreference = 'Stop'
$bin = [IO.Path]::GetFullPath($env:MGC_TOOL_BIN_PATH)
$current = [Environment]::GetEnvironmentVariable('Path', 'User')
$parts = @($current -split ';' | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$found = $false
foreach ($part in $parts) {
  try { if ([IO.Path]::GetFullPath([Environment]::ExpandEnvironmentVariables($part)).TrimEnd('\') -ieq $bin.TrimEnd('\')) { $found = $true } } catch { }
}
if (-not $found) { [Environment]::SetEnvironmentVariable('Path', (($bin + ';' + $current).Trim(';')), 'User') }
"#;
    let output = std::process::Command::new("powershell.exe")
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            SCRIPT,
        ])
        .env("MGC_TOOL_BIN_PATH", bin)
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "could not add MagiCore's bin directory to the Windows user PATH: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn uuid_from_pid() -> String {
    format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    )
}

/// Resolve stable per-project data outside the project's source tree.
/// Tách dữ liệu từng project khỏi cây mã nguồn.
pub fn project_data_dir(project_root: &Path) -> PathBuf {
    let canonical = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let identity = canonical.to_string_lossy();
    let digest = blake3::hash(identity.as_bytes()).to_hex();
    let name = canonical
        .file_name()
        .and_then(|value| value.to_str())
        .map(safe_data_component)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "project".to_string());
    mgc_data_dir()
        .join("projects")
        .join(format!("{name}-{}", &digest[..16]))
}

fn safe_data_component(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .take(48)
        .collect()
}

/// Standard subdirectories under project root (.magicore/)
pub struct ProjectPaths {
    pub root: PathBuf,
    pub store: PathBuf,
    pub cache: PathBuf,
    pub logs: PathBuf,
    pub locks: PathBuf,
    pub dlx: PathBuf,
    pub advisories: PathBuf,
    pub patches: PathBuf,
    pub quarantine: PathBuf,
    pub registry: PathBuf,
    pub lock_signatures: PathBuf, // sidecar .magicore/lock-signatures.json (ref: 16 §4, phản biện v2 #5)
}

impl ProjectPaths {
    /// Compute all standard paths from project root.
    pub fn from_root(root: &Path) -> Self {
        let mgc_dir = root.join(".magicore");
        Self {
            root: root.to_path_buf(),
            store: mgc_dir.join("store"),
            cache: mgc_dir.join("cache"),
            logs: mgc_dir.join("logs"),
            locks: mgc_dir.join("locks"),
            dlx: mgc_dir.join("dlx"),
            advisories: mgc_dir.join("advisories"),
            patches: mgc_dir.join("patches"),
            quarantine: mgc_dir.join("quarantine"),
            registry: mgc_dir.join("registry"),
            lock_signatures: mgc_dir.join("lock-signatures.json"),
        }
    }

    /// Ensure all directories exist.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.store)?;
        std::fs::create_dir_all(&self.cache)?;
        std::fs::create_dir_all(&self.logs)?;
        std::fs::create_dir_all(&self.locks)?;
        std::fs::create_dir_all(&self.dlx)?;
        std::fs::create_dir_all(&self.advisories)?;
        std::fs::create_dir_all(&self.patches)?;
        std::fs::create_dir_all(&self.quarantine)?;
        std::fs::create_dir_all(&self.registry)?;
        Ok(())
    }

    /// Path to patches directory (~/.magicore/patches/ or <project>/.magicore/patches/)
    pub fn patches_dir(&self) -> &Path {
        &self.patches
    }
}

/// User-global MagiCore data paths (MGC_DATA_DIR or ~/MagiCore-SystemData).
pub struct GlobalPaths {
    pub root: PathBuf,
    pub store: PathBuf,
    pub cache: PathBuf,
    pub logs: PathBuf,
    pub locks: PathBuf,
    pub dlx: PathBuf,
    pub advisories: PathBuf,
    pub patches: PathBuf,
    pub quarantine: PathBuf,
    pub registry: PathBuf,
    pub lock_signatures: PathBuf,
    /// Core-identity attestations by project (`identities/<path-hash>/attestation.json`).
    /// Chứng thực identity core theo project.
    pub identities: PathBuf,
}

impl GlobalPaths {
    /// Resolve user-global paths; `MAGICORE_STORE_ROOT` overrides only `store`.
    /// Resolve global paths from the visible data root; the legacy store override remains supported.
    /// Dùng root dữ liệu tập trung và vẫn hỗ trợ override store cũ.
    pub fn new() -> std::io::Result<Self> {
        let mgc_dir = mgc_data_dir();
        // Keep shared ecosystem stores aligned with the CAS override used by
        // mgc-store, including isolated Windows test and CI sandboxes.
        // Đồng bộ store ecosystem với override CAS của mgc-store, kể cả
        // sandbox test/CI Windows được cô lập.
        let store = std::env::var_os("MAGICORE_STORE_ROOT")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| mgc_dir.join("store"));
        Ok(Self {
            root: mgc_dir.clone(),
            store,
            cache: mgc_dir.join("cache"),
            logs: mgc_dir.join("logs"),
            locks: mgc_dir.join("locks"),
            dlx: mgc_dir.join("dlx"),
            advisories: mgc_dir.join("advisories"),
            patches: mgc_dir.join("patches"),
            quarantine: mgc_dir.join("quarantine"),
            registry: mgc_dir.join("registry"),
            lock_signatures: mgc_dir.join("lock-signatures.json"),
            identities: mgc_dir.join("identities"),
        })
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.store)?;
        std::fs::create_dir_all(&self.cache)?;
        std::fs::create_dir_all(&self.logs)?;
        std::fs::create_dir_all(&self.locks)?;
        std::fs::create_dir_all(&self.dlx)?;
        std::fs::create_dir_all(&self.advisories)?;
        std::fs::create_dir_all(&self.patches)?;
        std::fs::create_dir_all(&self.quarantine)?;
        std::fs::create_dir_all(&self.registry)?;
        Ok(())
    }

    pub fn patches_dir(&self) -> &Path {
        &self.patches
    }
}

/// Detect project root (walk up for markers) — shared with mgc-config/paths.rs
pub fn find_project_root(from: &Path) -> Option<PathBuf> {
    let mut current = from.to_path_buf();
    loop {
        if current.join("mgc.toml").exists()
            || current.join("mgc.lock").exists()
            || current.join("package.json").exists()
            || current.join("Cargo.toml").exists()
            || current.join("pyproject.toml").exists()
            || current.join(".git").exists()
        {
            return Some(current);
        }
        if !current.pop() {
            break;
        }
    }
    None
}
