// Audit-strict module wiring — expose the policy through the established engine path.
// Wiring này export policy audit-strict qua đường dẫn engine đang dùng.
mod audit_policy;
#[cfg(test)]
mod test;

pub(in crate::dispatch) use audit_policy::reject_unsupported_audit_strict;
