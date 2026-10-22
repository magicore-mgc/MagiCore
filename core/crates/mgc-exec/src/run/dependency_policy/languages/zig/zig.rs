//! Zig package/build denial and direct source execution exception.
//! Chặn package/build Zig, chỉ cho phép ngoại lệ chạy source trực tiếp.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    args: &[String],
) -> Option<&'static str> {
    if args == ["version"] || (args.len() == 2 && args[0] == "run" && args[1].ends_with(".zig")) {
        return None;
    }
    Some(
        "Zig package/build commands are blocked; only direct source execution is allowed through mgc-exec",
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
