//! Capability ownership assertions — assertion capability ownership.
//! Keeps the original assertion statements grouped by output surface.
//! Giữ assertion gốc, nhóm theo bề mặt output.

use crate::commands::capabilities::dependency_ownership::dependency_ownership;
use crate::commands::dep_gate::test_support::*;

#[allow(dead_code)] // Called by the grouped ownership test to keep assertions together.
pub(super) fn assert_manifest_and_core_overrides() {
    // Splitting cores expose per-ecosystem overrides with their concrete
    // manifest formats; the unspecialized base remains unsupported.
    // Core có phân nhánh xuất override theo ecosystem/manifest; base chưa chuyên biệt bị chặn.
    let lib_languages = dependency_ownership("lib")["languages"].clone();
    assert_eq!(lib_languages["ts"]["add"]["owner"], "mgc-native");
    assert_eq!(lib_languages["rust"]["install"]["owner"], "mgc-native");
    assert_eq!(lib_languages["rust"]["add"]["owner"], "mgc-native");
    // Native capability rows carry no external dependency-manager tools.
    // Capability native không khai báo tool dependency bên ngoài.
    assert_eq!(lib_languages["python"]["add"]["owner"], "mgc-native");
    assert_eq!(
        lib_languages["python"]["add"]["tools"],
        serde_json::json!([])
    );
    assert_eq!(lib_languages["python"]["remove"]["owner"], "mgc-native");
    assert_eq!(
        lib_languages["python"]["remove"]["tools"],
        serde_json::json!([])
    );
    assert_eq!(lib_languages["go"]["remove"]["owner"], "mgc-native");
    assert_eq!(lib_languages["go"]["add"]["owner"], "mgc-native");
    let game_languages = dependency_ownership("game")["languages"].clone();
    assert_eq!(game_languages["bevy"]["install"]["owner"], "mgc-native");
    assert_eq!(game_languages["bevy"]["install"]["requires"], "Cargo.toml");
    let iot_languages = dependency_ownership("iot")["languages"].clone();
    assert_eq!(iot_languages["esp32-rust"]["add"]["owner"], "mgc-native");
    assert_eq!(iot_languages["esp32-rust"]["add"]["requires"], "Cargo.toml");
    let lib_operations = &dependency_ownership("lib")["operations"];
    assert_eq!(lib_operations["add"]["owner"], "unsupported");
    assert_eq!(lib_operations["install"]["owner"], "unsupported");
    let variants = &dependency_ownership("lib")["manifest_variants"];
    assert_eq!(
        variants["ts/package-json"]["install"]["owner"],
        "mgc-native"
    );
    assert_eq!(variants["ts/unknown"]["install"]["owner"], "unsupported");
    assert_eq!(variants["rust/cargo-toml"]["add"]["owner"], "mgc-native");
    assert_eq!(variants["rust/unknown"]["add"]["owner"], "unsupported");
    assert_eq!(
        variants["python/pep621-native"]["install"]["owner"],
        "mgc-native"
    );
    assert_eq!(
        variants["python/pyproject-unsupported"]["install"]["owner"],
        "unsupported"
    );
    assert_eq!(variants["go/go-mod"]["install"]["owner"], "mgc-native");
    assert_eq!(variants["go/unknown"]["install"]["owner"], "unsupported");
    assert_eq!(variants["java/maven-pom"]["add"]["owner"], "mgc-native");
    assert_eq!(variants["java/maven-pom"]["install"]["owner"], "mgc-native");
    assert_eq!(variants["java/gradle"]["add"]["owner"], "unsupported");
    assert_eq!(variants["java/gradle"]["install"]["owner"], "unsupported");
    assert_eq!(variants["dotnet/csproj"]["update"]["owner"], "mgc-native");
    assert_eq!(variants["java/unknown"]["add"]["owner"], "unsupported");
    assert_eq!(
        variants["dotnet/unknown"]["install"]["owner"],
        "unsupported"
    );
    let app_languages = dependency_ownership("app")["languages"].clone();
    // "rn" is omitted when identical to the (unsupported) base row — the
    // languages map carries ONLY differing ecosystems. When present it
    // must read unsupported (the gate-level rule itself is locked by
    // app_rn_unsupported_flutter_delegated_unknown_unsupported).
    // `rn` chỉ xuất khi khác base; nếu xuất hiện thì owner vẫn phải là unsupported.
    if let Some(rn) = app_languages.get("rn") {
        assert_eq!(rn["install"]["owner"], "unsupported");
    }
    assert_eq!(app_languages["flutter"]["install"]["owner"], "mgc-native");
    assert_eq!(app_languages["flutter"]["update"]["owner"], "mgc-native");
    assert_eq!(app_languages["swift"]["update"]["owner"], "unsupported");
    // Exact app verbs stay native only where complete; no delegated labels.
    // App chỉ báo native cho verb đã hoàn chỉnh, không gắn nhãn delegated.
    assert_eq!(app_languages["swift"]["add"]["owner"], "unsupported");
    assert_eq!(app_languages["swift"]["install"]["owner"], "mgc-native");
    assert_eq!(app_languages["swift"]["remove"]["owner"], "unsupported");
    assert!(app_languages.get("kotlin").is_none()); // all operations inherit unsupported base owner.
    for op in [
        DepOp::Add,
        DepOp::Remove,
        DepOp::Update,
        DepOp::Install,
        DepOp::List,
    ] {
        assert!(matches!(
            owner_for(&ctx("app", Some(eco::KOTLIN), op)),
            DepOwner::Unsupported
        ));
    }
    assert!(matches!(
        owner_for(&ctx("app", Some(eco::OBJC), DepOp::List)),
        DepOwner::Unsupported
    ));
    assert!(matches!(
        owner_for(&ctx("app", Some(eco::OBJC), DepOp::Install)),
        DepOwner::Unsupported
    ));
    // Hardware list reads scaffold-only, never mgc-native.
    // Hardware list là scaffold-only, không được báo MGC-native.
    let hardware_full = dependency_ownership("hardware");
    assert_eq!(
        hardware_full["operations"]["list"]["owner"],
        "scaffold-only"
    );
    assert_eq!(
        hardware_full["operations"]["install"]["owner"],
        "unsupported"
    );
}
