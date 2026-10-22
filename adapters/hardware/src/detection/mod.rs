//! Wires the shared Hardware project-marker detector.
//! Nối bộ nhận diện marker project Hardware dùng chung.

mod detection;
pub(crate) use detection::manifest_is_any_mg;

#[cfg(test)]
mod test;
