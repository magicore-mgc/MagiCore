//! Wires the python native install owner.
//! Wiring owner python native install.

pub(super) mod error;
mod python_native_install;

pub(crate) use python_native_install::install_python_native;
#[cfg(test)]
pub(crate) use python_native_install::{
    read_cached_python_artifact, validate_python_importable_graph,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
