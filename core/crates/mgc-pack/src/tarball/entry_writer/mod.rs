//! Tar entry writer exports — export bộ ghi tar entry.

mod entry_writer;

pub(super) use entry_writer::append_entries;

#[cfg(test)]
#[path = "test/entry_writer.rs"]
mod tests;
