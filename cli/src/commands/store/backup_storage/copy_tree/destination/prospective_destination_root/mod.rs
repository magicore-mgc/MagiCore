//! Wires the prospective destination root owner implementation and nearest tests.
//! Nối implementation owner prospective destination root và test gần nhất.

#[path = "prospective_destination_root.rs"]
mod implementation;

pub(in crate::commands::store) use self::implementation::prospective_destination_root;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
