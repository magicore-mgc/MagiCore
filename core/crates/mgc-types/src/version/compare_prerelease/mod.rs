//! Wires the `compare_prerelease` owner and its nearest tests.
//! Nối owner `compare_prerelease` và test gần nhất.

mod compare_prerelease;
pub(super) use self::compare_prerelease::compare_prerelease;
#[cfg(test)]
mod test;
