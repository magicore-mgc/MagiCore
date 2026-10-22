//! Wires dependency-spec parsing and nearest-owner tests.
//! Nối parse dependency spec và test gần owner.

mod dependency_spec;
pub use self::dependency_spec::DependencySpec;
#[cfg(test)]
mod test;
