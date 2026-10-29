use super::*;

#[test]
fn verify_chain_parses_custom_or_default() {
    let tmp = std::env::temp_dir().join(format!("mgc-cicd-chain-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    std::fs::write(
        tmp.join("mgc.toml"),
        "[cicd]\nprovider = \"github-actions\"\nverify = [\"audit\", \"build\"]\n",
    )
    .unwrap();
    assert_eq!(verify_chain(&tmp).unwrap(), vec!["audit", "build"]);
    std::fs::write(
        tmp.join("mgc.toml"),
        "[cicd]\nprovider = \"github-actions\"\n",
    )
    .unwrap();
    // No core marker: the generic default still ends with a build step.
    // (Không có core marker: mặc định chung vẫn có bước build.)
    assert_eq!(verify_chain(&tmp).unwrap(), vec!["audit", "test", "build"]);

    // A declared CI/CD project can never satisfy a build step, so its default
    // chain stops before it instead of failing every run.
    // (Project khai báo CI/CD không thể thỏa bước build, nên chain mặc định
    // dừng trước bước đó thay vì fail mọi lần chạy.)
    std::fs::write(tmp.join(".mgc.core"), "cicd\n").unwrap();
    std::fs::write(
        tmp.join("mgc.toml"),
        "name = \"pipe\"\necosystem = \"cicd\"\n\n[cicd]\nprovider = \"github-actions\"\n",
    )
    .unwrap();
    assert_eq!(verify_chain(&tmp).unwrap(), vec!["audit", "test"]);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[cfg(unix)]
#[test]
fn cicd_config_readers_reject_external_mgc_toml_symlink() {
    let root = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    std::fs::write(
        external.path().join("mgc.toml"),
        "[cicd]\nverify = ['audit']\n\n[[deploy.targets]]\nprovider = 'cloudflare'\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        external.path().join("mgc.toml"),
        root.path().join("mgc.toml"),
    )
    .unwrap();

    assert!(verify_chain(root.path()).is_err());
    assert!(deploy_targets(root.path()).is_err());
}
