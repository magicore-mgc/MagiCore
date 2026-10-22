//! Static catalog of official backend initializer packages and arguments.
//! Catalog tĩnh package initializer backend và tham số chính thức.

use super::super::types::{OnlineGeneratorArgument, OnlineWebGenerator};

/// Return one backend initializer record without launching a package.
/// Trả record initializer backend, không tự chạy package.
pub(in crate::commands::core::web::implementation::online_generator) fn backend_web_generator(
    framework: &str,
) -> Option<OnlineWebGenerator> {
    use OnlineGeneratorArgument::{Literal as L, Project as P};

    let (package, binary, identity, args) = match framework {
        "express" => (
            "express-generator",
            "express",
            "express-generator:no-view",
            &[L("--no-view"), P][..],
        ),
        "nestjs" => (
            "@nestjs/cli",
            "nest",
            "nestjs:typescript",
            &[
                L("new"),
                P,
                L("--skip-install"),
                L("--skip-git"),
                L("--package-manager"),
                L("npm"),
                L("--language"),
                L("TS"),
                L("--no-observe"),
            ][..],
        ),
        _ => return None,
    };

    Some(OnlineWebGenerator {
        package,
        binary,
        identity,
        args,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
