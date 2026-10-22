#![cfg_attr(test, allow(clippy::unwrap_used))]

mod banner;
mod command_style;
pub mod help;
mod install_summary;
mod next_steps;
pub mod progress;
pub mod prompt;
mod quiet;
mod section;
mod status;
pub mod table;

pub use banner::print_banner;
pub use command_style::style_cmd;
pub use install_summary::{print_install_summary, print_install_summary_source};
pub use next_steps::print_next_steps;
pub use progress::{add_multi_bar, create_multi_progress, create_progress_bar, create_spinner};
pub use quiet::{is_quiet, set_quiet};
pub use section::section;
pub use status::{blank_line, error, hint, info, success, warning, warning_stderr};
