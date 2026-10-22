/// Managers that may be named by `--compat-runner` (Spec rev4 §7/§9).
/// Only JS-ecosystem managers have a compat contract today (the reported
/// Next/Angular pain); Python/Gradle/Dart/etc. need separate analysis and
/// stay rejected with a clear error until their contracts exist.
/// (Manager được `--compat-runner` nhận (Spec rev4 §7/§9). Hiện chỉ manager
/// JS có compat contract (đúng chỗ đau Next/Angular đã báo); Python/Gradle/
/// Dart... cần phân tích riêng, vẫn bị từ chối rõ ràng.)
pub const QUALIFIED_COMPAT_RUNNERS: &[&str] = &["npm", "pnpm", "yarn", "bun"];
