#![allow(clippy::unwrap_used)]
//! Registry server tests
//! (Tests: auth, model serialization, storage init, npm route matching — per RULE §5)

use mgc_registry_server::auth::AuthService;
use mgc_registry_server::model::Package;
use mgc_registry_server::storage::RegistryStore;
use std::collections::HashMap;

#[path = "registry/mod.rs"]
mod registry;
