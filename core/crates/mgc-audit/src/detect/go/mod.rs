//! Wire the Go detector and its nearest tests.
//! Nối detector Go với test gần owner nhất.

#[path = "go.rs"]
mod go;

pub use go::go_step;

#[cfg(test)]
#[path = "test/go.rs"]
mod tests;
