mod build_step;
pub(super) use build_step::ensure_build_step_supported;

#[cfg(test)]
#[path = "test/build_step.rs"]
mod tests;
