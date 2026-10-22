//! Ecosystem identifier exports and nearest tests.
//! Export định danh ecosystem và test gần owner.

mod ecosystem;

pub use self::ecosystem::Ecosystem;

#[cfg(test)]
#[path = "test/ecosystem.rs"]
mod tests;
