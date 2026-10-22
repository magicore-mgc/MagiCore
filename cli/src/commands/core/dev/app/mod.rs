mod android_device;
mod app;
mod command_builder;
mod dispatch;
mod ios_dev;
mod ios_simulator;
mod platform;
mod runtime_config;
mod swift_package;
mod tool_execution;

pub use app::dev;
pub use ios_simulator::{find_ios_simulator, find_ios_simulator_in};
pub use platform::{TargetPlatform, detect_target_platform};
pub(crate) use tool_execution::run_tool_with_env;

pub(super) use android_device::{android_emulator_running, run_android_device_command};
pub(super) use command_builder::{flutter_dev_command, kotlin_dev_command};
pub(super) use ios_dev::dev_ios;
pub(super) use ios_simulator::{
    boot_simulator_in, find_xcode_project, xcode_available, xcode_available_in,
};
pub(super) use runtime_config::{detect_app_runtime, dev_scheme, read_app_id};
pub(super) use swift_package::run_swift_package;
