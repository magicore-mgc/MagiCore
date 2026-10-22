//! Model and immutable revision validation owner.
//! Owner kiểm tra model và revision bất biến.

mod error;
mod model_identity;

pub(super) use model_identity::{check_model_id, check_revision};

#[cfg(test)]
mod test;
