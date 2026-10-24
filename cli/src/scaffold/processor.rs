// Scaffold processor facade — API scaffold và wiring của các owner.
// This file preserves the public Scaffolder path and delegates implementation.
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::scaffold::template_root::TemplateRoot;
use crate::wizard::engine::ScaffoldConfig;

pub struct Scaffolder;

#[path = "processor/framework_catalog/mod.rs"]
mod framework_catalog;
use framework_catalog::*;
#[path = "processor/online_generator_output/mod.rs"]
mod online_generator_output;
mod python_dev_group;
pub(in crate::scaffold::processor) use python_dev_group::ensure_python_dev_group;
#[path = "processor/scaffold_lifecycle/mod.rs"]
mod scaffold_lifecycle;
#[path = "processor/scaffold_writers/mod.rs"]
mod scaffold_writers;
#[path = "processor/template_materialization/mod.rs"]
mod template_materialization;
#[path = "processor/template_resolution/mod.rs"]
mod template_resolution;
use template_materialization::*;
#[path = "processor/template_context/mod.rs"]
mod template_context;
use template_context::*;
#[path = "processor/template_manifest/mod.rs"]
mod template_manifest;
use template_manifest::*;
#[path = "processor/generated_project_files/mod.rs"]
mod generated_project_files;
use generated_project_files::*;
#[path = "processor/framework_mapping/mod.rs"]
mod framework_mapping;
use framework_mapping::{
    effective_web_mode, framework_uses_react_shell, fullstack_backend_framework,
    fullstack_frontend_framework, fullstack_uses_frontend_foundation, fullstack_uses_react_shell,
    legacy_template_target, normalize_web_framework,
};

pub(in crate::scaffold) use crate::scaffold::hardware_marker::write as write_hardware_marker;
pub use framework_mapping::{infer_backend_language, is_all_in_one_fullstack};
pub(in crate::scaffold) use generated_project_files::slugify;
pub(crate) use template_manifest::validate_template_manifest;

#[cfg(test)]
#[path = "processor/test/processor.rs"]
mod tests;
