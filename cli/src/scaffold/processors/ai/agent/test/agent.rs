//! Contract tests for the agent writer.
//! Test contract cho writer agent.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path()).unwrap();
    let content = std::fs::read_to_string(target.path().join("src/agent.py")).unwrap();
    assert!(content.contains("class AIAgent"));
    assert!(content.contains("Agent processed successfully!"));
}

#[test]
fn test_ai_python_agent_scaffold_is_syntactically_valid() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("ai-agent");

    crate::scaffold::processors::ai::AiProcessor::files(&target, "ai-agent", "python-agent")
        .unwrap();

    let output = std::process::Command::new("python3")
        .args(["-m", "py_compile"])
        .arg(target.join("src/agent.py"))
        .arg(target.join("src/mgc_model_runtime.py"))
        .env("PYTHONPYCACHEPREFIX", target.join(".pycache"))
        .output()
        .expect("python3 is required by the AI scaffold lifecycle gate");

    assert!(
        output.status.success(),
        "generated AI Python must compile: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // A preloaded stdlib compression module must not shadow scaffold configuration.
    // Module compression stdlib đã load không được che cấu hình scaffold.
    let runtime = std::process::Command::new("python3")
        .arg("-c")
        .arg("import sys, types; sys.modules['compression'] = types.ModuleType('compression'); import agent; assert agent.AIAgent().runtime.target_bits is None")
        .env("PYTHONPATH", target.join("src"))
        .output()
        .unwrap();
    assert!(
        runtime.status.success(),
        "AI scaffold import failed: {}",
        String::from_utf8_lossy(&runtime.stderr)
    );

    let pyproject = std::fs::read_to_string(target.join("pyproject.toml")).unwrap();
    assert!(
        pyproject.contains("build-backend = \"setuptools.build_meta\""),
        "AI scaffold must support mgc build"
    );
    assert!(
        pyproject.contains("package-dir = {\"\" = \"src\"}"),
        "AI scaffold must package modules from src"
    );
}
