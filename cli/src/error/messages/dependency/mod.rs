//! Dependency graph and package inspection diagnostics.
//! Lỗi đồ thị dependency và tra cứu package.

mod error;

pub use error::{
    info_no_web_adapter, link_exists, link_usage, link_web_only, local_path_not_found,
    outdated_lock_unusable, outdated_no_web_adapter, unlink_not_linked, unlink_usage,
    unlink_web_only, why_web_only,
};
