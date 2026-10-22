//! Flutter executable PATH lookup wiring.
//! Wiring dò executable Flutter trong PATH.

mod path_lookup;
pub(crate) use path_lookup::find_flutter_executable_in_path;
