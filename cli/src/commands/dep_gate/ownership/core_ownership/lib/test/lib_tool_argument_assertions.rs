//! Lib ownership assertions — assertion ownership Lib.
//! Keeps the existing assertion statements grouped by lifecycle concern.
//! Giữ assertion hiện có, nhóm theo phần lifecycle.

use crate::commands::dep_gate::test_support::*;

pub(super) fn assert_tool_arguments_do_not_authorize_compat() {
    // The legacy tool argument does not select or spawn Python tooling in native mode.
    // Tham số tool cũ không chọn hoặc spawn tool Python khi đang ở native mode.
    assert!(
        gate(
            &ctx("lib", Some(eco::PYTHON), DepOp::Add),
            Some("pip"),
            &native(),
            None
        )
        .is_ok()
    );
    assert!(
        gate(
            &ctx("lib", Some(eco::PYTHON), DepOp::Add),
            Some("pip"),
            &native(),
            None
        )
        .is_ok(),
        "native operation has no compatibility requirement"
    );
    // Supplying a legacy tool name with native mode does not request compatibility.
    // Truyền tên tool cũ cùng native mode không tạo yêu cầu chạy compat.
    assert!(
        gate(
            &ctx("lib", Some(eco::PYTHON), DepOp::Add),
            Some("pip"),
            &native(),
            None
        )
        .is_ok(),
        "native engine remains available without compatibility"
    );
    // The legacy tool parameter alone does not open an external runner.
    // Chỉ có tham số tool cũ thì không mở runner bên ngoài.
    assert!(
        gate(
            &ctx("lib", Some(eco::PYTHON), DepOp::Remove),
            Some("pip"),
            &native(),
            None
        )
        .is_ok(),
        "native remove remains available"
    );
    assert!(
        gate(
            &ctx("lib", Some(eco::PYTHON), DepOp::Update),
            Some("pip"),
            &native(),
            None
        )
        .is_ok(),
        "native update remains available"
    );
    // Per-language tool parameters remain non-authorizing for native operations.
    // Tham số tool theo language không cấp quyền cho native operation.
    assert!(
        gate(
            &ctx("lib", Some(eco::RUST), DepOp::Add),
            Some("cargo"),
            &native(),
            None
        )
        .is_ok()
    );
    assert!(
        gate(
            &ctx("lib", Some(eco::RUST), DepOp::Add),
            Some("cargo"),
            &native(),
            None
        )
        .is_ok(),
        "native add remains available"
    );
    assert!(
        gate(
            &ctx("lib", Some(eco::RUST), DepOp::Remove),
            Some("cargo"),
            &native(),
            None
        )
        .is_ok(),
        "native remove remains available"
    );
    // Go remove runs natively; an explicit compatibility mode is rejected globally.
    // Go remove chạy native; chế độ compat tường minh bị từ chối toàn cục.
    assert!(
        gate(
            &ctx("lib", Some(eco::GO), DepOp::Remove),
            None,
            &native(),
            None
        )
        .is_ok()
    );
    assert!(
        gate(
            &ctx("lib", Some(eco::GO), DepOp::Remove),
            Some("go"),
            &explicit("go"),
            None
        )
        .is_err()
    );
    // Java POM/.NET csproj Update run natively; Gradle and unknown formats
    // fail at the manifest-aware gate, before adapter construction.
    // Update Java POM/.NET csproj chạy native; Gradle/format lạ bị chặn trước adapter.
    for (lang, format) in [(eco::JAVA, "maven-pom"), (eco::DOTNET, "csproj")] {
        assert!(
            gate(
                &lib_ctx(lang, Some(format), DepOp::Update),
                None,
                &native(),
                None
            )
            .is_ok(),
            "lib[{lang}] update is native"
        );
    }
    // Undetected lib language: pipeline ops fail closed (P1 wildcard fix).
    // Không detect được language thì pipeline Lib bị từ chối.
    assert!(gate(&ctx("lib", None, DepOp::Install), None, &native(), None).is_err());
    assert!(gate(&ctx("lib", None, DepOp::Add), None, &native(), None).is_err());
}
