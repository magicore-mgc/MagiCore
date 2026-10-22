//! Native registry resolution owners.
//! Các owner resolve registry native.

#[path = "dependency_closure/mod.rs"]
mod dependency_closure;
#[path = "entry_index/mod.rs"]
mod entry_index;
#[path = "integrity_projection/mod.rs"]
mod integrity_projection;
#[path = "package_projection/mod.rs"]
mod package_projection;
#[path = "resolve/mod.rs"]
mod resolve;
#[path = "root_selection/mod.rs"]
mod root_selection;

pub use package_projection::NativeResolution;
pub use resolve::{resolve_with_protocol, resolve_with_protocol_including_dev};
