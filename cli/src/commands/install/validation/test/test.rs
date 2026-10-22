#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::super::*;
use crate::commands::install::validation::validation::reject_toolchain_owned_packages;
use mgc_types::adapter::PackageAdapter;
use tempfile::tempdir;

#[cfg(feature = "clo")]
#[test]
fn clo_adapter_path_terraform_gates_without_compat() {
    // Terraform is unsupported until MGC owns its dependency lifecycle;
    // an absent compatibility runner must not be presented as opt-in support.
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("main.tf"), "terraform {}\n").unwrap();
    let err =
        clo_adapter_path_gate(dir.path(), crate::commands::dep_gate::DepOp::Install).unwrap_err();
    assert!(
        err.to_string()
            .contains("unsupported for dependency lifecycle"),
        "unexpected error: {err}"
    );
}

#[cfg(feature = "clo")]
#[test]
fn clo_adapter_path_cdk_skips_gate_natively() {
    // CDK rides the native web engine inside the adapter — no gate, no
    // compat needed.
    let dir = tempdir().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"x\"\n[cloud]\ntype = \"cdk\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("package.json"), r#"{"name":"x"}"#).unwrap();
    clo_adapter_path_gate(dir.path(), crate::commands::dep_gate::DepOp::Install).unwrap();
}

