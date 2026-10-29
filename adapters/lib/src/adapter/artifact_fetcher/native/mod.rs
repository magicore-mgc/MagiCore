//! Native artifact-fetch owners, one folder per language.
//! Owner fetch artifact native, mỗi ngôn ngữ một folder.

#[path = "native.rs"]
mod native;

pub(in crate::adapter) mod dotnet;
pub(in crate::adapter) mod go;
pub(in crate::adapter) mod java;
pub(in crate::adapter) mod python;
pub(in crate::adapter) mod rust;

pub(in crate::adapter) use native::fetch_with_protocol;

use mgc_store::ContentStore;
use mgc_types::{MgResult, adapter::ResolvedGraph};

use crate::language::LibLanguage;

/// Dispatch `graph` to the language's own fetch owner.
///
/// One match keeps the language→protocol decision in a single readable place
/// while each implementation stays in its own folder.
/// (Dispatch theo language; một match duy nhất cho quyết định
/// language→protocol, còn từng implementation nằm trong folder riêng.)
pub(in crate::adapter) async fn fetch(
    language: LibLanguage,
    graph: &ResolvedGraph,
    store: &ContentStore,
) -> MgResult<usize> {
    match language {
        LibLanguage::Rust => rust::fetch(graph, store).await,
        LibLanguage::Python => python::fetch(graph, store).await,
        LibLanguage::Go => go::fetch(graph, store).await,
        LibLanguage::Java => java::fetch(graph, store).await,
        LibLanguage::DotNet => dotnet::fetch(graph, store).await,
        // TypeScript never reaches here: `LibAdapter::fetch` hands it to the
        // Web engine first, which owns npm tarballs.
        // (TS không bao giờ tới đây: `fetch` chuyển cho Web engine trước.)
        LibLanguage::Ts => Err(mgc_types::MgError::Unsupported {
            core: "lib",
            capability: "fetch",
            guidance: "TypeScript artifacts are fetched by the embedded Web engine; \
                       call it through `mgc install` or `mgc build`"
                .to_string(),
        }),
    }
}
