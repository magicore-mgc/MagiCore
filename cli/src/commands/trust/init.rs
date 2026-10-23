//! mgc trust init — Initialize keyring
//! mgc trust init — Khởi tạo keyring

use mgc_crypto::keyring::{KeyPair, Keyring};

/// Execute `mgc trust init` — Thực thi `mgc trust init`
pub fn execute(force: bool) -> anyhow::Result<()> {
    let keyring_path = Keyring::default_path().map_err(|error| anyhow::anyhow!("{error}"))?;
    execute_at(&keyring_path, force)
}

fn execute_at(keyring_path: &std::path::Path, force: bool) -> anyhow::Result<()> {
    match std::fs::symlink_metadata(keyring_path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            anyhow::bail!(
                "keyring path '{}' must be a regular non-symlink file; no files were changed",
                keyring_path.display()
            );
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    if keyring_path.exists() && !force {
        match Keyring::load(keyring_path) {
            Ok(_) => {
                println!(
                    "✓ Keyring already initialized at: {}",
                    keyring_path.display()
                );
                println!("  Use --force to reinitialize");
                return Ok(());
            }
            Err(error) => {
                let backup = keyring_path.with_extension("json.bak");
                let backup_state = if backup.exists() {
                    match Keyring::load(&backup) {
                        Ok(_) => format!(
                            " A valid backup exists at '{}', but automatic restore is disabled to avoid overwriting or weakening key-file permissions.",
                            backup.display()
                        ),
                        Err(backup_error) => format!(
                            " The backup '{}' is also invalid: {backup_error}.",
                            backup.display()
                        ),
                    }
                } else {
                    " No backup exists.".to_string()
                };
                anyhow::bail!(
                    "existing keyring '{}' could not be validated: {error}.{backup_state} No files were changed. Recover the keyring explicitly after inspecting the files; do not use --force unless you intend to replace the existing signing identity.",
                    keyring_path.display()
                );
            }
        }
    }

    if force && keyring_path.exists() {
        println!("WARN: Reinitializing keyring (old keys will be lost)");
    }

    // Generate new key
    let key_pair = KeyPair::generate()?;
    let key_id = key_pair.key_id.clone();

    // Create keyring
    let mut keyring = Keyring::new();
    keyring.add_key(key_pair);

    // Save keyring
    keyring.save(keyring_path)?;

    println!("✓ Keyring initialized");
    println!("  Location: {}", keyring_path.display());
    println!("  Default key: {}", key_id);

    // R3.1 FIX (AUDIT VÒNG 2): Detect existing signatures and prompt re-sign
    let lockfile_path = std::env::current_dir()
        .ok()
        .map(|p| p.join("mgc.lock"))
        .unwrap_or_else(|| std::path::PathBuf::from("mgc.lock"));
    let sig_path = lockfile_path.with_extension("lock.sig");

    if sig_path.exists() && force {
        println!("\nWARN: Existing lockfile signature detected");
        println!("  Old signature will be invalid with new key");
        println!("  Run 'mgc trust sign' to re-sign with new key");
    }

    println!("\nNext steps:");
    println!("  • Run 'mgc trust sign' to sign your lockfile");
    println!("  • Run 'mgc trust list' to view keys");

    Ok(())
}

#[cfg(test)]
#[path = "test/init.rs"]
mod init_tests;
