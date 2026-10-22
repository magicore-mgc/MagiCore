//! Bounded HTTP streaming owner wiring.
//! Wiring owner HTTP streaming có giới hạn.

mod http_stream;
pub(in crate::install::download) use http_stream::http_download_to_staging;
#[cfg(test)]
pub(in crate::install) use http_stream::http_download_to_staging_with_limit;

#[cfg(test)]
mod test;
