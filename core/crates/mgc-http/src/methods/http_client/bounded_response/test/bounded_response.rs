//! Response byte-limit tests.
//! Test giới hạn byte của response.

use super::*;

#[test]
fn appends_chunks_up_to_the_exact_limit() {
    let mut body = b"ab".to_vec();
    append_response_chunk(&mut body, b"cd", 4).expect("chunk fits at the limit");
    assert_eq!(body, b"abcd");
}

#[test]
fn rejects_an_over_limit_chunk_without_appending_it() {
    let mut body = b"ab".to_vec();
    let error = append_response_chunk(&mut body, b"cd", 3).expect_err("chunk exceeds limit");
    assert_eq!(body, b"ab");
    assert_eq!(error.to_string(), "HTTP response exceeds byte limit");
}

#[test]
fn rejects_an_already_oversized_buffer_without_subtraction_underflow() {
    let mut body = b"abcd".to_vec();
    let error = append_response_chunk(&mut body, b"", 3).expect_err("buffer already exceeds limit");
    assert_eq!(body, b"abcd");
    assert_eq!(error.to_string(), "HTTP response exceeds byte limit");
}
