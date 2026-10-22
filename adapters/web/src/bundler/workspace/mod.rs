//! Web build workspace preparation wiring.
//! Wiring chuẩn bị workspace build Web.

mod workspace;

pub(super) use workspace::{detect_working_dir, prepare_workspace};
