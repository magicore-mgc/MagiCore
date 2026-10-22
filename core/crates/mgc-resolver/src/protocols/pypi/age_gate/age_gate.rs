use crate::protocols::pypi::{PypiFile, PypiProtocol};
use mgc_types::Version;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug, Clone, Copy)]
pub(in crate::protocols::pypi) struct PypiAgeGate {
    pub(in crate::protocols::pypi) cutoff_hours: u64,
    pub(in crate::protocols::pypi) allow_missing_time: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::protocols::pypi) enum AgeGateExclusion {
    TooYoung,
    Unstamped,
}

impl PypiProtocol {
    pub(in crate::protocols::pypi) fn release_age_exclusion(
        files: &[PypiFile],
        policy: PypiAgeGate,
        now: OffsetDateTime,
        name: &str,
        version: &Version,
    ) -> Option<AgeGateExclusion> {
        let mut latest_upload: Option<OffsetDateTime> = None;
        let mut has_missing_timestamp = false;
        for file in files {
            let Some(raw_timestamp) = file.upload_time_iso_8601.as_deref() else {
                has_missing_timestamp = true;
                continue;
            };
            let Ok(timestamp) = OffsetDateTime::parse(raw_timestamp, &Rfc3339) else {
                has_missing_timestamp = true;
                continue;
            };
            latest_upload = Some(latest_upload.map_or(timestamp, |current| current.max(timestamp)));
        }

        let minimum_age_secs = policy.cutoff_hours.saturating_mul(60 * 60);
        if let Some(timestamp) = latest_upload {
            let age_seconds = now
                .unix_timestamp()
                .saturating_sub(timestamp.unix_timestamp());
            if age_seconds < 0 || (age_seconds as u64) < minimum_age_secs {
                return Some(AgeGateExclusion::TooYoung);
            }
        }

        if has_missing_timestamp {
            if policy.allow_missing_time {
                tracing::warn!(
                    package = name,
                    version = %version,
                    "minimum-release-age: PyPI timestamp missing or invalid; allowing via explicit escape hatch"
                );
                return None;
            }
            return Some(AgeGateExclusion::Unstamped);
        }

        None
    }
}
