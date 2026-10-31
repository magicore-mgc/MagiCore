//! Native (non-Node) server halves: FastAPI, Flask, Django, Axum and Gin.
//! Nửa server native (không Node): FastAPI, Flask, Django, Axum và Gin.
//!
//! Each entry describes a minimal server MagiCore can produce completely from
//! data: no framework initializer and no package manager runs. The half always
//! lands in `server/`, the directory `detect_project_mode` reads as
//! `FullstackSplit`, and always ships a manifest the dev lane can launch.
//! Mỗi entry mô tả server tối thiểu mà MagiCore sinh trọn từ dữ liệu: không chạy
//! initializer framework và không chạy package manager. Nửa server luôn nằm ở
//! `server/` — thư mục `detect_project_mode` đọc là `FullstackSplit` — và luôn
//! kèm manifest để lane dev khởi động được.

use std::path::{Path, PathBuf};

use anyhow::Result;

/// Default port the generated server listens on for non-Node backends.
/// Cổng mặc định server non-Node sinh ra lắng nghe.
///
/// This is the Web backend port from the centralized `dev_port` table, so the
/// `PORT`/`SERVER_PORT` the dev lane exports already point the half at the same
/// port. RULE §13: the digits 4·3·1·5 appear exactly once.
/// Đây là port backend Web trong bảng `dev_port` trung tâm, nên `PORT`/
/// `SERVER_PORT` do lane dev xuất ra đã trỏ nửa server đúng port đó. RULE §13:
/// các chữ số 4·3·1·5 xuất hiện đúng một lần.
pub(in crate::commands::core::web::implementation) const NATIVE_SERVER_PORT: u16 =
    crate::commands::core::dev_port::PORT_WEB_BE;

/// Directory that holds the server half inside the project.
/// Thư mục chứa nửa server trong project.
pub(in crate::commands::core::web::implementation) const SERVER_DIR: &str = "server";

/// File-name placeholders rendered by [`write`].
/// Placeholder tên file do [`write`] thay thế.
const PROJECT_PLACEHOLDER: &str = "@PROJECT@";
const MODULE_PLACEHOLDER: &str = "@MODULE@";
const DISTRIBUTION_PLACEHOLDER: &str = "@DISTRIBUTION@";
const PORT_PLACEHOLDER: &str = "@PORT@";
const HOST_PLACEHOLDER: &str = "@HOST@";

/// Local bind address the generated halves use when nothing overrides it.
/// Địa chỉ bind local mà nửa server sinh ra dùng khi không có gì ghi đè.
///
/// The dev lane exports `HOST` for browser preflight; this literal is only the
/// fallback a standalone run uses.
/// Lane dev xuất `HOST` cho preflight trình duyệt; literal này chỉ là mặc định
/// khi chạy riêng.
const DEFAULT_HOST: &str = "127.0.0.1";

/// One file written by a native half.
/// Một file do nửa native ghi ra.
struct FileSpec {
    /// Path relative to `server/` — đường dẫn tương đối so với `server/`.
    relative_path: &'static str,
    /// Rendered template — template được kết xuất.
    template: &'static str,
}

/// A backend half MagiCore writes without running any framework tool.
/// Nửa backend MagiCore ghi mà không chạy công cụ framework nào.
struct NativeHalf {
    /// Files written under `server/` — file ghi dưới `server/`.
    files: &'static [FileSpec],
    /// Manifest that makes `detect_project_mode` see a backend.
    /// Manifest khiến `detect_project_mode` thấy backend.
    ///
    /// Each backend keeps its ecosystem's own file name (`Cargo.toml`, `go.mod`,
    /// `pyproject.toml`); the shared `manifest_presence` predicate reads all of
    /// them. Python halves declare `pyproject.toml` — the form MagiCore's native
    /// Python resolver owns — so `mgc dev` verifies locks instead of refusing the
    /// directory as an ambient dependency manager.
    /// Mỗi backend giữ đúng tên file của hệ sinh thái (`Cargo.toml`, `go.mod`,
    /// `pyproject.toml`); predicate `manifest_presence` dùng chung đọc tất cả.
    /// Nửa Python khai `pyproject.toml` — dạng resolver Python native của MagiCore
    /// sở hữu — nên `mgc dev` kiểm lock thay vì từ chối thư mục như manager ngoài.
    manifest_file: &'static str,
    /// Manifest template — template manifest.
    manifest: &'static str,
}

