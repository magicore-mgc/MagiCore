//! App audit owner wiring.
//! File này chỉ nối router và scanner; không triển khai logic audit.

mod audit;
pub mod scanner;

pub use audit::run_audit;
