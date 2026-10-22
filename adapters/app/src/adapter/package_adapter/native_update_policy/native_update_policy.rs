//! Determine whether this App lane owns the full native update round trip.
//! Xác định lane App có sở hữu đầy đủ quy trình native update hay không.

use crate::adapter::AppAdapter;
use crate::language::AppLanguage;

pub(in crate::adapter::package_adapter) fn supports_native_update(adapter: &AppAdapter) -> bool {
    // Only Flutter owns resolve-first plus a round-tripping pubspec writer.
    // Chỉ Flutter có resolve-first và writer pubspec ghi/đọc vòng đầy đủ.
    matches!(adapter.language, AppLanguage::Flutter)
}
