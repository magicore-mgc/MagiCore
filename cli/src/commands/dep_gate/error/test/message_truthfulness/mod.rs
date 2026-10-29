// Gate messages must describe the real reason — thông điệp gate phải nêu đúng lý do.
//
// The `list` refusal used to say "the manifest is toolchain-owned" while
// `mgc add` wrote that very manifest. A user reading it would conclude they
// should use cargo directly, which is exactly the bypass MagiCore forbids.
// (Lý do từ chối `list` từng nói "manifest thuộc toolchain" trong khi `mgc add`
// ghi chính manifest đó — người đọc sẽ tưởng phải dùng cargo, đúng kiểu bypass
// MagiCore cấm.)

use crate::commands::dep_gate::DepGateError;

fn unsupported(core: &str, operation: &str, ecosystem: Option<&'static str>) -> String {
    DepGateError::UnsupportedOperation {
        core: core.to_string(),
        operation: operation.to_string(),
        ecosystem: ecosystem.map(str::to_string),
    }
    .to_string()
}

#[test]
fn a_list_refusal_names_the_real_gap() {
    let message = unsupported("lib", "list", Some("rust"));
    assert!(
        message.contains("no verified installed-artifact inventory"),
        "the real reason must be stated: {message}"
    );
    assert!(
        !message.contains("toolchain-owned"),
        "the manifest is MagiCore-owned (`mgc add` writes it); claiming otherwise \
         pushes the user toward bypassing MagiCore: {message}"
    );
    assert!(
        message.contains("--dry-run"),
        "a refusal must tell the user what they can do instead: {message}"
    );
}

#[test]
fn every_list_refusal_uses_the_inventory_wording() {
    for (core, ecosystem) in [
        ("lib", "rust"),
        ("lib", "go"),
        ("lib", "java"),
        ("lib", "dotnet"),
    ] {
        let message = unsupported(core, "list", Some(ecosystem));
        assert!(
            message.contains("installed-artifact inventory"),
            "{core}/{ecosystem}: {message}"
        );
    }
}

#[test]
fn other_operations_keep_the_generic_runner_wording() {
    for operation in ["add", "remove", "update"] {
        let message = unsupported("lib", operation, Some("rust"));
        assert!(
            !message.contains("installed-artifact inventory"),
            "{operation} must not borrow the list-specific reason: {message}"
        );
        assert!(
            !message.contains("toolchain-owned"),
            "{operation}: {message}"
        );
    }
}

#[test]
fn scaffold_only_cores_say_so_without_mentioning_inventory() {
    for core in ["cicd", "hardware"] {
        let message = unsupported(core, "list", None);
        assert!(message.contains("scaffold-only"), "{core}: {message}");
        assert!(!message.contains("inventory"), "{core}: {message}");
    }
}

/// The three core families each get a different, true reason.
///
/// One generic sentence cannot be true for all of them: `lib`/`ai` manifests are
/// MagiCore-owned, `game`/`iot`/`clo` manifests belong to their toolchain, and
/// `cicd`/`hardware` have no package lifecycle at all.
/// (Ba họ core cần ba lý do khác nhau và đều đúng: manifest lib/ai do MagiCore
/// sở hữu, game/iot/clo thuộc toolchain, cicd/hardware không có lifecycle.)
#[test]
fn each_core_family_gets_a_true_reason() {
    for core in ["lib", "ai"] {
        let message = unsupported(core, "add", Some("rust"));
        assert!(
            message.contains("MagiCore owns the manifest"),
            "{core}: {message}"
        );
        assert!(
            !message.contains("toolchain-owned"),
            "{core}: claiming toolchain ownership would invite a cargo/pip bypass: {message}"
        );
    }
    for core in ["iot", "game", "clo"] {
        let message = unsupported(core, "add", Some("platformio"));
        assert!(
            message.contains("toolchain-owned"),
            "{core} really is toolchain-owned — the reason must say so: {message}"
        );
    }
    for core in ["cicd", "hardware"] {
        let message = unsupported(core, "add", None);
        assert!(message.contains("scaffold-only"), "{core}: {message}");
        assert!(!message.contains("toolchain-owned"), "{core}: {message}");
    }
}
