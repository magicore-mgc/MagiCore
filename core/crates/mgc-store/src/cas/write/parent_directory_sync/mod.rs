//! Exports platform-specific parent-directory durability sync.
//! Export đồng bộ độ bền thư mục cha theo platform.

mod parent_directory_sync;

pub use self::parent_directory_sync::sync_parent_dir;
