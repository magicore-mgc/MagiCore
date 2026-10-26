//! Decide which engine builds a Web project.
//! Quyết định engine nào build một project Web.
//!
//! MagiCore has its own in-process esbuild bundler and measured it faster than
//! a framework build for the plainest projects. It cannot replace every
//! framework though: Next.js compiles server components with SWC, Angular needs
//! its own compiler, and Nuxt builds a server with Nitro. Those must run their
//! program. Everything else can be built here.
//! MagiCore có bundler esbuild chạy trong process riêng và đã đo nhanh hơn bản
//! build của framework với các project đơn giản nhất. Nó không thay được mọi
//! framework: Next.js biên dịch server component bằng SWC, Angular cần compiler
//! riêng, Nuxt build server bằng Nitro. Những cái đó phải chạy chương trình của
//! chúng. Phần còn lại build được ở đây.
//!
//! Choosing by framework rather than by "does package.json happen to have a
//! build script" is what makes the choice deliberate: a framework that needs its
//! compiler keeps it even if a script is missing, and a framework that does not
//! gets MagiCore's engine even when the script is present.
//! Chọn theo framework thay vì theo "package.json có tình cờ có script build
//! không" là điều làm lựa chọn có chủ đích: framework cần compiler riêng vẫn
//! giữ nó kể cả khi thiếu script, và framework không cần thì nhận engine
//! MagiCore ngay cả khi script có mặt.

/// Which engine produced, or should produce, the output.
/// Engine nào đã hoặc nên tạo output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum Engine {
    /// MagiCore's in-process esbuild bundler.
    /// Bundler esbuild trong process của MagiCore.
    MagiCore,
    /// The framework's own build program.
    /// Chương trình build của chính framework.
    Framework,
}

/// Frameworks whose build cannot be reproduced with esbuild alone.
/// Framework mà build của chúng không tái tạo được chỉ bằng esbuild.
///
/// Each entry needs a compiler or a server bundler that is part of the
/// framework itself, not a generic JavaScript transform.
/// Mỗi entry cần compiler hoặc server bundler thuộc chính framework, không phải
/// phép biến đổi JavaScript tổng quát.
const NEEDS_OWN_COMPILER: &[&str] = &[
    // Server components or server bundling.
    // Server component hoặc bundler phía server.
    "next",
    "nextjs",
    "nuxt",
    "remix",
    "astro",
    // Own compiler or single-file component format esbuild cannot parse.
    // Compiler riêng hoặc định dạng single-file mà esbuild không parse được.
    "angular",
    "sveltekit",
    "svelte",
    "vue",
    "vue-vite",
    "vue-cli",
    // Static-site generators with their own routing and asset pipeline.
    // SSG có routing và pipeline asset riêng.
    "gatsby",
    "docusaurus",
    "vitepress",
    // Frameworks whose entry is a framework-specific file type.
    // Framework có entry là loại file riêng của framework.
    "solid-start",
    "qwik",
];

/// How the caller wants the engine picked.
/// Cách caller muốn chọn engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum Preference {
    /// Pick from the framework (the default).
    /// Chọn theo framework (mặc định).
    Auto,
    /// Force MagiCore's bundler.
    /// Ép dùng bundler MagiCore.
    MagiCore,
    /// Force the framework's program.
    /// Ép dùng chương trình framework.
    Framework,
}

/// Pick the engine for `framework` under `preference`.
///
/// Returns the engine and the reason, so the build banner can say why instead
/// of leaving the user to guess which program ran.
/// Chọn engine cho `framework` theo `preference`. Trả engine và lý do để banner
/// build nói rõ vì sao thay vì để user đoán chương trình nào đã chạy.
pub(crate) fn choose(
    framework: Option<&str>,
    has_framework_script: bool,
    preference: Preference,
) -> (Engine, String) {
    match preference {
        Preference::MagiCore => (
            Engine::MagiCore,
            "engine forced to MagiCore by flag".to_string(),
        ),
        Preference::Framework => (
            Engine::Framework,
            "engine forced to the framework by flag".to_string(),
        ),
        Preference::Auto => {
            let Some(framework) = framework else {
                if has_framework_script {
                    return (
                        Engine::Framework,
                        "no framework identified; using its build script".to_string(),
                    );
                }
                return (
                    Engine::MagiCore,
                    "no framework identified and no build script; using MagiCore engine"
                        .to_string(),
                );
            };
            if NEEDS_OWN_COMPILER.contains(&framework) {
                return (
                    Engine::Framework,
                    format!("{framework} needs its own compiler"),
                );
            }
            if !has_framework_script {
                return (
                    Engine::MagiCore,
                    format!("{framework} has no build script; using MagiCore engine"),
                );
            }
            (
                Engine::MagiCore,
                format!("{framework} is buildable in-process; using MagiCore engine"),
            )
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
