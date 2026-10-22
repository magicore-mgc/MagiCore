//! Lock registry for extraction coordination.
//! Registry lock cho điều phối extraction.
mod locks;
pub use locks::{extracted_package_root_lock, tarball_prefetch_lock};
