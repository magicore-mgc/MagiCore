//! PyPI route compatibility facade.
//! Facade tương thích cho các route PyPI.

mod download_file;
mod routes;
mod simple_index;
mod upload_legacy;

pub use routes::routes;
