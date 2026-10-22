//! Wires version-range behavior and private matching owners.
//! Nối hành vi version range và các owner matching private.

mod match_compound_range;
mod match_single_range;
mod parse_hyphen_range;
mod parse_wildcard_bounds;
mod version_range;
mod wildcard_satisfying_version;

use self::match_compound_range::match_compound_range;
use self::match_single_range::match_single_range;
use self::parse_hyphen_range::parse_hyphen_range;
use self::parse_wildcard_bounds::parse_wildcard_bounds;
pub use self::version_range::VersionRange;
use self::wildcard_satisfying_version::wildcard_satisfying_version;
#[cfg(test)]
mod test;
