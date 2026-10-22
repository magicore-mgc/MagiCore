//! Core identity contract implementation.
//! Triển khai hợp đồng định danh core.

use crate::ecosystem::Ecosystem;

/// Core identity — shared by every capability trait so default bodies can
/// build a precise `Unsupported` error (`core_id` returns `&'static str`
/// because `MgError::Unsupported` stores static strings; a METHOD, not an
/// associated const, keeps the trait dyn-compatible for
/// `Arc<dyn PackageAdapter>`).
/// Danh tính core — mọi capability trait kế thừa để default body sinh lỗi
/// `Unsupported` chuẩn (`core_id` trả `&'static str` vì
/// `MgError::Unsupported` lưu chuỗi tĩnh; dùng METHOD thay associated const
/// để trait vẫn dyn-compatible cho `Arc<dyn PackageAdapter>`).
pub trait CoreIdent {
    /// Static core id as used by the CLI and the lifecycle matrix
    /// ("web", "lib", "ai", "app", "game", "iot", "clo", "cicd", "hardware").
    /// Id core tĩnh trùng với CLI và lifecycle matrix.
    fn core_id(&self) -> &'static str;

    /// Human-readable core name — moved verbatim from `PackageAdapter`.
    /// Tên core — di chuyển nguyên vẹn từ `PackageAdapter`.
    fn name(&self) -> &str;

    /// Ecosystem discriminator — moved verbatim from `PackageAdapter`.
    /// Ecosystem — di chuyển nguyên vẹn từ `PackageAdapter`.
    fn ecosystem(&self) -> Ecosystem;
}
