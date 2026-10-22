//! Audit command planning and execution — lập plan và gọi audit handler.
//! Plan giữ nguyên policy cờ signature audit và route handler hiện hữu.
use crate::commands;
use crate::commands::definitions::AuditCmd;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum AuditPlan {
    Signatures {
        package: String,
        protocol: String,
        registry: String,
        token: Option<String>,
        json: bool,
    },
    Package {
        fix: bool,
        format: Option<String>,
    },
}

fn plan(cmd: Option<AuditCmd>, fix: bool, format: Option<String>) -> Result<AuditPlan> {
    match cmd {
        Some(AuditCmd::Signatures {
            package,
            protocol,
            registry,
            token,
            json,
        }) => {
            if fix || format.is_some() {
                return Err(crate::error::audit_signatures_flags_conflict());
            }
            Ok(AuditPlan::Signatures {
                package,
                protocol,
                registry,
                token,
                json,
            })
        }
        None => Ok(AuditPlan::Package { fix, format }),
    }
}

pub(in crate::dispatch) async fn dispatch(
    cmd: Option<AuditCmd>,
    fix: bool,
    format: Option<String>,
    core: Option<&str>,
) -> Result<()> {
    match plan(cmd, fix, format)? {
        AuditPlan::Signatures {
            package,
            protocol,
            registry,
            token,
            json,
        } => {
            commands::audit::signatures::run(
                core,
                &package,
                &protocol,
                &registry,
                token.as_deref(),
                json,
            )
            .await
        }
        AuditPlan::Package { fix, format } => {
            commands::audit::run(core, fix, format.as_deref()).await
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
