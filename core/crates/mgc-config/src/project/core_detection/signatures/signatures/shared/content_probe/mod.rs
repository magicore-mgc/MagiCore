//! Content probe owner.
//! Owner probe nội dung.

mod content_probe;

pub(in crate::project::project::core_detection::signatures::signatures) use content_probe::{
    files_with_extension, has_named_dependency, has_regular_extension_file, text_contains_any,
};
