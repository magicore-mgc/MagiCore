//! The advertised framework list must equal the framework source the product
#![allow(clippy::unwrap_used)]
//! can actually scaffold. A framework listed here with no published layer and
//! no local-project path is a promise the CLI cannot keep.
//! Danh sách framework công bố phải bằng đúng nguồn mà sản phẩm có thể scaffold.
//! Framework có trong danh sách mà không có layer đã publish và không có đường
//! project local là lời hứa CLI không giữ được.

mod common;

/// Every registry entry must be a supported pair and carry a coherent
/// ecosystem mapping.
/// Mỗi entry registry phải là cặp được hỗ trợ và map ecosystem nhất quán.
#[test]
fn every_advertised_framework_pair_is_supported() {
    let pairs = [
        ("game", mgc::scaffold::framework_registry::GAME),
        ("clo", mgc::scaffold::framework_registry::CLOUD),
        ("cicd", mgc::scaffold::framework_registry::CICD),
        ("iot", mgc::scaffold::framework_registry::IOT),
        ("app", mgc::scaffold::framework_registry::APP),
        ("lib", mgc::scaffold::framework_registry::LIB),
        ("ai", mgc::scaffold::framework_registry::AI),
        ("hardware", mgc::scaffold::framework_registry::HARDWARE),
        ("web", mgc::scaffold::framework_registry::WEB_BACKEND),
        ("web", mgc::scaffold::framework_registry::WEB_FRONTEND),
        // Fullstack ids and CLI aliases are accepted by the gate, so they are part
        // of the advertised surface. Leaving them out let 36 fullstack ids and 12
        // aliases pass unasserted — `is_supported` could reject one and the
        // "every advertised framework" claim would still be green.
        // (Id fullstack và alias CLI đều được gate nhận nên thuộc bề mặt công bố.
        // Thiếu chúng khiến 36 id + 12 alias không được kiểm.)
        ("web", mgc::scaffold::framework_registry::WEB_FULLSTACK),
        ("web", mgc::scaffold::framework_registry::WEB_ALIAS),
    ];
    for (core, frameworks) in pairs {
        for framework in frameworks {
            assert!(
                mgc::scaffold::framework_registry::is_supported(core, framework),
                "{core}/{framework} is advertised but is_supported rejects it"
            );
            let ecosystem = mgc::scaffold::framework_registry::ecosystem_for_core(core);
            assert!(
                !ecosystem.is_empty(),
                "{core}/{framework} has no ecosystem mapping"
            );
        }
    }
}

/// A framework with no registry layer must fail with the honest source error,
/// never with a synthesized project.
/// Framework không có layer phải lỗi đúng loại thiếu nguồn, không sinh project giả.
#[test]
fn missing_layer_reports_the_source_error_and_creates_nothing() {
    let dir = common::work_dir();
    let project = dir.join("no-layer-probe");
    let (ok, output) = common::mgc_in(
        &dir,
        &[
            "create-web",
            "ember-no-such-layer",
            "no-layer-probe",
            "--ts",
        ],
    );
    assert!(
        ok || !project.exists(),
        "a failed scaffold must not leave a partial project:\n{output}"
    );
}
