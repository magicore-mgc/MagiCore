//! Package Projection owner wiring.
//! Nối owner package projection.

mod package_projection;

#[cfg(test)]
mod test;

pub use package_projection::NativeResolution;

pub(in crate::native::engine::engine) use package_projection::project_packages;
