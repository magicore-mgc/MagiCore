//! Web framework catalog owner wiring.
//! Wiring owner catalog framework Web.

use super::*;

mod backend;
mod frontend;
mod fullstack;
#[path = "web_framework_catalog.rs"]
mod implementation;
#[cfg(test)]
mod test;
mod types;

pub(in crate::scaffold::processor) use implementation::{WEB_FRAMEWORKS, infer_web_create_config};
pub(in crate::scaffold::processor) use types::WebFrameworkConfig;