/// Catalog of non-Node server halves.
/// Danh mục nửa server không phải Node.
///
/// The backend name is the split-combo suffix (`react-fastapi` → `fastapi`).
/// Tên backend là hậu tố combo tách rời (`react-fastapi` → `fastapi`).
fn catalog(backend: &str) -> Option<NativeHalf> {
    match backend {
        "fastapi" => Some(NativeHalf {
            files: &[FileSpec {
                relative_path: "main.py",
                template: FASTAPI_MAIN,
            }],
            manifest_file: "pyproject.toml",
            manifest: FASTAPI_MANIFEST,
        }),
        "flask" => Some(NativeHalf {
            files: &[FileSpec {
                relative_path: "app.py",
                template: FLASK_APP,
            }],
            manifest_file: "pyproject.toml",
            manifest: FLASK_MANIFEST,
        }),
        "django" => Some(NativeHalf {
            files: &[
                FileSpec {
                    relative_path: "manage.py",
                    template: DJANGO_MANAGE,
                },
                FileSpec {
                    relative_path: "config/settings.py",
                    template: DJANGO_SETTINGS,
                },
                FileSpec {
                    relative_path: "config/urls.py",
                    template: DJANGO_URLS,
                },
                FileSpec {
                    relative_path: "api/__init__.py",
                    template: DJANGO_API_INIT,
                },
                FileSpec {
                    relative_path: "api/views.py",
                    template: DJANGO_VIEWS,
                },
            ],
            manifest_file: "pyproject.toml",
            manifest: DJANGO_MANIFEST,
        }),
        "axum" => Some(NativeHalf {
            files: &[FileSpec {
                relative_path: "src/main.rs",
                template: AXUM_MAIN,
            }],
            manifest_file: "Cargo.toml",
            manifest: AXUM_MANIFEST,
        }),
        "gin" => Some(NativeHalf {
            files: &[FileSpec {
                relative_path: "main.go",
                template: GIN_MAIN,
            }],
            manifest_file: "go.mod",
            manifest: GIN_MOD,
        }),
        _ => None,
    }
}

/// Whether `backend` has a native (non-Node) offline server half.
/// `backend` có nửa server offline native (không phải Node) hay không.
pub(in crate::commands::core::web::implementation) fn has_half(backend: &str) -> bool {
    catalog(backend).is_some()
}

/// `server/` directory of a native half inside `root`.
/// Thư mục `server/` của nửa native trong `root`.
pub(in crate::commands::core::web::implementation) fn half_dir(root: &Path) -> PathBuf {
    root.join(SERVER_DIR)
}

/// Write the native server half of `combo` (backend `backend`) into `server/`.
/// Ghi nửa server native của `combo` (backend `backend`) vào `server/`.
///
/// Every file is rendered before the first write, so a template that cannot be
/// rendered leaves the project untouched. The caller owns the tree because this
/// same `mgc create` created it; this is a creation writer, never an edit of an
/// existing project.
/// Mọi file được kết xuất trước lần ghi đầu, nên template không kết xuất được
/// thì project không bị chạm. Cây thuộc caller vì chính lệnh `mgc create` này
/// tạo ra; đây là writer tạo mới, không phải chỉnh sửa project đã có.
pub(in crate::commands::core::web::implementation) fn write(
    root: &Path,
    backend: &str,
    combo: &str,
) -> Result<()> {
    let half = catalog(backend).ok_or_else(|| no_half(combo, backend))?;
    let name = crate::scaffold::processor::Scaffolder::display_name(root);
    // Render every byte before the first write.
    // (Kết xuất toàn bộ nội dung trước lần ghi đầu.)
    let rendered = render_into(&half, &name);
    for (path, contents) in rendered {
        let path = half_dir(root).join(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, contents)?;
    }
    Ok(())
}

