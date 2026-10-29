//! Wires the cache path segment validator and its nearest tests.
//! Nối validator segment path cache và test gần nhất.

mod cache_segment;
pub(crate) use cache_segment::validate_cache_segment;
#[cfg(test)]
mod test;
