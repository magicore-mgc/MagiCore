pub(in crate::policy) const COMPOSER_MUTATION_VERBS: &[&str] =
    &["install", "update", "require", "remove"];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
