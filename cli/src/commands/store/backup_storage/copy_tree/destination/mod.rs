//! Wires destination preparation and containment owners.
//! Nối các owner chuẩn bị destination và kiểm tra containment.

mod destination;
#[path = "directory_validation/mod.rs"]
mod directory_validation;
mod error;
#[path = "prepare_destination_parent/mod.rs"]
mod prepare_destination_parent;
#[path = "prospective_destination_root/mod.rs"]
mod prospective_destination_root;

pub(in crate::commands::store) use self::destination::prepare_destination_root;
pub(in crate::commands::store) use self::directory_validation::{
    ensure_destination_directory, ensure_empty_destination_directory,
};
pub(super) use self::error::DestinationError;
pub(super) use self::prepare_destination_parent::prepare_destination_parent;
pub(in crate::commands::store) use self::prospective_destination_root::prospective_destination_root;
