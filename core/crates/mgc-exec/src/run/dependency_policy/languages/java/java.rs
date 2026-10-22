//! Gradle and Maven dependency-resolution flags.
//! Flag resolve dependency của Gradle và Maven.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    tool: &str,
    args: &[String],
) -> Option<&'static str> {
    let has = |flag: &str| args.iter().any(|arg| arg == flag);
    match tool {
        "gradle" | "gradlew" if !has("--offline") => Some("Gradle commands must include --offline"),
        "mvn" | "mvnw" if !has("-o") && !has("--offline") => {
            Some("Maven commands must include offline mode (-o)")
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
