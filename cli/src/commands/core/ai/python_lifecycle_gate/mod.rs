//! Native-Python lifecycle gate owner wiring.
//! Wiring owner gate lifecycle Python native.

mod python_lifecycle_gate;

#[cfg(feature = "ai")]
pub use python_lifecycle_gate::require_native_ai_python;
