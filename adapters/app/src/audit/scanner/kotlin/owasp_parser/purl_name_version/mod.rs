//! OWASP package URL name/version parsing owner wiring.
//! File này chỉ nối module purl_name_version.

mod purl_name_version;

pub(super) use purl_name_version::purl_name_version;
