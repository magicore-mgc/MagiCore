//! GPU probe owner exports.
//! (Xuất API nội bộ cho owner probe GPU.)
mod gpu;
mod linux;
pub(in crate::commands::optimizer) use gpu::detect_gpus;
