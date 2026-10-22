//! Interactive prompt wiring — wiring prompt tương tác.
//! Re-exports the existing prompt operations and keeps the module path stable.
//! Re-export các thao tác cũ và giữ nguyên module path.

mod confirm;
mod input;
mod multi_select;
mod password;
mod prompt_style;
mod select;

use prompt_style::style_prompt;

#[cfg(test)]
#[path = "test/prompt.rs"]
mod tests;

pub use confirm::confirm;
pub use input::input;
pub use multi_select::multi_select;
pub use password::password;
pub use select::select;
