use crate::scaffold::processors::lib::LibProcessor;

#[test]
fn python_scaffold_emits_src_package_and_import_test() {
    let temp = tempfile::tempdir().expect("tempdir");
    LibProcessor::files(temp.path(), "demo-lib", "python").expect("python scaffold");

    let metadata = std::fs::read_to_string(temp.path().join("pyproject.toml")).expect("pyproject");
    assert!(metadata.contains("name = \"demo-lib\""));
    assert!(metadata.contains("requires-python = \">=3.11\""));
    assert!(temp.path().join("src/demo_lib/__init__.py").is_file());
    assert!(temp.path().join("tests/test_package.py").is_file());
    let conftest =
        std::fs::read_to_string(temp.path().join("tests/conftest.py")).expect("conftest");
    assert!(conftest.contains("/ \"src\""));
}
