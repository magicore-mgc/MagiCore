//! Tests for the dev route table.

use super::super::dispatch::{DevRoute, route_for};

#[test]
fn known_cores_keep_the_existing_routes() {
    let routes = [
        ("web", DevRoute::Web),
        ("game", DevRoute::Game),
        ("iot", DevRoute::Iot),
        ("cloud", DevRoute::Cloud),
        ("cicd", DevRoute::Cicd),
        ("app", DevRoute::App),
        ("ai", DevRoute::Ai),
    ];

    for (core, expected) in routes {
        assert_eq!(route_for(core), expected);
    }
}

#[test]
fn unknown_cores_preserve_the_name_for_the_existing_error() {
    assert_eq!(route_for("lib"), DevRoute::Unsupported("lib"));
}
