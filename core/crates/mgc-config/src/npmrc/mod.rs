//! npmrc parsing and credential facade / Facade parse npmrc và credential.
mod auth_token_write;
mod credential_scope;
mod load;
mod lookup;
mod parse;
mod types;
pub use load::user_registry_only;
pub use types::{NpmCredential, NpmRc};
