//! Bevy fallback project files — file project fallback Bevy.

use std::path::Path;

use anyhow::Result;

use super::super::super::{slugify, write_file};

pub(in crate::scaffold::processors::game) fn files(target: &Path, name: &str) -> Result<()> {
    write_file(
        &target.join("Cargo.toml"),
        &CARGO_MANIFEST.replace("{}", &slugify(name)),
    )?;
    write_file(&target.join("src").join("main.rs"), MAIN_SOURCE)?;
    Ok(())
}

const CARGO_MANIFEST: &str = r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.19"
"#;

const MAIN_SOURCE: &str = r#"use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, frame_counter)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        color: Color::srgb(0.2, 0.8, 0.4),
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    });
}

fn frame_counter(mut frames: Local<u64>) {
    *frames += 1;
    if *frames % 60 == 0 {
        println!("MagiCore bevy game running ({} frames)", *frames);
    }
}
"#;
