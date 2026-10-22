//! Tests for the .NET manifest detector.
//! Kiểm thử detector manifest .NET.

#![allow(clippy::unwrap_used)]

use super::*;

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-detect-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn dotnet_step_fires_on_csproj() {
    let dir = tmp("dotnet");
    std::fs::write(dir.join("app.csproj"), "<Project/>\n").unwrap();
    let step = dotnet_step(&dir).expect("*.csproj must yield a dotnet step");
    assert_eq!(step.ecosystem, "dotnet");
    assert_eq!(step.scanner, "osv-nuget");
}

#[test]
fn dotnet_step_fires_on_solution() {
    let dir = tmp("dotnet-sln");
    std::fs::write(
        dir.join("app.sln"),
        "Microsoft Visual Studio Solution File\n",
    )
    .unwrap();
    let step = dotnet_step(&dir).expect("*.sln must yield a dotnet step");
    assert_eq!(step.ecosystem, "dotnet");
    assert_eq!(step.scanner, "osv-nuget");
}
