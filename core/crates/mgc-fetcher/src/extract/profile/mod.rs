//! Wires the opt-in extraction profile output.
//! Nối phần output profile giải nén tùy chọn.

mod profile;
#[cfg(test)]
pub(super) use self::profile::format_profile_line;
pub(super) use self::profile::report;
#[cfg(test)]
mod test;
