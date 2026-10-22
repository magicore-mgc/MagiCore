//! Materializer capability module wiring.
//! Wiring module capability materializer.

mod materializer;
pub use self::materializer::Materializer;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
