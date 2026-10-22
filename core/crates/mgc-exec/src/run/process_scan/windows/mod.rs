//! Windows process scan facade — export identity scanning and retry.
//! Facade quét process Windows — export quét identity và retry.

#[path = "windows.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::{
    WindowsProcessScan, WindowsProcessSnapshot,
};
