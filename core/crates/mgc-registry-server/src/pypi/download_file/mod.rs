//! PyPI artifact download behavior owner.
//! Owner hành vi tải artifact PyPI.

mod download_file;
mod error;
pub(super) use download_file::download_file;

#[cfg(test)]
mod test;
