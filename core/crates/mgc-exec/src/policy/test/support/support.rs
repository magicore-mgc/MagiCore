use crate::policy::{Invocation, LaneProfile, ReadOnlyException};

pub(crate) fn inv(program: &str, args: &[&str], in_compat_subtree: bool) -> Invocation {
    Invocation {
        program: program.to_string(),
        args: args.iter().map(|a| a.to_string()).collect(),
        env: Vec::new(),
        in_compat_subtree,
    }
}

pub(crate) fn lane() -> LaneProfile {
    LaneProfile {
        framework: "next".to_string(),
        version: "16.3.8".to_string(),
        os: "darwin".to_string(),
        operation: "dev".to_string(),
        allowed_pm_invocations: vec!["npm --version".to_string()],
        requires_pm: vec![],
    }
}

pub(crate) fn exc() -> ReadOnlyException {
    ReadOnlyException {
        program: "npm".to_string(),
        args_exact: vec!["--version".to_string()],
        unexpired: true,
        answer: None,
    }
}
