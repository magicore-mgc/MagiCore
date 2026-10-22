//! Wires package pin provenance and its tests.
//! Nối provenance của package pin và test.

mod provenance;
pub use self::provenance::Provenance;
#[cfg(test)]
mod test;
