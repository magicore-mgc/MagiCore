pub(in crate::policy) const JAVASCRIPT_MUTATION_VERBS: &[&str] = &[
    "install",
    "ci",
    "add",
    "remove",
    "uninstall",
    "update",
    "upgrade",
    "publish",
    "unpublish",
    "link",
    "unlink",
    "dedupe",
    "prune",
    "rebuild",
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
