//! Typed AI scaffold output error wrapper.
//! Wrapper lỗi typed cho output AI scaffold.

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(in crate::scaffold::processors::ai::implementation) struct AiScaffoldError(
    #[from] anyhow::Error,
);

pub(in crate::scaffold::processors::ai::implementation) type AiScaffoldResult<T> =
    std::result::Result<T, AiScaffoldError>;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