/// Render every file of `half` as `(path relative to server/, contents)`.
/// Kết xuất mọi file của `half` thành `(đường dẫn tương đối so với server/, nội dung)`.
fn render_into(half: &NativeHalf, name: &str) -> Vec<(PathBuf, String)> {
    let module = python_module_name(name);
    half.files
        .iter()
        .map(|file| (file.relative_path, file.template))
        .chain(std::iter::once((half.manifest_file, half.manifest)))
        .map(|(path, template)| (PathBuf::from(path), render(template, name, &module)))
        .collect()
}

/// Substitute the compile-time placeholders of one template.
/// Thay các placeholder biên dịch của một template.
fn render(template: &str, name: &str, module: &str) -> String {
    template
        .replace(PROJECT_PLACEHOLDER, name)
        .replace(MODULE_PLACEHOLDER, module)
        .replace(DISTRIBUTION_PLACEHOLDER, module)
        .replace(PORT_PLACEHOLDER, &NATIVE_SERVER_PORT.to_string())
        .replace(HOST_PLACEHOLDER, DEFAULT_HOST)
}

/// Python module name derived from the project display name.
/// Tên module Python suy từ tên hiển thị của project.
///
/// Django's settings module must be importable, so every character outside
/// `[a-z0-9_]` folds to `_` and a leading digit gets a prefix. The result is the
/// only value inserted into Python `DJANGO_SETTINGS_MODULE` text.
/// Module settings của Django phải import được, nên mọi ký tự ngoài `[a-z0-9_]`
/// được đổi thành `_` và chữ số đứng đầu được thêm tiền tố. Đây là giá trị duy
/// nhất đi vào văn bản `DJANGO_SETTINGS_MODULE` của Python.
fn python_module_name(name: &str) -> String {
    let mut module: String = name
        .to_ascii_lowercase()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect();
    if module.is_empty() || module.starts_with(|c: char| c.is_ascii_digit()) {
        module.insert_str(0, "app_");
    }
    module
}

/// Error for a half that is neither Node nor native.
/// Lỗi cho nửa không phải Node và cũng không phải native.
fn no_half(combo: &str, backend: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "the '{combo}' combo needs both halves and MagiCore has no offline generator for its \
         '{backend}' server half. Scaffold each half with `mgc create-web <frontend> <name>` and \
         `mgc create-web {backend} <name>`, or supply the combo through `--from-local <path>`. \
         Nothing was created."
    )
}

/// FastAPI application entry.
/// Entry ứng dụng FastAPI.
const FASTAPI_MAIN: &str = r#"# API server generated by MagiCore for a split fullstack project.
# Server API do MagiCore sinh cho project fullstack tách rời.
import os

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(title="@PROJECT@")

# The dev frontend runs on another origin, so it must be allowed explicitly.
# (Frontend dev chạy ở origin khác nên phải cho phép tường minh.)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/api/health")
def health():
    return {"status": "ok", "service": "@PROJECT@"}


if __name__ == "__main__":
    import uvicorn

    host = os.environ.get("HOST", "@HOST@")
    port = int(os.environ.get("PORT", "@PORT@"))
    uvicorn.run(app, host=host, port=port)
"#;

