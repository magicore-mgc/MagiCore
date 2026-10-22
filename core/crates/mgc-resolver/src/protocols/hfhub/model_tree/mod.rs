//! Registry tree JSON to locked file conversion.
//! Chuyển JSON tree của registry thành file đã lock.

mod error;
mod model_tree;

pub(super) use model_tree::parse_model_tree;

#[cfg(test)]
mod test;
