//! Wires OS lock-contention classification and its contract tests.
//! Nối phân loại contention của OS lock và test contract.

mod contention;
pub(in crate::project_lock) use self::contention::is_lock_contention;
