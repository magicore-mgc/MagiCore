//! Wire the Java detector and its nearest tests.
//! Nối detector Java với test gần owner nhất.

#[path = "java.rs"]
mod java;

pub use java::java_step;

#[cfg(test)]
#[path = "test/java.rs"]
mod tests;
