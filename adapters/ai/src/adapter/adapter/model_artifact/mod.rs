//! Wires the model-artifact audit owner.
//! Wiring owner audit model-artifact.

mod error;
mod model_artifact;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use model_artifact::model_artifact_report;
#[cfg(test)]
pub(super) use model_artifact::{model_artifact_file_label, model_artifact_package_name};
