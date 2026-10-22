//! NPM deprecation value deserialization owner.
//! Owner deserialize giá trị deprecated của NPM.

mod optional_deprecation;

pub(super) use optional_deprecation::de_optional_string;
