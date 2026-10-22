// wheel_compatibility behavior tests — test behavior thuộc owner wheel_compatibility.
use super::*;

#[test]
fn runtime_wheel_gate_checks_platform_and_python_abi() {
    let matches = |filename: &str, os, arch, python, platform_version| {
        PypiProtocol::is_importable_wheel_for_target(filename, os, arch, python, platform_version)
    };

    assert!(matches(
        "pydantic_core-2.0.0-cp312-cp312-macosx_11_0_arm64.whl",
        "macos",
        "aarch64",
        Some((3, 12)),
        Some((14, 0))
    ));
    assert!(matches(
        "pydantic_core-2.0.0-cp312-cp312-manylinux_2_17_x86_64.whl",
        "linux",
        "x86_64",
        Some((3, 12)),
        Some((2, 39))
    ));
    assert!(matches(
        "pydantic_core-2.0.0-cp312-cp312-win_amd64.whl",
        "windows",
        "x86_64",
        Some((3, 12)),
        None
    ));
    assert!(matches(
        "pydantic_core-2.0.0-cp37-abi3-win_amd64.whl",
        "windows",
        "x86_64",
        Some((3, 12)),
        None
    ));
    assert!(!matches(
        "pydantic_core-2.0.0-cp312-cp312-win_amd64.whl",
        "windows",
        "x86_64",
        None,
        None
    ));
    assert!(!matches(
        "pydantic_core-2.0.0-cp312-cp312-win_amd64.whl",
        "macos",
        "aarch64",
        Some((3, 12)),
        Some((14, 0))
    ));
    assert!(!matches(
        "pydantic_core-2.0.0-cp311-cp311-manylinux_2_17_x86_64.whl",
        "linux",
        "x86_64",
        Some((3, 12)),
        Some((2, 39))
    ));
    assert!(matches(
        "six-1.17.0-py3-none-any.whl",
        "windows",
        "x86_64",
        None,
        None
    ));
    assert!(!matches(
        "demo-1.0.0-cp312-cp312-macosx_15_0_arm64.whl",
        "macos",
        "aarch64",
        Some((3, 12)),
        Some((14, 0))
    ));
    assert!(!matches(
        "demo-1.0.0-cp312-cp312-manylinux_2_39_x86_64.whl",
        "linux",
        "x86_64",
        Some((3, 12)),
        Some((2, 28))
    ));
}
