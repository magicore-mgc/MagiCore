//! Flutter outdated-output parser fixture owner wiring.
//! File này chỉ export parser regression helper.

mod outdated_parser;

pub(crate) use outdated_parser::parse_flutter_outdated_json;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
