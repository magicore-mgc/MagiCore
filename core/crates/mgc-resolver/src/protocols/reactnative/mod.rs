//! React Native layered dependencies across the JavaScript, Android, and iOS tiers.
//! Dependency React Native phân tầng giữa JavaScript, Android và iOS.
//!
//! The JS tier is resolved by the Web adapter's npm pipeline. Android
//! `gradle.lockfile` pins are exact Maven coordinates. iOS `Podfile.lock`
//! supplies the pod closure and SHA-1 map; each downloaded spec must match
//! its recorded checksum. Missing iOS lock input remains an explicit
//! fail-closed outcome in the app-layer resolver.
//!
//! Tier JS do pipeline npm của adapter Web resolve. Pin Android trong
//! `gradle.lockfile` là tọa độ Maven chính xác. `Podfile.lock` iOS cung cấp
//! bao đóng pod và map SHA-1; mỗi spec tải xuống phải khớp checksum đã ghi.
//! Thiếu lock iOS vẫn là kết quả fail-closed rõ ràng ở resolver tầng app.

mod cocoapods;
mod gradle_lock;
mod podfile_lock;
#[path = "reactnative.rs"]
mod tier_markers;

pub use cocoapods::{CocoaPodsProtocol, VerifiedPodSpec, cocoapods_spec_url};
pub use gradle_lock::{GradleLockEntry, collapse_gradle_pins, parse_gradle_lockfile};
pub use podfile_lock::{PodfileLock, PodfileLockPod, parse_podfile_lock, pod_root};
pub use tier_markers::{PODSPEC_SHA1_MARKER_PREFIX, RN_TIER_MARKER_PREFIX};
