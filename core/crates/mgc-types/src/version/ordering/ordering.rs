//! Implements total ordering from release precedence and build metadata.
//! Cài đặt total ordering từ precedence release và build metadata.

use super::super::Version;
use std::cmp::Ordering;

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.precedence_cmp(other)
            .then_with(|| self.build.cmp(&other.build))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
