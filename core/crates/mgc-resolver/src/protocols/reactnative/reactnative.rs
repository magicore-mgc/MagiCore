//! Stable React Native lock-marker prefixes.
//! Tiền tố marker lock React Native ổn định.

/// Marker written for the iOS tier's verified spec sha1.
/// Marker chứa sha1 spec đã xác minh của tier iOS.
pub const PODSPEC_SHA1_MARKER_PREFIX: &str = "podspec-sha1:";

/// Marker prefix recording which React Native tier produced a lock entry.
/// Tiền tố marker ghi tier React Native tạo lock entry.
pub const RN_TIER_MARKER_PREFIX: &str = "rn-tier:";

#[cfg(test)]
#[path = "test/reactnative.rs"]
mod tests;