/// FastAPI project manifest.
/// Manifest project FastAPI.
///
/// Dependencies stay plain `name + range`: MagiCore's native resolver models no
/// PEP 508 extras or markers, and a manifest it cannot own makes `mgc dev`
/// refuse the half instead of running it.
/// Dependency giữ dạng `name + range` thuần: resolver native của MagiCore không
/// mô hình extras/marker PEP 508, và manifest nó không sở hữu khiến `mgc dev`
/// từ chối nửa server thay vì chạy.
const FASTAPI_MANIFEST: &str = r#"# Project manifest generated by MagiCore for a split fullstack server half.
# Manifest project do MagiCore sinh cho nửa server của fullstack tách rời.
[project]
name = "@DISTRIBUTION@-server"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = ["fastapi>=0.110", "uvicorn>=0.29"]
"#;

/// Flask application entry.
/// Entry ứng dụng Flask.
const FLASK_APP: &str = r#"# API server generated by MagiCore for a split fullstack project.
# Server API do MagiCore sinh cho project fullstack tách rời.
import os

from flask import Flask, jsonify

app = Flask(__name__)


# The dev frontend runs on another origin, so allow it explicitly.
# (Frontend dev chạy ở origin khác nên cho phép tường minh.)
@app.after_request
def allow_dev_origin(response):
    response.headers["Access-Control-Allow-Origin"] = "*"
    response.headers["Access-Control-Allow-Headers"] = "*"
    response.headers["Access-Control-Allow-Methods"] = "*"
    return response


@app.get("/api/health")
def health():
    return jsonify(status="ok", service="@PROJECT@")


if __name__ == "__main__":
    host = os.environ.get("HOST", "@HOST@")
    port = int(os.environ.get("PORT", "@PORT@"))
    app.run(host=host, port=port)
"#;

/// Flask project manifest.
/// Manifest project Flask.
const FLASK_MANIFEST: &str = r#"# Project manifest generated by MagiCore for a split fullstack server half.
# Manifest project do MagiCore sinh cho nửa server của fullstack tách rời.
[project]
name = "@DISTRIBUTION@-server"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = ["flask>=3.0"]
"#;

/// Django management entrypoint.
/// Entrypoint quản lý Django.
const DJANGO_MANAGE: &str = r#"# Management entrypoint generated by MagiCore for a split fullstack project.
# Entrypoint quản lý do MagiCore sinh cho project fullstack tách rời.
import os
import sys


def main():
    os.environ.setdefault("DJANGO_SETTINGS_MODULE", "@MODULE@.settings")
    from django.core.management import execute_from_command_line

    execute_from_command_line(sys.argv)


if __name__ == "__main__":
    main()
"#;

/// Django settings module for a development server.
/// Module settings Django cho server phát triển.
const DJANGO_SETTINGS: &str = r#"# Settings generated by MagiCore for a split fullstack project.
# Settings do MagiCore sinh cho project fullstack tách rời.
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent

SECRET_KEY = "dev-only-replace-before-deploying"
DEBUG = True
ALLOWED_HOSTS = ["*"]

# The dev frontend runs on another origin; allow it explicitly.
# (Frontend dev chạy ở origin khác; cho phép tường minh.)
CORS_ALLOW_ALL_ORIGINS = True

INSTALLED_APPS = ["api"]

ROOT_URLCONF = "@MODULE@.urls"
TEMPLATES = [
    {
        "BACKEND": "django.template.backends.django.DjangoTemplates",
        "DIRS": [],
        "APP_DIRS": True,
        "OPTIONS": {"context_processors": []},
    }
]

DEFAULT_AUTO_FIELD = "django.db.models.AutoField"
"#;

/// Django URL configuration.
/// Cấu hình URL Django.
const DJANGO_URLS: &str = r#"# URL routes generated by MagiCore for a split fullstack project.
# Route URL do MagiCore sinh cho project fullstack tách rời.
from django.urls import path

from api import views

urlpatterns = [
    path("api/health", views.health),
]
"#;

/// Django API application package marker.
/// Marker package ứng dụng API Django.
const DJANGO_API_INIT: &str = r#"# API package generated by MagiCore for a split fullstack project.
# Package API do MagiCore sinh cho project fullstack tách rời.
"#;

