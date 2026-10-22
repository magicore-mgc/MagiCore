/// Error from a `DependencyProvider` (network failure, registry 500, etc.).
#[derive(Debug, Clone)]
pub struct DependencyError(pub String);

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for DependencyError {}
