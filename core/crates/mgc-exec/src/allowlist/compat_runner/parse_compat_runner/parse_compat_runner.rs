use crate::allowlist::QUALIFIED_COMPAT_RUNNERS;
use anyhow::{Result, bail};

/// Parse `--compat-runner <manager>`: lowercase basename, must be qualified.
/// `None` (flag absent) means native mode — no delegation.
/// (Parse `--compat-runner <manager>`: basename lowercase, phải qualified.
/// `None` (không truyền flag) nghĩa là native mode — không ủy quyền.)
pub fn parse_compat_runner(flag: Option<&str>) -> Result<Option<String>> {
    let Some(raw) = flag else {
        return Ok(None);
    };
    let name = raw
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(raw)
        .to_ascii_lowercase();
    let name = name
        .strip_suffix(".exe")
        .or_else(|| name.strip_suffix(".cmd"))
        .unwrap_or(&name)
        .to_string();
    if !QUALIFIED_COMPAT_RUNNERS.contains(&name.as_str()) {
        bail!(
            "--compat-runner '{raw}' is not a qualified compat manager (qualified: {}); dependency/lifecycle operations stay owned by MagiCore",
            QUALIFIED_COMPAT_RUNNERS.join(", ")
        );
    }
    Ok(Some(name))
}
