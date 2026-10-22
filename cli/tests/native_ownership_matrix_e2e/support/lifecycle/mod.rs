//! Native lifecycle helper exports. / Export helper lifecycle native.

mod add_install;
mod remove;
mod update;

pub(crate) use add_install::{
    blocked_mutation_unchanged, native_add_cell_with_env, native_install_cell_with_env,
};
pub(crate) use remove::{
    native_remove_cell, native_remove_cell_with_env, native_remove_only,
    native_remove_only_with_env,
};
pub(crate) use update::{
    native_update_cell_with_env, outdated_ai, outdated_bevy, outdated_dotnet, outdated_esp32,
    outdated_flutter, outdated_go, outdated_java, outdated_python, outdated_rust,
};
