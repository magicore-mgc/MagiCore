//! ShadowPath lifecycle owner wiring.
//! Wiring owner vòng đời ShadowPath.

mod shadow_path;

#[cfg(unix)]
pub(in crate::run::implementation) use shadow_path::is_generated_shadow_directory;
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use shadow_path::unique_temp_dir;