#[cfg(feature = "clo")]
#[test]
fn clo_adapter_path_undetected_type_fails_closed() {
    // No detectable cloud type is never assumed native.
    let dir = tempdir().unwrap();
    let err =
        clo_adapter_path_gate(dir.path(), crate::commands::dep_gate::DepOp::Install).unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[cfg(feature = "iot")]
#[test]
fn toolchain_owned_packages_rejected_before_adapter_calls() {
    // P0-1: platformio.ini (toolchain-owned) + packages fails PURELY —
    // no adapter call, no network, no spawn, no journal. The provider
    // toolchain must run explicitly.
    // (Manifest của tool + packages → lỗi thuần, không side effect.)
    let dir = tempdir().unwrap();
    let ini = "[env:test]\nplatform = atmelavr\nframework = arduino\n";
    std::fs::write(dir.path().join("platformio.ini"), ini).unwrap();
    let adapter =
        mgc_iot_adapter::adapter_for(dir.path()).expect("platformio must detect an iot adapter");
    assert!(
        !adapter.manifest_owned(),
        "platformio.ini must be toolchain-owned"
    );
    let err = reject_toolchain_owned_packages(&adapter, &["some-pkg".to_string()]).unwrap_err();
    assert!(
        format!("{err:#}").contains("does not own its native dependency lifecycle"),
        "must name the ownership reason: {err:#}"
    );
    assert_eq!(
        std::fs::read(dir.path().join("platformio.ini")).unwrap(),
        ini.as_bytes(),
        "manifest must be byte-identical (zero side effects)"
    );
    assert!(
        !dir.path()
            .join(".magicore/journal/dependency-mutation")
            .exists(),
        "no journal may be staged by a rejected op"
    );
    // Empty packages on the same lane stays allowed (pure install path).
    // (Không packages thì qua — đường install thuần.)
    reject_toolchain_owned_packages(&adapter, &[]).unwrap();
}

#[cfg(feature = "game")]
#[test]
fn game_owner_preflight_bevy_passes_godot_fails() {
    // P1-1: preflight dùng engine detect thật — bevy qua gate Native,
    // godot rớt catch-all Unsupported (không bao giờ đánh giá bằng
    // ownership của Bevy).
    // (Preflight uses the detected engine — godot fails closed.)
    let bevy = tempdir().unwrap();
    std::fs::write(
        bevy.path().join("mgc.toml"),
        "name = \"g\"\necosystem = \"game\"\n[game]\nengine = \"bevy\"\n",
    )
    .unwrap();
    std::fs::write(
        bevy.path().join("Cargo.toml"),
        "[package]\nname = \"g\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let bevy_adapter =
        mgc_game_adapter::adapter_for(bevy.path()).expect("bevy must detect a game adapter");
    validate_install_owner(&bevy_adapter, bevy.path()).unwrap();

    let godot = tempdir().unwrap();
    std::fs::write(godot.path().join("project.godot"), "; godot\n").unwrap();
    let godot_adapter =
        mgc_game_adapter::adapter_for(godot.path()).expect("godot must detect a game adapter");
    validate_install_owner(&godot_adapter, godot.path())
        .expect_err("godot must fail the ownership gate (Unsupported, never Bevy's cell)");
}

#[cfg(feature = "lib")]
#[test]
fn generic_install_owner_accepts_native_maven_and_dotnet_manifests() {
    // The generic `mgc install` entry must carry the same manifest-format
    // ownership evidence as `mgc install-lib` for native Java/.NET lanes.
    // (Lệnh install tổng quát phải truyền bằng chứng manifest như install-lib.)
    let maven = tempdir().unwrap();
    std::fs::write(
        maven.path().join("pom.xml"),
        "<project><modelVersion>4.0.0</modelVersion></project>",
    )
    .unwrap();
    let maven_adapter = mgc_lib_adapter::adapter_for(maven.path(), None, None)
        .unwrap()
        .expect("Maven manifest must detect a library adapter");
    validate_install_owner(&maven_adapter, maven.path())
        .expect("native Maven POM must pass the generic install ownership gate");

    let dotnet = tempdir().unwrap();
    std::fs::write(
        dotnet.path().join("Demo.csproj"),
        "<Project><PropertyGroup><TargetFramework>net8.0</TargetFramework></PropertyGroup></Project>",
    )
    .unwrap();
    let dotnet_adapter = mgc_lib_adapter::adapter_for(dotnet.path(), None, None)
        .unwrap()
        .expect("csproj must detect a library adapter");
    validate_install_owner(&dotnet_adapter, dotnet.path())
        .expect("native .NET csproj must pass the generic install ownership gate");

    let gradle = tempdir().unwrap();
    std::fs::write(gradle.path().join("build.gradle.kts"), "plugins {}\n").unwrap();
    let gradle_adapter = mgc_lib_adapter::adapter_for(gradle.path(), None, None)
        .unwrap()
        .expect("Gradle manifest must still detect a library adapter");
    validate_install_owner(&gradle_adapter, gradle.path())
        .expect_err("Gradle scripts must remain outside native Maven ownership");

    let ambiguous = tempdir().unwrap();
    std::fs::write(
        ambiguous.path().join("pom.xml"),
        "<project><modelVersion>4.0.0</modelVersion></project>",
    )
    .unwrap();
    std::fs::write(ambiguous.path().join("build.gradle"), "plugins {}\n").unwrap();
    let ambiguous_adapter = mgc_lib_adapter::adapter_for(ambiguous.path(), None, None)
        .unwrap()
        .expect("mixed Maven/Gradle project must detect a library adapter");
    validate_install_owner(&ambiguous_adapter, ambiguous.path())
        .expect_err("mixed Maven/Gradle ownership must fail closed");
}

#[cfg(feature = "ai")]
#[test]
fn generic_install_owner_recognizes_native_ai_pep621_lane() {
    let native = tempdir().unwrap();
    std::fs::write(
        native.path().join("pyproject.toml"),
        "[project]\nname = \"demo-agent\"\ndependencies = [\"six>=1.16\"]\n\n[tool.magicore]\nframework = \"python-agent\"\n",
    )
    .unwrap();
    std::fs::write(
        native.path().join("mgc.toml"),
        "name = \"demo-agent\"\necosystem = \"ai\"\n[ai]\nframework = \"python-agent\"\n",
    )
    .unwrap();
    let adapter =
        mgc_ai_adapter::adapter_for(native.path()).expect("AI pyproject should detect an adapter");
    assert!(
        adapter.python_lane.is_some(),
        "PEP 621 lane should be native"
    );
    validate_install_owner(&adapter, native.path())
        .expect("generic install must pass the same native AI lane as install-ai");
    validate_install_owner_for_op(
        &adapter,
        native.path(),
        crate::commands::dep_gate::DepOp::OfflineReinstall,
    )
    .expect("native AI offline ownership must reach verified cache-only replay");

    std::fs::write(native.path().join("uv.lock"), "version = 1\n").unwrap();
    let foreign = mgc_ai_adapter::adapter_for(native.path())
        .expect("AI project with foreign lock should still detect");
    assert!(
        foreign.python_lane.is_none(),
        "foreign lock must disable native lane"
    );
    validate_install_owner(&foreign, native.path())
        .expect_err("generic install must not admit a Python project owned by uv.lock");
    validate_install_owner_for_op(
        &foreign,
        native.path(),
        crate::commands::dep_gate::DepOp::OfflineReinstall,
    )
    .expect_err("foreign AI lock must not admit offline replay");
}

#[cfg(feature = "clo")]
#[test]
fn generic_install_owner_gates_cloudflare_and_cdk_by_real_manifest_route() {
    let cloudflare = tempdir().unwrap();
    std::fs::write(cloudflare.path().join("wrangler.toml"), "name = 'demo'\n").unwrap();
    let cloudflare_adapter = mgc_cloud_adapter::adapter_for(cloudflare.path())
        .unwrap()
        .expect("wrangler.toml should detect Cloudflare");
    validate_install_owner(&cloudflare_adapter, cloudflare.path())
        .expect_err("Cloudflare must not skip the generic ownership gate");

    let cdk = tempdir().unwrap();
    std::fs::write(
        cdk.path().join("mgc.toml"),
        "name = \"demo\"\necosystem = \"cloud\"\n[cloud]\ntype = \"cdk\"\n",
    )
    .unwrap();
    std::fs::write(
        cdk.path().join("package.json"),
        r#"{"name":"demo","dependencies":{"aws-cdk-lib":"^2.0.0"}}"#,
    )
    .unwrap();
    let cdk_adapter = mgc_cloud_adapter::adapter_for(cdk.path())
        .unwrap()
        .expect("CDK project should detect a cloud adapter");
    validate_install_owner(&cdk_adapter, cdk.path())
        .expect("CDK with package.json should route through the native Web engine");
    validate_install_owner_for_op(
        &cdk_adapter,
        cdk.path(),
        crate::commands::dep_gate::DepOp::OfflineReinstall,
    )
    .expect("CDK embeds the Web engine's cache-only offline installer");
}
