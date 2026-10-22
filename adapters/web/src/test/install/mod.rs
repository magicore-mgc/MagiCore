// install_tests — split from unit_tests.rs by behavior/subject (P07).
// install_tests — tách từ unit_tests.rs theo behavior/chủ đề (P07).
use super::*;


#[path = "mutation_policy.rs"]
mod mutation_policy;
#[path = "package_materialization.rs"]
mod package_materialization;
#[path = "binary_and_dependency_layout.rs"]
mod binary_and_dependency_layout;
#[path = "install_transaction.rs"]
mod install_transaction;
#[path = "registry_cache_fallback.rs"]
mod registry_cache_fallback;
#[path = "shared_store_links.rs"]
mod shared_store_links;
#[path = "extracted_root_recovery.rs"]
mod extracted_root_recovery;
