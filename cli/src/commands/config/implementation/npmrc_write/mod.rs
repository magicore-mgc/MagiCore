mod npmrc_write;
#[cfg(test)]
pub(super) use npmrc_write::merge_file;
pub(in crate::commands::config) use npmrc_write::{delete_npmrc, set_npmrc};

#[cfg(test)]
mod test;
