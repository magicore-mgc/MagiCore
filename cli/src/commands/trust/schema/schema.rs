//! Trust subcommand schema (English help text — RULE §7).
//! Schema lệnh con trust (text help tiếng Anh — RULE §7).

use clap::Subcommand;

/// Trust subcommands (help text English — RULE §7).
/// Lệnh con trust (text help tiếng Anh — RULE §7).
#[derive(Debug, Clone, Subcommand)]
pub enum TrustCmd {
    /// Initialize keyring
    Init {
        /// Force reinitialize
        #[arg(long)]
        force: bool,
    },

    /// Sign lockfile
    Sign {
        /// Lockfile path
        #[arg(default_value = "mgc.lock")]
        lockfile: String,

        /// Key ID to use
        #[arg(long)]
        key_id: Option<String>,
    },

    /// Verify lockfile
    Verify {
        /// Lockfile path
        #[arg(default_value = "mgc.lock")]
        lockfile: String,
    },

    /// List keys
    List,

    /// Approve package lifecycle scripts
    Approve {
        /// Package name
        package: String,
    },

    /// Deny package lifecycle scripts
    Deny {
        /// Package name
        package: String,
    },

    /// Prune stale trust policies
    Prune,

    /// List installed packages with lifecycle scripts but no policy yet
    Pending,

    /// Attest the project core identity (binds .mgc.core + mgc.toml to a key)
    Anchor {
        /// Re-attest even when the current attestation still verifies
        #[arg(long)]
        re_attest: bool,

        /// Rotate to a freshly generated key (keeps one previous key)
        #[arg(long)]
        rotate: bool,

        /// Key ID to use (not combinable with --rotate, which generates one)
        #[arg(long)]
        key_id: Option<String>,
    },
}
