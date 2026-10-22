// Bare route error categories — map local failures to established CLI diagnostics.
// Enum này phân loại lỗi bare rồi ánh xạ về diagnostic CLI hiện hành.

pub(super) enum BareRouteError {
    GlobalInstallUnsupported,
    InstallDryRunUnsupported { core: String },
    OfflineInstallUnsupported { core: String },
    CoreNotDetected { verb: String },
    UnknownCore { core: String },
}

impl BareRouteError {
    pub(super) fn into_error(self) -> anyhow::Error {
        match self {
            Self::GlobalInstallUnsupported => crate::error::global_package_install_unsupported(),
            Self::InstallDryRunUnsupported { core } => {
                crate::error::install_dry_run_unsupported(&core)
            }
            Self::OfflineInstallUnsupported { core } => anyhow::anyhow!(
                "offline install is not implemented for core '{}'; the request was not run",
                core
            ),
            Self::CoreNotDetected { verb } => crate::error::bare_core_not_detected(&verb),
            Self::UnknownCore { core } => crate::error::unknown_core(&core),
        }
    }
}

pub(super) fn unknown_core(core: &str) -> anyhow::Error {
    BareRouteError::UnknownCore {
        core: core.to_string(),
    }
    .into_error()
}
