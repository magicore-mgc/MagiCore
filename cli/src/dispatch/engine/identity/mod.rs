// Identity owner wiring — export workspace and explicit-command identity helpers.
// Wiring owner identity — export helper nhận diện workspace và command tường minh.
mod identity;
#[cfg(test)]
mod test;

pub(super) use identity::{
    explicit_dependency_core, resolve_recursive_workspace_core, validate_explicit_dependency_core,
    workspace_package_name,
};
