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
                "Could not find entry point. Checked: src/index.ts, src/index.tsx, src/main.ts, src/main.tsx, and package.json main/module fields",
            ),
        }
    }
}

impl std::error::Error for EntryPointError {}
