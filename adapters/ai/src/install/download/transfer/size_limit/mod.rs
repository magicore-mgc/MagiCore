//! Remote byte-limit owner wiring.
//! Wiring owner giới hạn byte remote.

mod size_limit;
pub(in crate::install) use size_limit::next_model_download_total;
pub(in crate::install::download) use size_limit::{
    MAX_REMOTE_MODEL_BYTES_ENV, max_remote_model_bytes,
};

#[cfg(test)]
mod test;
