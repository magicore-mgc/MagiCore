//! Live project reference owner wiring.
//! File này chỉ nối module đọc reference sống của project.

mod references;

pub(super) use references::{
    count_web_shared_live_project_refs, read_web_shared_pinned_package_roots,
};
