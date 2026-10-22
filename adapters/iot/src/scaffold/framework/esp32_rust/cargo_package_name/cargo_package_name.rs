//! Validate a stable Cargo manifest package name.
//! Xác thực tên package trong manifest Cargo stable.

use mgc_types::{MgError, MgResult};

const INVALID_NAME_GUIDANCE: &str = concat!(
    "Cargo package names must start with a Unicode identifier character or underscore ",
    "and continue with Unicode identifier characters or hyphens"
);

pub(in crate::scaffold) fn validate_esp32_cargo_package_name(name: &str) -> MgResult<()> {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return Err(invalid_package_name());
    };
    if first != '_' && !unicode_ident::is_xid_start(first) {
        return Err(invalid_package_name());
    }
    if characters.any(|character| character != '-' && !unicode_ident::is_xid_continue(character)) {
        return Err(invalid_package_name());
    }
    Ok(())
}

fn invalid_package_name() -> MgError {
    MgError::InvalidPackageName(INVALID_NAME_GUIDANCE.to_owned())
}
