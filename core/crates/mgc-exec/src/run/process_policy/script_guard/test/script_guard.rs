//! Script guard integration contracts.
//! Contract tích hợp guard script.

#[test]
fn script_gate_ignores_package_manager_mentions_after_shebang()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let script = root.path().join("next");
    std::fs::write(
        &script,
        "#!/usr/bin/env node\n// npm is mentioned in a comment\nconst note = 'npm install';\n",
    )?;

    super::reject_forbidden_script_file(script.to_str().ok_or("script path is not UTF-8")?)?;
    Ok(())
}
#[test]
fn script_gate_rejects_forbidden_package_manager_interpreter()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let script = root.path().join("bad-script");
    std::fs::write(&script, "#!/usr/bin/env npm\ninstall\n")?;

    let error =
        super::reject_forbidden_script_file(script.to_str().ok_or("script path is not UTF-8")?)
            .expect_err("npm must remain forbidden as the script interpreter");
    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm'")
    );
    Ok(())
}
