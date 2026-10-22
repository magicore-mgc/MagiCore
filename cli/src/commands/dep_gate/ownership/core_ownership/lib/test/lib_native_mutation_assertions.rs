//! Lib ownership assertions — assertion ownership Lib.
//! Keeps the existing assertion statements grouped by lifecycle concern.
//! Giữ assertion hiện có, nhóm theo phần lifecycle.

use crate::commands::dep_gate::test_support::*;

pub(super) fn assert_native_mutations() {
    // Native Add (resolve-first + mgc-side manifest edit, zero spawn):
    // Rust/Python/Go are language-owned; Java/.NET require explicit POM /
    // single-csproj formats. Gradle is rejected by the gate before adapter.
    // Add native resolve trước, sửa manifest bằng MGC; Java/.NET cần đúng format.
    for lang in [eco::RUST, eco::PYTHON, eco::GO] {
        assert!(
            gate(&ctx("lib", Some(lang), DepOp::Add), None, &native(), None).is_ok(),
            "lib[{lang}] add is native (no toolchain spawn)"
        );
    }
    for (lang, format) in [(eco::JAVA, "maven-pom"), (eco::DOTNET, "csproj")] {
        assert!(
            gate(
                &lib_ctx(lang, Some(format), DepOp::Add),
                None,
                &native(),
                None
            )
            .is_ok(),
            "lib[{lang}/{format}] add is native (no toolchain spawn)"
        );
    }
}
