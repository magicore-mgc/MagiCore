#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Adapter tests

use super::*;
use crate::engine::GameEngine;

#[test]
fn test_cache_dir() {
    let dir = cache_dir(GameEngine::Godot).unwrap();
    assert!(dir.ends_with("cache/game/godot"));
    assert!(dir.starts_with(mgc_platform::paths::mgc_data_dir().join("cache/game")));
}

#[test]
fn test_cache_dir_unity() {
    let dir = cache_dir(GameEngine::Unity).unwrap();
    assert!(dir.ends_with("cache/game/unity"));
    assert!(dir.starts_with(mgc_platform::paths::mgc_data_dir().join("cache/game")));
}

#[test]
fn test_cached_binary_path() {
    let path = cached_binary_path(GameEngine::Godot, "4.3.0").unwrap();
    assert!(path.to_string_lossy().contains("godot-4.3.0"));
}

#[test]
fn test_cached_binary_path_bevy_error() {
    let result = cached_binary_path(GameEngine::Bevy, "0.14");
    assert!(result.is_err());
}

#[test]
fn test_is_cached_bevy() {
    let cached = is_cached(GameEngine::Bevy, "0.14").unwrap();
    assert!(!cached);
}
