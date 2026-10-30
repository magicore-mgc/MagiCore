//! Read Web release-age policy from project config or shared storage.
//! Đọc policy tuổi phát hành Web từ cấu hình project hoặc shared storage.

use std::path::Path;

use mgc_config::project::load_security_config;
use mgc_store::{Database, Layout};

use crate::lockfile::project_cache_dir;
use crate::supply_chain::{error::SupplyChainError, types::AgePolicySettings};

/// Read the configured Web release age, preserving project-first precedence.
/// Đọc tuổi phát hành Web theo precedence project trước.
pub(in crate::supply_chain) fn configured_store_min_age()
-> Result<AgePolicySettings, SupplyChainError> {
    let cwd = std::env::current_dir().map_err(|source| SupplyChainError::ProjectRoot { source })?;
    configured_store_min_age_at(&cwd)
}

fn configured_store_min_age_at(cwd: &Path) -> Result<AgePolicySettings, SupplyChainError> {
    let config_path = cwd.join("mgc.toml");
    let security =
        load_security_config(cwd).map_err(|source| SupplyChainError::ProjectSecurityConfig {
            path: config_path,
            source,
        })?;
    let project_age_secs = security
        .as_ref()
        .and_then(|settings| settings.min_age_for_ecosystem("web"))
        .map(|hours| project_hours_to_seconds(hours, &cwd.join("mgc.toml")))
        .transpose()?;
    let allow_missing_time = security
        .as_ref()
        .and_then(|settings| settings.allow_missing_time)
        .unwrap_or(false);

    if project_age_secs.is_some() {
        return Ok(AgePolicySettings {
            min_age_secs: project_age_secs,
            allow_missing_time,
        });
    }

    let layout = Layout::new(project_cache_dir(cwd));
    let store_age_secs = read_store_min_age_secs(&layout.db_path())?;

    // Neither the project nor the store named an age: the documented default
    // applies, exactly like `mgc versions` already does. Reporting `None` here
    // made `mgc install` disagree with `mgc versions` AND made the resolver
    // fetch abbreviated packuments (no `time`) while the guard expected them.
    // (Không project/store nào đặt tuổi thì áp mặc định, đúng như `mgc versions`.
    // Trả `None` ở đây khiến `mgc install` lệch `mgc versions` và resolver fetch
    // abbreviated packument không có `time` trong khi guard cần timestamp.)
    let min_age_secs = match store_age_secs {
        Some(seconds) => Some(seconds),
        None => Some(project_hours_to_seconds(
            mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS,
            &cwd.join("mgc.toml"),
        )?),
    };

    Ok(AgePolicySettings {
        min_age_secs,
        allow_missing_time,
    })
}

fn project_hours_to_seconds(hours: u64, path: &Path) -> Result<i64, SupplyChainError> {
    let seconds = hours
        .checked_mul(60 * 60)
        .and_then(|value| i64::try_from(value).ok())
        .ok_or_else(|| SupplyChainError::MinimumAgeOutOfRange {
            path: path.to_path_buf(),
            value: hours,
            unit: "hours",
        })?;
    Ok(seconds)
}

fn read_store_min_age_secs(path: &Path) -> Result<Option<i64>, SupplyChainError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => {}
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(SupplyChainError::StorePolicyInspect {
                path: path.to_path_buf(),
                source,
            });
        }
    }

    let database =
        Database::open_read_only(path).map_err(|source| SupplyChainError::StorePolicyDatabase {
            path: path.to_path_buf(),
            source,
        })?;
    let Some(seconds) =
        database
            .release_policy("web")
            .map_err(|source| SupplyChainError::StorePolicyDatabase {
                path: path.to_path_buf(),
                source,
            })?
    else {
        return Ok(None);
    };

    i64::try_from(seconds)
        .map(Some)
        .map_err(|_| SupplyChainError::MinimumAgeOutOfRange {
            path: path.to_path_buf(),
            value: seconds,
            unit: "seconds",
        })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
