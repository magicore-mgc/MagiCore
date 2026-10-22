//! Carry the release-age threshold and missing-time decision together.
//! Mang ngưỡng tuổi release và quyết định timestamp thiếu cùng nhau.

/// Fully normalized policy values used by the Web resolver guard.
/// Giá trị policy đã chuẩn hóa cho guard resolver Web.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct AgePolicySettings {
    pub(crate) min_age_secs: Option<i64>,
    pub(crate) allow_missing_time: bool,
}
