//! Staging path owner wiring.
//! Wiring owner đường dẫn staging.

mod staging;
pub(in crate::install) use staging::filename_from_url;
pub(in crate::install::download) use staging::unique_staging_path;

#[cfg(test)]
mod test;
