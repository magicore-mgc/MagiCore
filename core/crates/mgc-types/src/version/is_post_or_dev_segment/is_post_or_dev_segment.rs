//! Recognizes the post/dev suffix shape accepted by the parser.
//! Nhận diện shape hậu tố post/dev được parser chấp nhận.

pub(in crate::version) fn is_post_or_dev_segment(segment: &str) -> bool {
    let after = if let Some(after) = segment.strip_prefix("post") {
        after
    } else if let Some(after) = segment.strip_prefix("dev") {
        after
    } else {
        return false;
    };
    after.is_empty() || after.bytes().all(|b| b.is_ascii_digit())
}
