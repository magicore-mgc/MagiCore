//! Wires package pin data and its nearest contract tests.
//! Nối dữ liệu package pin và test contract gần nhất.

mod package;
pub use self::package::Package;
#[cfg(test)]
mod test;
