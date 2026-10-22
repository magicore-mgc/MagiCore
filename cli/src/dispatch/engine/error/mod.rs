// Shared engine policy error wiring — keep one error owner for sibling policies.
// Wiring error policy dùng chung để các policy sibling không tạo type trùng.
mod error;
#[cfg(test)]
mod test;

pub(super) use error::EnginePolicyError;
