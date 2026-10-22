//! Classify local advisory-query errors and preserve shared error semantics.
//! Phân loại lỗi advisory-query cục bộ và giữ nguyên ngữ nghĩa lỗi chung.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::audit) enum AdvisoryQueryError {
    #[error("audit client error: {0}")]
    ClientBuild(#[source] reqwest::Error),
    #[error(transparent)]
    Endpoint(MgError),
    #[error("audit response parse error: {0}")]
    ResponseJson(#[source] reqwest::Error),
    #[error(transparent)]
    AdvisoryParse(MgError),
}

impl From<AdvisoryQueryError> for MgError {
    fn from(error: AdvisoryQueryError) -> Self {
        match error {
            AdvisoryQueryError::ClientBuild(error) => {
                Self::Network(format!("audit client error: {error}"))
            }
            AdvisoryQueryError::Endpoint(error) | AdvisoryQueryError::AdvisoryParse(error) => error,
            AdvisoryQueryError::ResponseJson(error) => {
                Self::Other(format!("audit response parse error: {error}"))
            }
        }
    }
}
