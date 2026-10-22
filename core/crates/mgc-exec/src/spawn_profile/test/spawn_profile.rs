// The recorder observes a real child tree (python parent + python child).
#[test]
fn recorder_observes_live_subtree() {
    let mut child = std::process::Command::new("python3")
        .args([
            "-c",
            concat!(
                "import subprocess, sys, time; ",
                "subprocess.Popen([sys.executable, '-c', ",
                "'import time; time.sleep(30)']); time.sleep(30)"
            ),
        ])
        .spawn()
        .unwrap();
    let observed = crate::spawn_profile::record_subtree(
        child.id(),
        std::time::Duration::from_secs(3),
        std::time::Duration::from_millis(200),
    );
    let _ = child.kill();
    let _ = child.wait();
    let py_count = observed
        .iter()
        .filter(|(name, _)| name.to_ascii_lowercase().contains("python"))
        .count();
    assert!(
        py_count >= 2,
        "expected parent+child python processes, saw: {observed:?}"
    );
}
