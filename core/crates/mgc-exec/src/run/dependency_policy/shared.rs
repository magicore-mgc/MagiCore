//! Shared command-position parsing for dependency policy owners.
//! Parser vị trí command dùng chung cho các owner dependency policy.

pub(super) fn first_command(args: &[String]) -> (&str, &str) {
    let mut positional = Vec::with_capacity(2);
    let mut skip_value = false;
    for arg in args {
        if skip_value {
            skip_value = false;
            continue;
        }
        if matches!(
            arg.as_str(),
            "--manifest-path"
                | "--config"
                | "--color"
                | "--message-format"
                | "--target-dir"
                | "--target"
                | "--package"
                | "--exclude"
                | "--package-path"
                | "-C"
        ) {
            skip_value = true;
            continue;
        }
        if arg.starts_with('-') || arg.starts_with('+') {
            continue;
        }
        positional.push(arg.as_str());
        if positional.len() == 2 {
            break;
        }
    }
    (
        positional.first().copied().unwrap_or_default(),
        positional.get(1).copied().unwrap_or_default(),
    )
}
