// Recursive workspace wiring — export its coordinator and register focused owners.
// Wiring recursive workspace — export coordinator và đăng ký owner theo trách nhiệm.
mod execution_plan;
mod recursive;
#[cfg(test)]
mod test;
mod workspace_filter;
mod workspace_order;

pub(in crate::dispatch) use recursive::run_recursive;
