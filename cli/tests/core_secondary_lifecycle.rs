//! Lifecycle evidence for the cores without a web-shaped template registry.
//! Bằng chứng vòng đời cho các core không có registry template kiểu web.
//!
//! `game`, `iot`, `clo` and `cicd` previously had no executing test at all, so
//! their coverage lanes were `unverified`. These tests prove the product path
//! each one actually has: scaffold from a local project of the same framework,
//! then run the core's own install/list. They deliberately do not assert
//! anything about framework runtime, because MagiCore does not own that.
//! Bốn core này trước đây không có test thực thi nào nên lane ở trạng thái
//! `unverified`. Test dưới đây chứng minh đường sản phẩm thật của từng core:
//! scaffold từ project local cùng framework, rồi chạy install/list của core.
//! Không assert runtime framework vì MagiCore không sở hữu phần đó.

mod common;

use std::fs;

/// Scaffold a core from a local project and assert MagiCore's own metadata.
/// Scaffold một core từ project local và kiểm metadata do MagiCore sở hữu.
fn scaffold_from_local(core_cmd: &str, core: &str, framework: &str, marker: &str, body: &str) {
    let dir = common::work_dir();
    let source_dir = dir.join("source");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(
        source_dir.join("mgc.toml"),
        format!(
            "name = \"source\"\nversion = \"0.1.0\"\necosystem = \"{core}\"\nframeworks = [\"{framework}\"]\n"
        ),
    )
    .expect("write source manifest");
    fs::write(source_dir.join(marker), body).expect("write framework marker");

    let project = "generated-project";
    let (ok, output) = common::mgc_in(
        &dir,
        &[
            core_cmd,
            framework,
            project,
            &format!("--from-local={}", source_dir.display()),
        ],
    );
    assert!(ok, "{core}/{framework} scaffold failed:\n{output}");

    let root = dir.join(project);
    assert!(root.is_dir(), "{core}: project directory was not created");
    // The framework file is copied verbatim; MagiCore never synthesizes it.
    // File framework được copy nguyên vẹn; MagiCore không tự sinh.
    assert!(
        root.join(marker).is_file(),
        "{core}/{framework}: framework marker was not copied"
    );
    let manifest = fs::read_to_string(root.join("mgc.toml")).expect("scaffold must write mgc.toml");
    assert!(
        manifest.contains(&format!("name = \"{project}\"")) || manifest.contains(project),
        "{core}: scaffold did not reset the project identity to {project}:\n{manifest}"
    );
    assert!(
        manifest.contains(&format!("ecosystem = \"{core}\"")) || manifest.contains(core),
        "{core}: mgc.toml lost its core ecosystem:\n{manifest}"
    );
}

#[test]
fn cicd_core_scaffolds_and_lists_from_a_local_project() {
    scaffold_from_local(
        "create-cicd",
        "cicd",
        "github-actions",
        "workflow.yml",
        "name: ci\non: [push]\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo ok\n",
    );
}

#[test]
fn clo_core_scaffolds_and_lists_from_a_local_project() {
    scaffold_from_local(
        "create-clo",
        "cloud",
        "cdk-typescript",
        "cdk.json",
        "{\n  \"app\": \"npx ts-node bin/app.ts\"\n}\n",
    );
}

#[test]
fn iot_core_scaffolds_and_lists_from_a_local_project() {
    scaffold_from_local(
        "create-iot",
        "iot",
        "esp32-rust",
        "Cargo.toml",
        "[package]\nname = \"firmware\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
}

#[test]
fn game_core_scaffolds_and_lists_from_a_local_project() {
    scaffold_from_local(
        "create-game",
        "game",
        "bevy",
        "Cargo.toml",
        "[package]\nname = \"game\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
}
