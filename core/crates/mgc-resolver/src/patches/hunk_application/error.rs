//! Typed errors raised while matching and replacing hunk line ranges.
//! Lỗi có kiểu khi đối chiếu và thay thế các dòng của hunk.

#[derive(Debug, thiserror::Error)]
pub(super) enum HunkApplicationError {
    #[error("hunk range overflows addressable file length")]
    RangeOverflow,
    #[error("hunk range exceeds file length: {end} > {actual}")]
    RangeExceeds { end: usize, actual: usize },
    #[error("patch context mismatch at line {line}: expected {expected:?}, got {actual:?}")]
    ContextMismatch {
        line: usize,
        expected: Vec<String>,
        actual: Vec<String>,
    },
    #[error("invalid hunk operation: {0}")]
    InvalidOperation(char),
}
