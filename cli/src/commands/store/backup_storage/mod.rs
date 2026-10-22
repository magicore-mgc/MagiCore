//! Wires backup tree, clear and default-path owners.
//! Nối owner copy cây, clear và đường dẫn backup mặc định.

mod clear_tree;
mod copy_tree;
mod default_path;

pub(super) use clear_tree::clear_dir;
pub(super) use copy_tree::CopyTreeError;
pub(super) use copy_tree::{
    ValidatedSourceTree, copy_dir, copy_validated_tree, ensure_empty_destination_directory,
    prepare_destination_root, prospective_destination_root, validate_present_tree,
};
pub(super) use default_path::default_backup_dir;
