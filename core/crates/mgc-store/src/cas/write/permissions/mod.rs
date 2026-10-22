//! Exports the private CAS blob permission operation to its parent owner.
//! Export thao tác permission blob CAS private cho owner cha.

mod permissions;

pub(super) use self::permissions::set_permissions;
