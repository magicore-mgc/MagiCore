//! Routes outdated queries by detected core identity.
//! Định tuyến query outdated theo core đã nhận diện.

#[cfg(feature = "web")]
use super::super::web::query_outdated_web;
use super::super::{model::OutdatedPkg, native::query_outdated_native};
use crate::context::ProjectContext;
use anyhow::Result;

pub(in crate::commands::outdated) async fn query_outdated(
    core: Option<&str>,
) -> Result<Vec<OutdatedPkg>> {
    let ctx = ProjectContext::load_with_core(core)?;
    let adapter = ctx.adapter();
    if !is_web_core(adapter.core_id()) {
        return query_outdated_native(&ctx).await;
    }

    #[cfg(feature = "web")]
    {
        query_outdated_web(ctx).await
    }
    #[cfg(not(feature = "web"))]
    {
        let _ = adapter;
        Err(crate::error::outdated_no_web_adapter())
    }
}

/// Web alone uses the shared npm metadata lane; all other known cores use native resolvers.
/// Chỉ Web dùng lane npm chung; mọi core đã biết khác dùng native resolver.
fn is_web_core(core_id: &str) -> bool {
    core_id == "web"
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
