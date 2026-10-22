// source_entries owner facade — mặt nối owner source_entries

mod source_entries;
pub(super) use source_entries::hash_source_entries;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
