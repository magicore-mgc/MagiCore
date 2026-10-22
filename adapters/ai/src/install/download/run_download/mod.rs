//! Model download orchestration wiring.
//! Wiring điều phối download model.

mod download_model;
mod types;
pub use download_model::download_model;

#[cfg(test)]
mod test;
