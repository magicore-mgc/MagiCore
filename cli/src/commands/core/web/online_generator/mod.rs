//! Web online-generator owner wiring and stable internal exports.
//! Wiring owner initializer Web online và export nội bộ ổn định.

mod backend;
mod frontend;
mod online_generator;
mod types;

pub(crate) use online_generator::online_web_generator;
pub(crate) use types::{OnlineGeneratorArgument, OnlineWebGenerator};
