//! Adapter/core ownership firewall for the generic install path.
//! Tường lửa ownership adapter/core cho đường install tổng quát.

mod validation;

pub(super) use validation::validate_install_request;
#[cfg(test)]
pub(super) use validation::{
    clo_adapter_path_gate, validate_install_owner, validate_install_owner_for_op,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
