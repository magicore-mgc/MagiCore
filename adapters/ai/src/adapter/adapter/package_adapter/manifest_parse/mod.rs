//! AI manifest-parser owner wiring.
//! Wiring owner parser manifest AI.

mod manifest_parse;

pub(in crate::adapter::implementation) use manifest_parse::parse_manifest_for_adapter;
