//! Wires extraction-local errors.
//! Nối các lỗi cục bộ của luồng giải nén.

mod error;
pub(super) use self::error::{ExtractionError, ExtractionResult};
#[cfg(test)]
mod test;
