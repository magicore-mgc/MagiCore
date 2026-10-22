//! Web layer resolver owner wiring.
//! Nối owner phân giải Web layer.

#[path = "web_template_layers.rs"]
mod implementation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
