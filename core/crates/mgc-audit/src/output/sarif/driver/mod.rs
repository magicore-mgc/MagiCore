//! Wire SARIF driver construction and its tests.
//! Nối việc dựng SARIF driver và test của owner.

#[path = "driver.rs"]
mod driver;

pub(super) use self::driver::build_driver;

#[cfg(test)]
#[path = "test/driver.rs"]
mod tests;
