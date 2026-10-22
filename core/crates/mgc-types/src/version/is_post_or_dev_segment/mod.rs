//! Wires the `is_post_or_dev_segment` owner and its nearest tests.
//! Nối owner `is_post_or_dev_segment` và test gần nhất.

mod is_post_or_dev_segment;
pub(super) use self::is_post_or_dev_segment::is_post_or_dev_segment;
#[cfg(test)]
mod test;
