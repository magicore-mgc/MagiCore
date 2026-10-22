mod ios_simulator;
pub(in crate::commands::core::dev) use ios_simulator::{
    boot_simulator_in, find_xcode_project, xcode_available, xcode_available_in,
};
pub use ios_simulator::{find_ios_simulator, find_ios_simulator_in};
