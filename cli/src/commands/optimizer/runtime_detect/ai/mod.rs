//! AI runtime owner exports and nearest tests.
mod ai;
pub(in crate::commands::optimizer) use ai::detect_ai_runtime;

#[cfg(test)]
#[path = "test/ai.rs"]
mod tests;
