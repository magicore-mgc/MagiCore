//! PyPI legacy multipart upload owner.
//! Owner upload multipart legacy PyPI.

mod error;
mod upload_legacy;
pub(super) use upload_legacy::upload_legacy;

#[cfg(test)]
mod test;
