// Web command profiling wiring — nối helper đo thời gian command Web.
// Profiling helpers stay internal to the implementation module.
#[path = "profiling.rs"]
mod implementation;
pub(super) use implementation::web_command_profile_mark;
