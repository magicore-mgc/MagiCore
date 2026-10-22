//! Pending native resolution state and its fail-closed error owner.
//! State resolve native chờ install và owner lỗi fail-closed.

mod error;
mod pending_install_state;

use error::PendingInstallStateError;

pub(crate) use pending_install_state::PendingInstallState;
