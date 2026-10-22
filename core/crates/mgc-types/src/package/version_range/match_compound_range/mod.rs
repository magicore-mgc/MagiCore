//! Wires the `match_compound_range` helper and its focused tests.
//! Nối helper `match_compound_range` và test chuyên biệt.

mod match_compound_range;
pub(super) use self::match_compound_range::match_compound_range;
#[cfg(test)]
mod test;
