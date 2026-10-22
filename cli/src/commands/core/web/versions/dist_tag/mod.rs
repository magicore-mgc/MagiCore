//! Explicit npm dist-tag resolution under optional age policy.
//! Resolve npm dist-tag tường minh theo age policy tùy chọn.

mod dist_tag;
pub(crate) use dist_tag::resolve_npm_dist_tag_version;
#[cfg(all(test, feature = "web"))]
pub(in crate::commands::core::web::implementation) use dist_tag::selected_dist_tag_version;
