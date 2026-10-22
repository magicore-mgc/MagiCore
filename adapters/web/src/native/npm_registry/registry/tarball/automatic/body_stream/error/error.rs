// Typed failures from response-body consumption. / Lỗi có kiểu khi đọc body response.
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum BodyStreamError {
    #[error("registry response stream failed")]
    ResponseStream(#[source] anyhow::Error),
    #[error("failed to create tarball staging file")]
    StagingCreate(#[source] io::Error),
    #[error("failed to write tarball staging file")]
    StagingWrite(#[source] io::Error),
    #[error("failed to flush tarball staging file")]
    StagingFlush(#[source] io::Error),
    #[error("tarball staging file is not open")]
    StagingNotOpen,
    #[error("registry response body length exceeds the supported range")]
    LengthOverflow,
}
