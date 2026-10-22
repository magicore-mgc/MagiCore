//! Read-only parsing of language-specific dependency manifests.
//! Parse chỉ đọc manifest dependency theo ngôn ngữ.

mod manifest_parse;

pub(super) use manifest_parse::parse_manifest;
