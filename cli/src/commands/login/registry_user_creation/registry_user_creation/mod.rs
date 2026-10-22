//! Wire the user-creation request implementation.
//! Nối implementation request tạo user.

#[path = "registry_user_creation.rs"]
mod registry_user_creation;

pub(crate) use registry_user_creation::create_registry_user;
pub(in crate::commands::login) use registry_user_creation::create_registry_user_inner;
