//! CI and automation diagnostics.
//! Lỗi CI và automation.

mod error;

pub use error::{
    ci_template_unknown, cicd_deploy_target_not_implemented, cicd_project_not_detected,
    cicd_reason, cicd_test_step_failed, cicd_verb_not_applicable, cicd_verify_empty_chain,
    cicd_verify_unknown_step, cloudflare_build_in_cicd_core, lib_language_without_initializer,
};
