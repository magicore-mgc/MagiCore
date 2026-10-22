//! Wire source-selection policy owners.
//! Nối các owner policy chọn source.

mod catch_all_fallback;
mod error;
mod source_selection;
mod specific_claim;
mod unmatched_fallback;

pub use error::SourceSelectionError;
pub use source_selection::{UNKNOWN_SOURCE_PRIORITY, select_source};
