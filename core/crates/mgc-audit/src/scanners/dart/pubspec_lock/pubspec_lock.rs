//! Stable pubspec.lock parser entry point.
//! Entry point parser pubspec.lock ổn định.

use crate::scanners::osv::OsvPin;
use mgc_types::MgResult;

use super::parser::parse;

pub(in crate::scanners::dart) fn read(raw: &str) -> MgResult<(Vec<OsvPin>, Vec<String>)> {
    parse(raw)
}

#[cfg(test)]
#[path = "test/pubspec_lock.rs"]
mod tests;
