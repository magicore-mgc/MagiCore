pub(in crate::policy) const PYTHON_MUTATION_VERBS: &[&str] = &[
    "install",
    "add",
    "remove",
    "uninstall",
    "update",
    "upgrade",
    "sync",
    "lock",
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
