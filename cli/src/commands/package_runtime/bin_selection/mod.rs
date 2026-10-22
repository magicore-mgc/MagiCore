//! Bin Selection owner wiring.
//! Nối owner bin_selection.

mod bin_selection;
mod error;
pub(crate) use bin_selection::{select_bin_name, valid_bin_name};
pub use error::{
    registry_package_ambiguous_bin, registry_package_invalid_bin, registry_package_undeclared_bin,
};
