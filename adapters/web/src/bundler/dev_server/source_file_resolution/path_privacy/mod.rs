//! Path privacy owner.
//! Owner privacy path.

mod path_privacy;

pub(crate) use path_privacy::{
    has_private_component, is_public_well_known_request, is_public_well_known_target,
};
