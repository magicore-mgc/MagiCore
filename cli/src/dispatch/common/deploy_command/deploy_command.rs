//! Deploy command routing — chọn lane deploy theo project core và build features.
//! Không tự suy đoán lane khác với detector và thứ tự feature hiện hữu.
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum DeployTarget {
    #[cfg(feature = "cicd")]
    Cicd,
    #[cfg(feature = "clo")]
    Clo,
}

fn plan_target(core: Option<&str>) -> Result<DeployTarget> {
    match core {
        #[cfg(feature = "cicd")]
        Some("cicd") => Ok(DeployTarget::Cicd),
        #[cfg(all(not(feature = "cicd"), feature = "clo"))]
        Some("cicd") => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "clo")]
        _ => Ok(DeployTarget::Clo),
        #[cfg(not(feature = "clo"))]
        _ => Err(crate::error::core_not_in_build("clo")),
    }
}

pub(in crate::dispatch) async fn dispatch(run: bool) -> Result<()> {
    let core = crate::dispatch::types::detect_ecosystem()?;
    match plan_target(core.as_deref())? {
        #[cfg(feature = "cicd")]
        DeployTarget::Cicd => crate::commands::core::dev::cicd::deploy(run).await,
        #[cfg(feature = "clo")]
        DeployTarget::Clo => crate::commands::core::dev::clo::deploy(run).await,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
