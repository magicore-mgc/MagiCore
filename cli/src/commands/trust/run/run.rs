//! Trust command dispatch owner.
//! Owner dispatch lệnh trust.

use super::super::schema::TrustCmd;

/// Execute trust command — Thực thi lệnh trust
pub fn execute(cmd: TrustCmd) -> anyhow::Result<()> {
    match cmd {
        TrustCmd::Init { force } => super::super::init::execute(force),
        TrustCmd::Sign { lockfile, key_id } => {
            super::super::sign::execute(&lockfile, key_id.as_deref())
        }
        TrustCmd::Verify { lockfile } => super::super::verify::execute(&lockfile),
        TrustCmd::List => super::super::list::execute(),
        TrustCmd::Approve { package } => super::super::approve::execute(&package),
        TrustCmd::Deny { package } => super::super::deny::execute(&package),
        TrustCmd::Prune => super::super::prune::execute(),
        TrustCmd::Pending => super::super::pending::execute(),
        TrustCmd::Anchor {
            re_attest,
            rotate,
            key_id,
        } => super::super::anchor::execute(re_attest, rotate, key_id.as_deref()),
    }
}

/// Async wrapper for CLI dispatch — Wrapper async cho CLI dispatch
pub async fn run(cmd: TrustCmd) -> anyhow::Result<()> {
    execute(cmd)
}
