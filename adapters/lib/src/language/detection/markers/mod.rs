//! Lib language marker detector module wiring.
//! Wiring module nhận diện marker ngôn ngữ Lib.

mod markers;

pub(super) use markers::detect_marker_language;
