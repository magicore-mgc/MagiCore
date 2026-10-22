//! Wiring facade for wheel RECORD CSV parsing and errors.
//! Facade wiring cho parser CSV RECORD và lỗi cục bộ.

#[path = "error.rs"]
mod error;
#[path = "record_csv.rs"]
mod implementation;

#[cfg(test)]
pub(super) use error::RecordCsvError;
pub(super) use implementation::parse_record_row;
