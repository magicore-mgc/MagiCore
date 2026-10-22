// Engine policy error categories — preserve the existing CLI diagnostic surface.
// Enum này phân loại lỗi policy và giữ nguyên diagnostic CLI hiện hành.

pub(in crate::dispatch::engine) enum EnginePolicyError {
    RecursiveNotAvailable,
    RecursiveUnsupported { command: &'static str },
    FilterUnsupported { command: &'static str },
    FilterRequiresRecursive,
    AuditStrictUnsupported { command: &'static str },
}

impl EnginePolicyError {
    pub(in crate::dispatch::engine) fn into_error(self) -> anyhow::Error {
        match self {
            Self::RecursiveNotAvailable => anyhow::anyhow!(
                "--recursive is declared on the CLI surface but workspace recursion is not implemented yet. Refusing to pretend it ran."
            ),
            Self::RecursiveUnsupported { command } => anyhow::anyhow!(
                "--recursive is not implemented for '{}' yet. Beta safety rule: refusing a silent no-op on workspace commands.",
                command
            ),
            Self::FilterUnsupported { command } => anyhow::anyhow!(
                "--filter is not wired into '{}' yet (the publish recursive pipeline does not accept filter). Refusing a silent no-op.",
                command
            ),
            Self::FilterRequiresRecursive => anyhow::anyhow!(
                "--filter requires --recursive (it filters workspace targets). Use `mgc <cmd> --recursive --filter <glob>`."
            ),
            Self::AuditStrictUnsupported { command } => {
                crate::error::audit_strict_unsupported_command(command)
            }
        }
    }
}
