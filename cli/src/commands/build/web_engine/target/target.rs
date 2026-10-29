use mgc_config::project::ProjectExecutionConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WebBuildTarget {
    CompatibilityShell,
    NativeReady,
    CompiledExecutable,
}

impl WebBuildTarget {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::CompatibilityShell => "compatibility-shell",
            Self::NativeReady => "native-ready",
            Self::CompiledExecutable => "compiled-executable",
        }
    }
}

pub(crate) fn resolve_web_build_target(
    execution: &ProjectExecutionConfig,
    explicit_target: Option<&str>,
) -> WebBuildTarget {
    if let Some(target) = explicit_target.map(|value| value.trim().to_ascii_lowercase()) {
        return match target.as_str() {
            "native" | "compiled" | "compiled-executable" | "executable" => {
                WebBuildTarget::CompiledExecutable
            }
            "native-ready" => WebBuildTarget::NativeReady,
            _ => WebBuildTarget::CompatibilityShell,
        };
    }

    match execution.lane.trim().to_ascii_lowercase().as_str() {
        "compiled-executable" => WebBuildTarget::CompiledExecutable,
        "native-ready" => WebBuildTarget::NativeReady,
        _ => WebBuildTarget::CompatibilityShell,
    }
}
