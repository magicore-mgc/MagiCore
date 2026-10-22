//! Apply the publish-age guard to resolved registry metadata.
//! Áp dụng guard tuổi phát hành cho metadata registry đã resolve.

use std::collections::HashMap;
use std::sync::Arc;

use mgc_resolver::solver::Resolution;

use super::super::super::error::SupplyChainError;
use crate::native::npm_registry::{PackageMetadata, check_publish_age};

/// Reject missing package metadata, disallowed unknown time, and releases inside quarantine.
/// Từ chối thiếu metadata, thời gian chưa rõ không được phép và release còn trong quarantine.
pub(in crate::supply_chain) fn enforce_publish_age(
    resolutions: &[Resolution],
    metadata: &HashMap<String, Arc<PackageMetadata>>,
    min_age_secs: i64,
    allow_missing_time: bool,
) -> Result<(), SupplyChainError> {
    if min_age_secs <= 0 {
        return Ok(());
    }
    if chrono::TimeDelta::try_seconds(min_age_secs).is_none() {
        return Err(SupplyChainError::MinimumAgeSecondsOutOfRange {
            seconds: min_age_secs,
        });
    }

    for resolution in resolutions {
        let name = resolution.package_id.name_str();
        let version = resolution.package_id.version().to_string();
        let Some(pkg_meta) = metadata.get(name) else {
            return Err(SupplyChainError::MissingPackageMetadata {
                name: name.to_owned(),
                version,
            });
        };

        match pkg_meta.time.get(&version) {
            None if allow_missing_time => {
                super::super::super::warning::warn_once_if_missing_publish_time();
                continue;
            }
            None => {
                return Err(SupplyChainError::MissingPublishTimestamp {
                    name: name.to_owned(),
                    version,
                });
            }
            Some(timestamp) if chrono::DateTime::parse_from_rfc3339(timestamp).is_err() => {
                if allow_missing_time {
                    super::super::super::warning::warn_once_if_missing_publish_time();
                    continue;
                }
                return Err(SupplyChainError::InvalidPublishTimestamp {
                    name: name.to_owned(),
                    version,
                });
            }
            Some(_) => {}
        }

        match check_publish_age(pkg_meta, &version, min_age_secs) {
            Ok(true) => {}
            Ok(false) => {
                return Err(SupplyChainError::MissingPublishTimestamp {
                    name: name.to_owned(),
                    version,
                });
            }
            Err(message) => return Err(SupplyChainError::PublishAge { message }),
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
