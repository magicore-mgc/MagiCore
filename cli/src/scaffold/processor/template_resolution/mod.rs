//! Wire template-resolution owners and preserve the processor-facing facade.
//! Nối các owner template-resolution và giữ facade cho processor.

mod layer_readiness;
mod monorepo_layer_stack;
mod template_resolution;
mod web_template_layers;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
