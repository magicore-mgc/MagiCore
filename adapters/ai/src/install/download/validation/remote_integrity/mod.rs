//! Remote checksum and HuggingFace revision requirements.
//! Yêu cầu checksum nguồn mạng và revision HuggingFace bất biến.

mod remote_integrity;
pub(in crate::install) use remote_integrity::validate_remote_integrity;

#[cfg(test)]
mod test;
