//! Wires the `wildcard_satisfying_version` helper and its focused tests.
//! Nối helper `wildcard_satisfying_version` và test chuyên biệt.

mod wildcard_satisfying_version;
pub(super) use self::wildcard_satisfying_version::wildcard_satisfying_version;
#[cfg(test)]
mod test;
