//! Wire fail-closed report consistency checks and tests.
//! Nối kiểm tra consistency fail-closed và test.

#[path = "consistency.rs"]
mod consistency;

pub(super) use self::consistency::validate_vulnerabilities;

#[cfg(test)]
#[path = "test/consistency.rs"]
mod tests;
