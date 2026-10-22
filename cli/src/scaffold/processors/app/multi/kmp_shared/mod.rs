//! KMP shared output wiring.
//! Wiring output KMP dùng chung.

#[path = "kmp_shared.rs"]
mod implementation;
pub(in crate::scaffold::processors::app::implementation) use implementation::write;
