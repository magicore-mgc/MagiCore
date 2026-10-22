//! Typed, URL-safe failures for Hugging Face HTTP operations.
//! Định nghĩa lỗi có kiểu và đã gỡ URL cho thao tác HTTP Hugging Face.

use mgc_types::MgError;

/// Hugging Face API operation failures / Lỗi operation API Hugging Face.
#[derive(Debug, thiserror::Error)]
pub(in crate::native::hf_client) enum HuggingFaceClientError {
    /// Request failed before an HTTP response was received.
    /// Request lỗi trước khi nhận HTTP response.
    #[error("{operation} failed: {source}")]
    Request {
        /// Safe operation label for a diagnostic.
        /// Nhãn operation an toàn để đưa vào diagnostic.
        operation: &'static str,
        /// URL-stripped request cause, retained for error chaining.
        /// Nguyên nhân request đã gỡ URL và vẫn giữ trong error chain.
        #[source]
        source: reqwest::Error,
    },
    /// The endpoint returned an unsuccessful HTTP status.
    /// Endpoint trả về HTTP status không thành công.
    #[error("{operation} HTTP {status}")]
    HttpStatus {
        /// Safe operation label for a diagnostic.
        /// Nhãn operation an toàn để đưa vào diagnostic.
        operation: &'static str,
        /// The response status code.
        /// Mã status của response.
        status: reqwest::StatusCode,
    },
    /// Response JSON could not be decoded.
    /// Không thể giải mã JSON của response.
    #[error("HuggingFace JSON parse failed: {source}")]
    JsonParse {
        /// URL-stripped decoding cause, retained for error chaining.
        /// Nguyên nhân giải mã đã gỡ URL và vẫn giữ trong error chain.
        #[source]
        source: reqwest::Error,
    },
    /// The caller supplied an empty model identifier.
    /// Caller truyền model identifier rỗng.
    #[error("model id is empty")]
    EmptyModelId,
}

impl HuggingFaceClientError {
    pub(in crate::native::hf_client) fn request(
        operation: &'static str,
        source: reqwest::Error,
    ) -> Self {
        Self::Request {
            operation,
            source: source.without_url(),
        }
    }

    pub(in crate::native::hf_client) fn http_status(
        operation: &'static str,
        status: reqwest::StatusCode,
    ) -> Self {
        Self::HttpStatus { operation, status }
    }

    pub(in crate::native::hf_client) fn json_parse(source: reqwest::Error) -> Self {
        Self::JsonParse {
            source: source.without_url(),
        }
    }

    #[cfg(test)]
    pub(super) fn code(&self) -> &'static str {
        match self {
            Self::Request { .. } => "MGC_AI_HF_REQUEST",
            Self::HttpStatus { .. } => "MGC_AI_HF_HTTP_STATUS",
            Self::JsonParse { .. } => "MGC_AI_HF_JSON",
            Self::EmptyModelId => "MGC_AI_HF_INVALID_MODEL_ID",
        }
    }

    #[cfg(test)]
    pub(super) fn operation(&self) -> &'static str {
        match self {
            Self::Request { operation, .. } | Self::HttpStatus { operation, .. } => operation,
            Self::JsonParse { .. } => "HuggingFace JSON parse",
            Self::EmptyModelId => "get_model_info",
        }
    }

    #[cfg(test)]
    pub(super) fn is_retryable(&self) -> bool {
        match self {
            Self::Request { source, .. } => source.is_timeout() || source.is_connect(),
            Self::HttpStatus { status, .. } => {
                matches!(
                    *status,
                    reqwest::StatusCode::REQUEST_TIMEOUT | reqwest::StatusCode::TOO_MANY_REQUESTS
                ) || status.is_server_error()
            }
            Self::JsonParse { .. } | Self::EmptyModelId => false,
        }
    }
}

impl From<HuggingFaceClientError> for MgError {
    fn from(error: HuggingFaceClientError) -> Self {
        let is_network = matches!(
            &error,
            HuggingFaceClientError::Request { .. } | HuggingFaceClientError::HttpStatus { .. }
        );
        let message = error.to_string();
        if is_network {
            Self::Network(message)
        } else {
            Self::Other(message)
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
