#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn project_values_override_user_values() {
    let mut user =
        NpmRc::parse("registry=https://user.example/\n@team:registry=https://user-team.example/\n")
            .unwrap();
    user.merge(
        NpmRc::parse(
            "registry=https://project.example/\n@team:registry=https://project-team.example/\n",
        )
        .unwrap(),
    );
    assert_eq!(user.registry.as_deref(), Some("https://project.example/"));
    assert_eq!(
        user.registry_for(Some("@team")).as_deref(),
        Some("https://project-team.example/")
    );
}
