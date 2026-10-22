//! Shared package-entry construction for App install lanes.
//! Tạo package-entry dùng chung cho các lane install App.
mod package_entry;
pub(crate) use package_entry::entry_from_package;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
