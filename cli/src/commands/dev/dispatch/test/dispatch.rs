//! Tests for the dev route table.
//! Test bảng route của `mgc dev`.

use super::super::dispatch::{DevRoute, route_for};

#[test]
fn known_cores_keep_the_existing_routes() {
    let routes = [
        ("web", DevRoute::Web),
        ("game", DevRoute::Game),
        ("iot", DevRoute::Iot),
        ("cloud", DevRoute::Cloud),
        ("clo", DevRoute::Cloud),
        ("cicd", DevRoute::Cicd),
        ("app", DevRoute::App),
        ("ai", DevRoute::Ai),
        ("lib", DevRoute::Lib),
        ("library", DevRoute::Lib),
        ("hardware", DevRoute::Hardware),
    ];

    for (core, expected) in routes {
        assert_eq!(route_for(core), expected);
    }
}

/// A core nobody routes keeps its name so the error can name it back.
/// (Core không được route giữ nguyên tên để lỗi nêu lại được.)
#[test]
fn unknown_cores_preserve_the_name_for_the_existing_error() {
    assert_eq!(route_for("quantum"), DevRoute::Unsupported("quantum"));
}

/// Every core the CLI advertises must reach a route, so no advertised core can
/// fall through to "not implemented" by omission.
/// (Mọi core CLI quảng cáo phải có route, để không core nào rơi vào "chưa
/// implement" do thiếu sót.)
#[test]
fn every_advertised_core_has_a_route() {
    for core in [
        "web", "lib", "ai", "app", "game", "iot", "clo", "cloud", "cicd", "hardware",
    ] {
        assert_ne!(
            route_for(core),
            DevRoute::Unsupported(core),
            "'{core}' is advertised by `mgc capabilities` but has no dev route"
        );
    }
}