/// Django API views.
/// View API Django.
const DJANGO_VIEWS: &str = r#"# Views generated by MagiCore for a split fullstack project.
# View do MagiCore sinh cho project fullstack tách rời.
from django.http import JsonResponse


def health(request):
    return JsonResponse({"status": "ok", "service": "@PROJECT@"})
"#;

/// Django project manifest.
/// Manifest project Django.
///
/// The health view needs no ORM, so the minimal project declares no database
/// and a single command starts it.
/// View health không cần ORM nên project tối thiểu không khai database và một
/// lệnh là khởi động được.
const DJANGO_MANIFEST: &str = r#"# Project manifest generated by MagiCore for a split fullstack server half.
# Manifest project do MagiCore sinh cho nửa server của fullstack tách rời.
[project]
name = "@DISTRIBUTION@-server"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = ["django>=5.0"]
"#;

/// Axum server entry.
/// Entry server Axum.
const AXUM_MAIN: &str = r#"// API server generated by MagiCore for a split fullstack project.
// Server API do MagiCore sinh cho project fullstack tách rời.
use std::env;

use axum::http::header;
use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

// The dev frontend runs on another origin; allow it explicitly.
// (Frontend dev chạy ở origin khác nên cho phép tường minh.)
const CORS_ALLOW_ALL: &str = "*";

async fn allow_dev_origin(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    for name in [
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        header::ACCESS_CONTROL_ALLOW_METHODS,
    ] {
        headers.insert(name, header::HeaderValue::from_static(CORS_ALLOW_ALL));
    }
    response
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "@PROJECT@" }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("HOST").unwrap_or_else(|_| String::from("@HOST@"));
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(@PORT@);
    let app = Router::new()
        .route("/api/health", get(health))
        .layer(axum::middleware::from_fn(allow_dev_origin));
    let listener = tokio::net::TcpListener::bind((host.as_str(), port)).await?;
    eprintln!("@PROJECT@ API listening on http://{host}:{port}");
    axum::serve(listener, app).await?;
    Ok(())
}
"#;

/// Axum Cargo manifest.
/// Manifest Cargo của Axum.
const AXUM_MANIFEST: &str = r#"# Cargo manifest generated by MagiCore for a split fullstack project.
# Manifest Cargo do MagiCore sinh cho project fullstack tách rời.
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "server"
path = "src/main.rs"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net"] }
serde_json = "1"
"#;

/// Gin server entry.
/// Entry server Gin.
const GIN_MAIN: &str = r#"// API server generated by MagiCore for a split fullstack project.
// Server API do MagiCore sinh cho project fullstack tách rời.
package main

import (
	"fmt"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
)

// The dev frontend runs on another origin; allow it explicitly.
// (Frontend dev chạy ở origin khác nên cho phép tường minh.)
const corsAllowAll = "*"

func allowDevOrigin() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Header("Access-Control-Allow-Origin", corsAllowAll)
		c.Header("Access-Control-Allow-Headers", corsAllowAll)
		c.Header("Access-Control-Allow-Methods", corsAllowAll)
		c.Next()
	}
}

func main() {
	router := gin.New()
	router.Use(allowDevOrigin())
	router.GET("/api/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"status": "ok", "service": "@PROJECT@"})
	})
	binding := fmt.Sprintf("%s:%s", os.Getenv("HOST"), os.Getenv("PORT"))
	fmt.Fprintf(os.Stderr, "@PROJECT@ API listening on %s\n", binding)
	if err := router.Run(binding); err != nil {
		panic(err)
	}
}
"#;

/// Gin module manifest.
/// Manifest module Gin.
const GIN_MOD: &str = r#"// Module manifest generated by MagiCore for a split fullstack project.
// Manifest module do MagiCore sinh cho project fullstack tách rời.
module server

go 1.22

require github.com/gin-gonic/gin v1.10.0
"#;

#[cfg(test)]
#[path = "test/native_backend.rs"]
mod tests;
