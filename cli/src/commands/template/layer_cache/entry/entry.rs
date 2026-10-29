//! Public entry for ensuring a template layer is available.
//! Entry công khai đảm bảo template layer sẵn có.

use anyhow::Result;

use super::super::flow::ensure_layer_in;
use crate::commands::template::templates_cache_dir;

/// Đảm bảo template layer có sẵn (disk / cache) — nếu thiếu và registry có config,
/// tự fetch. Registry-first: klass pnpm đua `create-*` từ registry.
///
/// Returns typed ScaffoldResolveStatus thay vì bool - KHÔNG được bỏ qua!
/// Reject an unsafe template cache for `rel` before any host tool runs.
/// Từ chối cache template không an toàn cho `rel` trước khi chạy host tool.
///
/// The layer resolver already refuses symlinked or non-directory cache entries,
/// but a core may prefer its own language scaffolder and never reach the
/// resolver, so a planted entry would be ignored instead of aborting.
/// (Resolver layer đã từ chối entry cache là symlink/không phải thư mục, nhưng
/// core có thể dùng scaffolder của ngôn ngữ và không bao giờ tới resolver, nên
/// entry bị cài sẽ bị bỏ qua thay vì dừng.)
pub fn ensure_cache_healthy(rel: &str) -> anyhow::Result<()> {
    // Only the local cache path is inspected: nothing is fetched or created.
    // (Chỉ kiểm path cache cục bộ: không tải, không tạo gì.)
    let cache_root = templates_cache_dir();
    if !cache_root.exists() {
        return Ok(());
    }
    let entry = cache_root.join(rel);
    match std::fs::symlink_metadata(&entry) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err(crate::scaffold::error::template_cache_unsafe_path(&entry))
        }
        _ => Ok(()),
    }
}

pub async fn ensure_layer(
    rel: &str,
) -> Result<
    crate::scaffold::resolver::ScaffoldResolveStatus,
    crate::scaffold::resolver::ScaffoldResolveError,
> {
    ensure_layer_in(rel, &templates_cache_dir(), None, None).await
}
