//! Typed failures owned by Web entry-point discovery.
//! Lỗi typed thuộc khâu tìm entry point Web.

use std::fmt;

#[derive(Debug)]
pub(super) enum EntryPointError {
    NotFound,
}

impl fmt::Display for EntryPointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => formatter.write_str(
                "Could not find entry point. Checked conventional browser entries (src/index.*, src/main.*, index.*, main.*), package.json main/module/bin fields, and conventional server entries (server.js, app.js, index.js, src/server.js)",
            ),
        }
    }
}

impl std::error::Error for EntryPointError {}
