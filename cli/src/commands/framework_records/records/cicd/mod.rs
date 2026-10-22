//! CI/CD record wiring — nối record CI/CD.
#[path = "cicd.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
