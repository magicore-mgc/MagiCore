/// First positional subcommand (skips leading flags and `--` terminator).
/// Subcommand vị trí đầu (bỏ qua flag đầu và terminator `--`).
pub(in crate::policy) fn first_subcommand(args: &[String]) -> Option<String> {
    let mut ended_flags = false;
    for arg in args {
        if !ended_flags && arg == "--" {
            ended_flags = true;
            continue;
        }
        if !ended_flags && arg.starts_with('-') {
            continue;
        }
        return Some(arg.to_ascii_lowercase());
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
