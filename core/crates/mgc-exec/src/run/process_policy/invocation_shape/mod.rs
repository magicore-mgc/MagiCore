//! Invocation-shape owner exports.
//! Export owner hình dạng invocation.

#[path = "invocation_shape.rs"]
mod implementation;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

#[cfg(test)]
pub(in crate::run::implementation) use implementation::matched_invocation_shape;
pub(in crate::run::implementation) use implementation::matched_invocation_shape_argv;
