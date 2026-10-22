//! Progress API wiring — wiring API progress.
//! Combines the previous helper functions and manager under one public owner.
//! Gom helper cũ và manager vào cùng owner public.

mod progress;
#[cfg(test)]
#[path = "test/progress.rs"]
mod tests;

pub use progress::{
    ProgressManager, add_multi_bar, create_multi_progress, create_progress_bar, create_spinner,
};
