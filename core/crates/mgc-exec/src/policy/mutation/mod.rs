mod composer_verbs;
mod dart_verbs;
mod deno_verbs;
mod first_subcommand;
mod javascript_verbs;
mod mutation;
mod python_verbs;

pub(super) use composer_verbs::COMPOSER_MUTATION_VERBS;
pub(super) use dart_verbs::DART_MUTATION_VERBS;
pub(super) use deno_verbs::DENO_MUTATION_VERBS;
pub(super) use first_subcommand::first_subcommand as parse_first_subcommand;
pub(super) use javascript_verbs::JAVASCRIPT_MUTATION_VERBS;
pub use mutation::is_mutation_invocation;
pub(super) use python_verbs::PYTHON_MUTATION_VERBS;
