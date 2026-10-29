//! Typed errors for the Lib dev loop.
//! Lỗi typed cho vòng dev Lib.

use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub(in crate::commands::core::dev) enum LibDevError {
    /// The project declares no language whose sources the loop can watch.
    /// (Project không khai ngôn ngữ mà vòng lặp biết watch.)
    #[error(
        "no lib language detected at '{root}': expected Cargo.toml, pyproject.toml, go.mod, \
         pom.xml/build.gradle, a .csproj, or tsconfig.json/package.json. \
         `mgc dev --core lib` rebuilds on source change and needs one of those manifests"
    )]
    LanguageUnknown { root: String },

    /// A language was detected but no source file matched its watch rules.
    /// (Đã nhận diện ngôn ngữ nhưng không có source khớp rule watch.)
    #[error(
        "'{root}' has a lib manifest but no watched source ({extensions:?}); \
         a build-watch loop with nothing to watch would report success forever"
    )]
    NoSources { root: String, extensions: String },
}

pub(in crate::commands::core::dev) fn language_unknown(root: &Path) -> anyhow::Error {
    anyhow::Error::new(LibDevError::LanguageUnknown {
        root: root.display().to_string(),
    })
}

pub(in crate::commands::core::dev) fn no_sources(
    root: &Path,
    extensions: &[&str],
) -> anyhow::Error {
    anyhow::Error::new(LibDevError::NoSources {
        root: root.display().to_string(),
        extensions: extensions.join(", "),
    })
}
