//! Test project and user npmrc target selection.
//! Kiểm tra chọn đích npmrc của project và user.
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::{project_npmrc_path, user_npmrc_path};
use crate::commands::login::token_storage::TokenStorageError;

#[test]
fn project_token_target_is_local_npmrc() {
    assert_eq!(
        project_npmrc_path(PathBuf::from("/workspace/project")),
        PathBuf::from("/workspace/project/.npmrc")
    );
}

#[test]
fn user_token_target_is_home_npmrc() {
    assert_eq!(
        user_npmrc_path(Some(PathBuf::from("/home/alice"))).unwrap(),
        PathBuf::from("/home/alice/.npmrc")
    );
}

#[test]
fn user_token_target_requires_home_directory() {
    let error = user_npmrc_path(None).unwrap_err();
    assert!(matches!(&error, TokenStorageError::NoHomeDirectory));
    assert_eq!(error.to_string(), "No home directory");
}

#[test]
fn persistence_errors_keep_the_original_anyhow_chain() {
    use anyhow::Context;

    let original = anyhow::anyhow!("write failed").context("persist token");
    let expected = format!("{original:#}");
    let actual = super::super::token_storage_diagnostic(TokenStorageError::Persistence(original));
    assert_eq!(format!("{actual:#}"), expected);
}
