// generated_project_files owner wiring — nối owner generated_project_files.
// Keep the implementation and re-export boundary small.
use super::*;
#[path = "generated_project_files.rs"]
mod implementation;
pub(in crate::scaffold) use implementation::slugify;
pub(in crate::scaffold::processor) use implementation::{
    common_gitignore, common_readme, config_feature_set, extract_template_tokens, quoted_list,
};
