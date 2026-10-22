/// Resolution failure: conflict or provider error.
#[derive(Debug, Clone)]
pub struct SolveError {
    pub message: String,
}

impl std::fmt::Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for SolveError {}
impl From<String> for SolveError {
    fn from(s: String) -> Self {
        Self { message: s }
    }
}
