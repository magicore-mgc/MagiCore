//! Validate identity and operation capability of a native adapter — xác minh identity và capability adapter.
//! Adapter identity alone never bypasses the static dependency ownership policy.
//! Identity adapter không tự vượt qua policy ownership dependency.

use crate::commands::compat::CompatMode;
use crate::commands::dep_gate::eco;
use crate::commands::dep_gate::{DepContext, DepGateError, DepOp, DepOwner, owner_for};
use anyhow::Result;

/// Gate a dependency operation routed through an already-constructed
/// adapter whose identity and declared operation capability are checked
/// here. This supports project-specific native lanes (AI/Python with a
/// PEP 621 manifest) without changing the static owner of foreign-lock
/// compatibility lanes.
/// (Cổng native theo adapter đã dựng: xác minh identity + capability,
/// không mở nhầm nhánh foreign-lock delegated.)
pub fn gate_native_adapter(
    ctx: &DepContext<'_>,
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    compat: &CompatMode,
) -> Result<()> {
    if let CompatMode::Explicit(value) = compat {
        return Err(DepGateError::compat_disabled(value).into());
    }
    let identity = adapter
        .manifest_identity()
        .ok_or_else(|| DepGateError::unsupported(ctx.core, ctx.op.as_str(), ctx.ecosystem))?;
    let expected_language = ctx.ecosystem.unwrap_or_default();
    if identity.core != ctx.core || identity.language != expected_language {
        return Err(DepGateError::identity_mismatch(
            &ctx.describe(),
            ctx.op.as_str(),
            ctx.core,
            expected_language,
            &identity.core,
            &identity.language,
        )
        .into());
    }
    if ctx.core == "ai" && ctx.ecosystem == Some(eco::PYTHON) && identity.format != "pyproject.toml"
    {
        return Err(DepGateError::unsupported(ctx.core, ctx.op.as_str(), ctx.ecosystem).into());
    }

    // The validated AI Python adapter identity exists only after adapter
    // construction selected the MGC-owned PEP 621 lane. Put that manifest
    // fact into the same context consumed by owner_for and capabilities JSON.
    // Chỉ adapter đã xác minh PEP 621 mới bổ sung manifest này vào context dùng chung.
    let owner_context = if ctx.core == "ai" && ctx.ecosystem == Some(eco::PYTHON) {
        DepContext {
            manifest_format: Some("mgc-pyproject"),
            ..*ctx
        }
    } else {
        *ctx
    };
    if !matches!(owner_for(&owner_context), DepOwner::Native) {
        return Err(DepGateError::unsupported(ctx.core, ctx.op.as_str(), ctx.ecosystem).into());
    }
    let required = match ctx.op {
        DepOp::Install | DepOp::OfflineReinstall => {
            mgc_types::capabilities::Capability::ContentStoreProvider
        }
        DepOp::List => mgc_types::capabilities::Capability::LockfileProvider,
        DepOp::Add | DepOp::Remove | DepOp::Update => {
            mgc_types::capabilities::Capability::DependencyResolver
        }
        _ => {
            return Err(DepGateError::unsupported(ctx.core, ctx.op.as_str(), ctx.ecosystem).into());
        }
    };
    if !adapter.capabilities().contains(&required)
        || (ctx.op == DepOp::Update && !adapter.supports_native_update())
    {
        return Err(DepGateError::unsupported(ctx.core, ctx.op.as_str(), ctx.ecosystem).into());
    }
    Ok(())
}
