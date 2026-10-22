// Environment setup and deny-shim tests.
// Test setup môi trường và deny-shim.

use super::{ShadowPath, is_path_env_key};
use std::fs;

#[test]
fn path_environment_key_uses_host_case_semantics() {
    assert!(is_path_env_key("PATH"));
    #[cfg(windows)]
    assert!(is_path_env_key("Path"));
    #[cfg(not(windows))]
    assert!(!is_path_env_key("Path"));
}

#[test]
#[cfg(unix)]
fn shadow_directory_creation_refuses_a_preexisting_symlink()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let target = root.path().join("attacker-owned");
    fs::create_dir(&target)?;
    let protected_file = target.join("npm");
    fs::write(&protected_file, "do not overwrite")?;
    let link = root.path().join("shadow-link");
    std::os::unix::fs::symlink(&target, &link)?;

    let result = ShadowPath::create_at(link, &[]);

    assert!(
        result.is_err(),
        "existing symlink must not be adopted as a shim directory"
    );
    assert_eq!(fs::read_to_string(protected_file)?, "do not overwrite");
    Ok(())
}
