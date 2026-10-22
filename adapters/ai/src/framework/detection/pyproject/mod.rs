//! Pyproject AI framework reader wiring.
//! Wiring module reader framework từ pyproject.

mod pyproject;

pub(in crate::framework) use pyproject::framework_from_pyproject;
