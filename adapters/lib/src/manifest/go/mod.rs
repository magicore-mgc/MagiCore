//! Go manifest owners / Owner manifest Go.
mod parse_go_mod_manifest;
mod write_go_mod_manifest;
pub(crate) use self::parse_go_mod_manifest::parse_go_mod_manifest;
pub(crate) use self::write_go_mod_manifest::write_go_mod_manifest;
