//! Flutter SDK dependency overlays for App package operations.
//! Owner dependency ngầm từ SDK Flutter cho thao tác package App.

mod flutter_sdk_dependencies;

pub(super) use flutter_sdk_dependencies::{lock_replay_manifest, resolves_implicit_dependencies};
