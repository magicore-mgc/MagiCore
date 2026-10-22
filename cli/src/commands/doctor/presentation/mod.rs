//! Human and machine-readable doctor output.
//! Output doctor cho người dùng và máy đọc.

mod presentation;

pub(super) use presentation::{render_human, render_json};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
