//! Compares release precedence while excluding build metadata.
//! So sánh precedence release và bỏ qua build metadata.

use super::super::Version;
use super::super::compare_prerelease::compare_prerelease;
use std::cmp::Ordering;

impl Version {
    pub fn precedence_cmp(&self, other: &Self) -> Ordering {
        match (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch)) {
            Ordering::Equal => match (self.dev, other.dev) {
                (Some(left), Some(right)) => match left.cmp(&right) {
                    Ordering::Equal => {
                        match compare_prerelease(self.pre.as_deref(), other.pre.as_deref()) {
                            Ordering::Equal => self.post.cmp(&other.post),
                            ordering => ordering,
                        }
                    }
                    ordering => ordering,
                },
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => match compare_prerelease(self.pre.as_deref(), other.pre.as_deref())
                {
                    Ordering::Equal => self.post.cmp(&other.post),
                    ordering => ordering,
                },
            },
            ordering => ordering,
        }
    }
}
