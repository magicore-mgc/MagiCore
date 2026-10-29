//! Shared `mgc dev` command entry point.
//! Điểm vào dùng chung của lệnh `mgc dev`.

mod dev;
mod forwarded_flags;

pub use dev::run;
pub use forwarded_flags::validate_forwarded_runtime_flags;
