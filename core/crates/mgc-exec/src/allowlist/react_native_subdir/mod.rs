#[path = "react_native_subdir.rs"]
mod react_native_subdir;

pub(super) use react_native_subdir::is_react_native_subdir;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
