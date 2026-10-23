//! Public entry for ensuring a template layer is available.
//! Entry công khai đảm bảo template layer sẵn có.

use anyhow::Result;

use super::super::flow::ensure_layer_in;
use crate::commands::template::templates_cache_dir;

/// Đảm bảo template layer có sẵn (disk / cache) — nếu thiếu và registry có config,
/// tự fetch. Registry-first: klass pnpm đua `create-*` từ registry.
///
/// Returns typed ScaffoldResolveStatus thay vì bool - KHÔNG được bỏ qua!
pub async fn ensure_layer(
    rel: &str,
) -> Result<
    crate::scaffold::resolver::ScaffoldResolveStatus,
    crate::scaffold::resolver::ScaffoldResolveError,
> {
    ensure_layer_in(rel, &templates_cache_dir(), None, None).await
}
