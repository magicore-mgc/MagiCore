//! OWASP dependency-check fixture parser wiring.
//! File này chỉ nối schema và helper parser regression.

#[cfg(test)]
mod owasp_parser;
#[cfg(test)]
mod purl_name_version;
#[cfg(test)]
mod schema;
#[cfg(test)]
mod severity;
#[cfg(test)]
mod truncate_utf8;

#[cfg(test)]
pub(crate) use owasp_parser::parse_owasp_dependency_check_json;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
