use super::super::types::Incompatibility;

#[derive(Debug, Clone, thiserror::Error)]
pub enum SolveError {
    // Boxed: the incompatibility graph dwarfs the unit variant, and the
    // Version payload grew for PEP 440 post/dev segments — keep the enum
    // under the workspace large-enum-variant gate without weakening it.
    #[error("unsatisfiable constraints")]
    Unsatisfiable(Box<Incompatibility>),
    #[error("no package found to satisfy constraints")]
    NoPackageFound,
}

impl From<Incompatibility> for SolveError {
    fn from(inc: Incompatibility) -> Self {
        SolveError::Unsatisfiable(Box::new(inc))
    }
}
