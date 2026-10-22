//! Wires the `parse_wildcard_bounds` helper and its focused tests.
//! Nối helper `parse_wildcard_bounds` và test chuyên biệt.

mod parse_wildcard_bounds;
pub(super) use self::parse_wildcard_bounds::parse_wildcard_bounds;
#[cfg(test)]
mod test;
