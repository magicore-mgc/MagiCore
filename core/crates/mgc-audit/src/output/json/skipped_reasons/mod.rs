//! Wire skip-reason mapping and local tests.
//! Nối ánh xạ lý do bị bỏ qua và test cục bộ.

#[path = "skipped_reasons.rs"]
mod skipped_reasons;

pub(super) use self::skipped_reasons::skipped_reasons;

#[cfg(test)]
#[path = "test/skipped_reasons.rs"]
mod tests;
