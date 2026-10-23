//! Path check owner.
//! Owner kiểm path.

mod path_check;

pub(in crate::commands::store) use path_check::{validate_real_directory, validated_link_target};
