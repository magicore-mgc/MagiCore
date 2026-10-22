// path_key owner facade — mặt nối owner path_key

mod path_key;
pub(super) use path_key::source_path_key;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
