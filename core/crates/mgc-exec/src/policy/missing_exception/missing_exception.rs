use crate::policy::{DenyCode, PolicyDecision};

/// Public re-export for callers that only need the missing-exception code.
/// Re-export cho caller chỉ cần mã thiếu exception.
pub fn missing_exception() -> PolicyDecision {
    PolicyDecision::Deny {
        code: DenyCode::MissingException,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
