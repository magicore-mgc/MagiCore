//! Wires the `parse_hyphen_range` helper and its focused tests.
//! Nối helper `parse_hyphen_range` và test chuyên biệt.

mod parse_hyphen_range;
pub(super) use self::parse_hyphen_range::parse_hyphen_range;
#[cfg(test)]
mod test;
