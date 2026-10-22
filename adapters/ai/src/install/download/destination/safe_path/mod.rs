//! Safe model destination path wiring.
//! Wiring đường dẫn destination model an toàn.

mod safe_path;
pub(in crate::install::download) use safe_path::ensure_safe_model_destination;
