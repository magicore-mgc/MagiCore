mod error;
mod index;

pub(super) use error::{IndexError, IndexResult};
pub(super) use index::{download_url_for, index_entries, index_url_for, select_index_entry};
