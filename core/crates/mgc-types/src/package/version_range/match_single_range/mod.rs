//! Wires the `match_single_range` helper and its focused tests.
//! Nối helper `match_single_range` và test chuyên biệt.

mod match_single_range;
pub(super) use self::match_single_range::match_single_range;
#[cfg(test)]
mod test;
