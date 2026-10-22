//! Transfer owner wiring.
//! Wiring owner truyền dữ liệu.

mod http_stream;
mod local_copy;
mod size_limit;
pub(in crate::install::download) use http_stream::http_download_to_staging;
#[cfg(test)]
pub(in crate::install) use http_stream::http_download_to_staging_with_limit;
pub(super) use local_copy::copy_local_to_staging;
pub(in crate::install) use size_limit::next_model_download_total;
pub(in crate::install::download) use size_limit::{
    MAX_REMOTE_MODEL_BYTES_ENV, max_remote_model_bytes,
};
