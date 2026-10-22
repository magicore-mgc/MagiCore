#![cfg(test)]

//! Preserve engine labels and reject unknown labels.
//! Giữ nhãn engine và từ chối nhãn không biết.

use crate::engine::GameEngine;

#[test]
fn engine_names_round_trip_to_the_same_variants() {
    for (name, expected) in [
        ("bevy", GameEngine::Bevy),
        ("godot", GameEngine::Godot),
        ("unity", GameEngine::Unity),
        ("unreal", GameEngine::Unreal),
    ] {
        assert_eq!(GameEngine::from_str(name), Some(expected));
        assert_eq!(expected.as_str(), name);
    }
    assert_eq!(GameEngine::from_str("other"), None);
}
