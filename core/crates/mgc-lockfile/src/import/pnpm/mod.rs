//! pnpm lockfile parser owner — owner parser lockfile pnpm.

mod pnpm;
pub(in crate::import) use pnpm::parse_pnpm;
