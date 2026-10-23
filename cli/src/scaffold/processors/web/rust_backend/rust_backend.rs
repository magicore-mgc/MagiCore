//! Native Rust Web backend starters (Axum, Actix Web).
//! Starter backend Web Rust native (Axum, Actix Web).

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;
use super::super::common::package_name;

pub(crate) fn write_axum(target: &Path, name: &str) -> Result<()> {
    let package_name = package_name(name);
    write_file(
        &target.join("Cargo.toml"),
        &format!(
            "[package]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\naxum = \"0.8\"\nserde_json = \"1\"\ntokio = {{ version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }}\n"
        ),
    )?;
    write_file(
        &target.join("src/main.rs"),
        "use axum::{routing::get, Json, Router};\nuse serde_json::{json, Value};\n\nasync fn health() -> Json<Value> {\n    Json(json!({\"status\": \"ok\"}))\n}\n\n#[tokio::main]\nasync fn main() {\n    let app = Router::new().route(\"/api/health\", get(health));\n    let listener = tokio::net::TcpListener::bind(\"0.0.0.0:4315\")\n        .await\n        .expect(\"bind HTTP listener\");\n    axum::serve(listener, app).await.expect(\"serve HTTP requests\");\n}\n",
    )
}

pub(crate) fn write_actix_web(target: &Path, name: &str) -> Result<()> {
    let package_name = package_name(name);
    write_file(
        &target.join("Cargo.toml"),
        &format!(
            "[package]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nactix-web = \"4\"\n"
        ),
    )?;
    write_file(
        &target.join("src/main.rs"),
        "use actix_web::{get, App, HttpServer, Responder};\n\n#[get(\"/api/health\")]\nasync fn health() -> impl Responder {\n    \"{\\\"status\\\":\\\"ok\\\"}\"\n}\n\n#[actix_web::main]\nasync fn main() -> std::io::Result<()> {\n    let port = std::env::var(\"PORT\")\n        .ok()\n        .and_then(|value| value.parse().ok())\n        .unwrap_or(4315);\n    HttpServer::new(|| App::new().service(health))\n        .bind((\"0.0.0.0\", port))?\n        .run()\n        .await\n}\n",
    )
}
