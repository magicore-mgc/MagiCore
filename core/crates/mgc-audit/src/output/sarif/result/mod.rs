//! Wire vulnerability result projection and its tests.
//! Nối ánh xạ vulnerability result và test của owner.

#[path = "result.rs"]
mod result;

pub(super) use self::result::map_vulnerability;

#[cfg(test)]
#[path = "test/result.rs"]
mod tests;
