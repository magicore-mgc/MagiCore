//! Registry candidate key owner / owner khóa tra cứu registry.

mod registry_candidates;
#[cfg(test)]
mod test;

pub(in crate::auth::resolve_auth) use registry_candidates::registry_candidates;
