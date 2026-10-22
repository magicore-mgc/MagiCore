/// File name for one profile tuple — tên file cho một bộ profile.
pub fn profile_file_name(framework: &str, operation: &str, os: &str) -> String {
    let safe = |value: &str| {
        value
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                    character
                } else {
                    '-'
                }
            })
            .collect::<String>()
    };
    format!("{}-{}-{}.json", safe(framework), safe(operation), safe(os))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
