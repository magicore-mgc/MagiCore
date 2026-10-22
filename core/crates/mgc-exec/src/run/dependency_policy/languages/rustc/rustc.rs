//! Allow only the read-only Rust host-target probe.
//! Chỉ cho phép probe target host Rust chỉ đọc.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    args: &[String],
) -> Option<&'static str> {
    if args.len() == 1 && args[0] == "-vV" {
        None
    } else {
        Some("rustc is restricted to the read-only -vV host target query")
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
