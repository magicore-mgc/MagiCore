//! Shared scaffold version policy type and age-gate adapter.
//! Type policy version scaffold dùng chung và adapter age-gate.

/// Normalized age policy shared by scaffold selection and install policy.
/// Policy tuổi đã chuẩn hóa, dùng chung giữa scaffold và install.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScaffoldAgePolicy {
    pub cutoff_hours: u64,
    pub allow_missing_time: bool,
}

#[cfg(feature = "web")]
impl From<ScaffoldAgePolicy> for mgc_web_adapter::provider::AgePolicy {
    fn from(policy: ScaffoldAgePolicy) -> Self {
        Self {
            cutoff_hours: policy.cutoff_hours,
            allow_missing_time: policy.allow_missing_time,
        }
    }
}

#[cfg(test)]
#[path = "test/versions.rs"]
mod tests;
