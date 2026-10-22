//! Shared result passed from package resolution to later add phases.
//! Kết quả dùng chung chuyển từ phase resolve sang các phase add tiếp theo.

use super::super::AddedPackage;
use mgc_types::PackageId;

/// State produced by resolving one ordered add request.
/// Trạng thái do resolve một request add có thứ tự tạo ra.
pub(in crate::commands::core::shared::implementation::implementation::package_add) struct AddProgress
{
    pub(in crate::commands::core::shared::implementation::implementation::package_add) added_ids:
        Vec<PackageId>,
    pub(in crate::commands::core::shared::implementation::implementation::package_add) added_packages:
        Vec<AddedPackage>,
    pub(in crate::commands::core::shared::implementation::implementation::package_add) changed_any:
        bool,
}
