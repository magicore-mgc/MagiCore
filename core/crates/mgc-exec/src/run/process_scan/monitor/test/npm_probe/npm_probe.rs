//! `npm_probe.rs` — Covers the exact npm `--version` allow exception and generated read-only shim recognition..
//! Bao phủ ngoại lệ npm `--version` chính xác và nhận diện generated shim chỉ đọc.
use super::super::*;

#[test]
#[cfg(unix)]
fn npm_version_probe_requires_exact_executor_shim_path_and_arguments() {
    let root = tempfile::tempdir().expect("create process-shim test directory");
    let shadow = ShadowPath::create_at(root.path().join("mgc-exec shadow path test"), &[])
        .expect("create authenticated executor shims");
    let shim = shadow.path().join("npm");
    let unquoted_sh_probe = format!("/bin/sh {} --version", shim.display());
    let quoted_sh_probe = format!("/bin/sh \"{}\" --version", shim.display());
    let bash_image_sh_probe = format!("/bin/sh {} --version", shim.display());
    let impostor_then_shim = format!(
        "/bin/sh /real/path/npm --version {} --version",
        shim.display()
    );

    assert!(is_shadow_npm_version_probe(
        b"sh",
        unquoted_sh_probe.as_bytes(),
        shadow.path()
    ));
    assert!(is_shadow_npm_version_probe(
        b"sh",
        quoted_sh_probe.as_bytes(),
        shadow.path()
    ));
    assert!(is_shadow_npm_version_probe(
        b"bash",
        bash_image_sh_probe.as_bytes(),
        shadow.path()
    ));
    assert!(is_shadow_npm_version_probe(
        b"npm",
        format!("{} --version", shim.display()).as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"sh",
        impostor_then_shim.as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh {} install", shim.display()).as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh {} --version install", shim.display()).as_bytes(),
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"npm",
        b"npm --version",
        shadow.path()
    ));
    assert!(!is_shadow_npm_version_probe(
        b"node",
        format!("node {} --version", shim.display()).as_bytes(),
        shadow.path()
    ));
}

#[cfg(unix)]
#[test]
fn nested_generated_npm_version_probe_is_read_only_and_exact() {
    let shadow = ShadowPath::create(&[]).expect("create generated executor shims");
    let shim = shadow.path().join("npm");
    let exact_probe = format!("/bin/sh \"{}\" --version", shim.display());

    assert!(super::is_generated_shadow_npm_version_probe(
        b"sh",
        exact_probe.as_bytes()
    ));
    assert!(super::is_generated_shadow_npm_version_probe(
        b"bash",
        exact_probe.as_bytes()
    ));
    assert!(!super::is_generated_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh \"{}\" install", shim.display()).as_bytes()
    ));
    assert!(!super::is_generated_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh \"{}\" --version install", shim.display()).as_bytes()
    ));

    let fake_root = tempfile::tempdir().expect("create arbitrary shim directory");
    let fake_dir = fake_root.path().join("mgc-exec-shadow-path-1-2-3");
    std::fs::create_dir(&fake_dir).expect("create fake generated-name directory");
    let fake_shim = fake_dir.join("npm");
    std::fs::write(&fake_shim, super::NPM_VERSION_PROBE_SHIM_CONTENT)
        .expect("write exact probe bytes to an untrusted directory");
    assert!(!super::is_generated_shadow_npm_version_probe(
        b"sh",
        format!("/bin/sh {} --version", fake_shim.display()).as_bytes()
    ));
}

#[cfg(unix)]
#[test]
fn shell_wrapped_npm_probe_accepts_mac_sh_image_alias_only_for_exact_probe() {
    assert!(super::is_npm_version_shell_probe(
        b"bash",
        b"/bin/sh -c npm --version"
    ));
    assert!(!super::is_npm_version_shell_probe(
        b"bash",
        b"/bin/sh -c npm install"
    ));
    assert!(!super::is_npm_version_shell_probe(b"npm", b"npm --version"));
}
