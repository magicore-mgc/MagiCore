// Registry/model mapping — preserve nested operation contracts.
// Mapping registry/model — giữ hợp đồng thao tác lồng nhau.
use super::super::common_to_dispatch;
use crate::commands::definitions::commands::common::CommonCommands;
use crate::commands::model::ModelCmd;
use crate::commands::registry::{RegistryCmd, UserCmd};
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_registry_user_management_payload() {
    match common_to_dispatch(CommonCommands::Registry {
        cmd: RegistryCmd::User {
            cmd: UserCmd::Add {
                name: "ci-publisher".to_owned(),
                password_stdin: true,
                scopes: vec!["@example/*".to_owned()],
                role: "publisher".to_owned(),
                registry: "https://registry.example".to_owned(),
            },
        },
    }) {
        Some(CommonCommand::Registry {
            cmd:
                RegistryCmd::User {
                    cmd:
                        UserCmd::Add {
                            name,
                            password_stdin,
                            scopes,
                            role,
                            registry,
                        },
                },
        }) => {
            assert_eq!(name, "ci-publisher");
            assert!(password_stdin);
            assert_eq!(scopes, vec!["@example/*".to_owned()]);
            assert_eq!(role, "publisher");
            assert_eq!(registry, "https://registry.example");
        }
        _ => panic!("registry command or payload was changed"),
    }
}

#[test]
fn maps_model_push_payload() {
    match common_to_dispatch(CommonCommands::Model {
        cmd: ModelCmd::Push {
            paths: vec!["models/model.gguf".to_owned()],
            repo: "ai/sample".to_owned(),
            tag: "v2".to_owned(),
            registry: "https://registry.example".to_owned(),
            token: Some("model-token".to_owned()),
        },
    }) {
        Some(CommonCommand::Model {
            cmd:
                ModelCmd::Push {
                    paths,
                    repo,
                    tag,
                    registry,
                    token,
                },
        }) => {
            assert_eq!(paths, vec!["models/model.gguf".to_owned()]);
            assert_eq!(repo, "ai/sample");
            assert_eq!(tag, "v2");
            assert_eq!(registry, "https://registry.example");
            assert_eq!(token.as_deref(), Some("model-token"));
        }
        _ => panic!("model command or payload was changed"),
    }
}
