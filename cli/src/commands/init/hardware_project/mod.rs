//! Hardware project owner exports.
//! Export owner tạo dự án Hardware.

mod hardware_project;

pub(in crate::commands::init) use hardware_project::init_hardware;
