//! Ordered multi-platform scaffold coordinator.
//! Điều phối scaffold đa nền tảng theo thứ tự.

use std::path::Path;

use super::MultiProjectContext;
use anyhow::Result;

#[path = "android/mod.rs"]
mod android;
#[path = "flutter/mod.rs"]
mod flutter;
#[path = "ios/mod.rs"]
mod ios;
#[path = "kmp_shared/mod.rs"]
mod kmp_shared;
#[path = "react_native/mod.rs"]
mod react_native;

pub(in crate::scaffold::processors::app::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    let context = MultiProjectContext::new(target, name);
    kmp_shared::write(&context)?;
    android::write(&context)?;
    ios::write(&context)?;
    react_native::write(&context)?;
    flutter::write(&context)?;
    Ok(())
}
