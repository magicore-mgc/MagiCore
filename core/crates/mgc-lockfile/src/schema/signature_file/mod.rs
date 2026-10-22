//! Wires signature sidecar parsing, rendering and tests.
//! Nối parse, render signature sidecar và test.

mod signature_file;
pub use self::signature_file::SignatureFile;
#[cfg(test)]
mod test;
