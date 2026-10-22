//! Shared data required by every multi-platform writer.
//! Dữ liệu dùng chung mà từng writer đa nền tảng cần.

use std::path::Path;

use crate::scaffold::processors::slugify;

pub(in crate::scaffold::processors::app::implementation) struct MultiProjectContext<'a> {
    pub(in crate::scaffold::processors::app::implementation::multi) target: &'a Path,
    pub(in crate::scaffold::processors::app::implementation::multi) name: &'a str,
    pub(in crate::scaffold::processors::app::implementation::multi) slug: String,
    pub(in crate::scaffold::processors::app::implementation::multi) package: String,
}

impl<'a> MultiProjectContext<'a> {
    pub(in crate::scaffold::processors::app::implementation::multi) fn new(
        target: &'a Path,
        name: &'a str,
    ) -> Self {
        let slug = slugify(name);
        let package = slug.replace('-', "_");
        Self {
            target,
            name,
            slug,
            package,
        }
    }
}
