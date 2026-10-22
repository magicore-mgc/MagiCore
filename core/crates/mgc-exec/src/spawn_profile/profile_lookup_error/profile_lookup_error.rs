/// Profile lookup failure — lỗi tra cứu profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileLookupError {
    /// No profile file for this tuple — không có file profile cho bộ này.
    NotFound,
    /// Profile file failed to parse or has an unknown schema.
    /// File profile parse hỏng hoặc schema lạ.
    Invalid(String),
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
