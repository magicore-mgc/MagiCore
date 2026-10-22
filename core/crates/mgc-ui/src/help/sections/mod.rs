//! Private help section wiring — wiring nội bộ các section help.
//! Each same-name owner renders one stable output section.
//! Mỗi owner cùng tên dựng một section output ổn định.

mod command_name;
mod common;
mod create;
mod options;
mod per_core;
mod single_core;

pub(super) use common::append_common_section;
pub(super) use create::append_create_section;
pub(super) use options::append_options_section;
pub(super) use per_core::append_per_core_section;
pub(super) use single_core::append_single_core_section;
