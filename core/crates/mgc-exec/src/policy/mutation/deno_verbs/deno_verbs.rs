pub(in crate::policy) const DENO_MUTATION_VERBS: &[&str] =
    &["add", "remove", "install", "publish", "upgrade", "update"];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
