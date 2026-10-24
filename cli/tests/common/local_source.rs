//! Offline scaffold sources for lifecycle tests.
//! Nguồn scaffold offline cho test lifecycle.
//!
//! MagiCore never embeds framework source: a scaffold takes its files from the
//! registry layer or from a project that already exists. Tests therefore build
//! the smallest project that proves each framework identity and pass it through
//! `--from-local`, which exercises the offline branch the product promises.
//! MagiCore không nhúng source framework: scaffold lấy file từ registry layer
//! hoặc từ project đã tồn tại. Vì vậy test dựng project tối thiểu chứng minh
//! danh tính từng framework rồi truyền qua `--from-local`, đúng nhánh offline
//! mà sản phẩm cam kết.

use std::fs;
use std::path::{Path, PathBuf};

/// A source project for one core/framework pair.
/// Project nguồn cho một cặp core/framework.
pub struct LocalSource {
    root: PathBuf,
}

impl LocalSource {
    /// Path to pass as `--from-local=<path>`.
    /// Đường dẫn truyền vào `--from-local=<path>`.
    pub fn path(&self) -> &Path {
        &self.root
    }

    /// `--from-local=<path>` argument.
    /// Tham số `--from-local=<path>`.
    pub fn flag(&self) -> String {
        format!("--from-local={}", self.root.display())
    }
}

/// Build a reusable source project for `core`/`framework` under `parent`.
/// Dựng project nguồn tái dùng cho `core`/`framework` dưới `parent`.
pub fn local_source(parent: &Path, core: &str, framework: &str) -> LocalSource {
    let root = parent.join(format!("source-{core}-{framework}"));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create local source root");

    // The manifest proves identity; the marker files below prove the framework.
    // Manifest chứng minh danh tính; các file marker bên dưới chứng minh framework.
    fs::write(
        root.join("mgc.toml"),
        format!(
            "name = \"source-{core}-{framework}\"\nversion = \"0.1.0\"\necosystem = \"{core}\"\nframeworks = [\"{framework}\"]\n"
        ),
    )
    .expect("write source manifest");

    match (core, framework) {
        ("ai", _) => {
            fs::write(
                root.join("pyproject.toml"),
                "[build-system]\nrequires = [\"setuptools>=68\"]\nbuild-backend = \"setuptools.build_meta\"\n\n[project]\nname = \"agent\"\nversion = \"0.1.0\"\n",
            )
            .expect("write pyproject");
            fs::create_dir_all(root.join("src")).expect("create src");
            fs::write(
                root.join("src").join("agent.py"),
                "class AIAgent:\n    def run(self, mode: str) -> bool:\n        return True\n",
            )
            .expect("write agent module");
        }
        ("lib", "go") => {
            fs::write(
                root.join("go.mod"),
                "module example.com/source_lib\n\ngo 1.22\n",
            )
            .expect("write go.mod");
            fs::write(
                root.join("source_lib.go"),
                "package sourcelib\n\nfunc Answer() int {\n\treturn 42\n}\n",
            )
            .expect("write go source");
        }
        ("lib", "python") => {
            fs::write(
                root.join("pyproject.toml"),
                "[project]\nname = \"source_lib\"\nversion = \"0.1.0\"\n",
            )
            .expect("write pyproject");
        }
        ("lib", "typescript") | ("lib", "ts") => {
            fs::write(
                root.join("package.json"),
                "{\n  \"name\": \"source-lib\",\n  \"version\": \"0.1.0\",\n  \"main\": \"dist/index.js\"\n}\n",
            )
            .expect("write package.json");
            fs::write(
                root.join("tsconfig.json"),
                "{\n  \"compilerOptions\": {}\n}\n",
            )
            .expect("write tsconfig");
        }
        ("lib", "rust") => {
            fs::write(
                root.join("Cargo.toml"),
                "[package]\nname = \"source_lib\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"\n",
            )
            .expect("write Cargo.toml");
            fs::create_dir_all(root.join("src")).expect("create src");
            fs::write(
                root.join("src").join("lib.rs"),
                "pub fn answer() -> u32 {\n    42\n}\n",
            )
            .expect("write lib.rs");
        }
        ("app", "flutter") => {
            fs::write(
                root.join("pubspec.yaml"),
                "name: source_app\nversion: 0.1.0\nenvironment:\n  sdk: \">=3.0.0 <4.0.0\"\n",
            )
            .expect("write pubspec");
            fs::create_dir_all(root.join("lib")).expect("create lib");
            fs::write(
                root.join("lib").join("main.dart"),
                "void main() {\n  runApp(const Placeholder());\n}\n",
            )
            .expect("write main.dart");
        }
        ("web", "axum") => {
            fs::write(
                root.join("Cargo.toml"),
                "[package]\nname = \"webapp\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\naxum = \"0.8\"\nserde_json = \"1\"\ntokio = { version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }\n",
            )
            .expect("write Cargo.toml");
            fs::create_dir_all(root.join("src")).expect("create src");
            fs::write(
                root.join("src").join("main.rs"),
                "use axum::{routing::get, Json, Router};\nuse serde_json::{json, Value};\n\nasync fn health() -> Json<Value> {\n    Json(json!({\"status\": \"ok\"}))\n}\n\n#[tokio::main]\nasync fn main() {\n    let app = Router::new().route(\"/api/health\", get(health));\n    let listener = tokio::net::TcpListener::bind(\"0.0.0.0:4315\")\n        .await\n        .expect(\"bind HTTP listener\");\n    axum::serve(listener, app).await.expect(\"serve HTTP requests\");\n}\n",
            )
            .expect("write main.rs");
        }
        ("web", "fastapi") => {
            fs::write(
                root.join("pyproject.toml"),
                "[project]\nname = \"webapp\"\nversion = \"0.1.0\"\ndependencies = [\"fastapi\", \"uvicorn\"]\n",
            )
            .expect("write pyproject");
            fs::write(
                root.join("main.py"),
                "from fastapi import FastAPI\n\napp = FastAPI()\n\n\n@app.get(\"/\")\ndef root() -> dict[str, str]:\n    return {\"service\": \"webapp\"}\n\n\n@app.get(\"/api/health\")\ndef health() -> dict[str, str]:\n    return {\"status\": \"ok\"}\n",
            )
            .expect("write main.py");
        }
        ("cloud", "cdk-typescript") | ("cloud", "cdk") => {
            fs::write(
                root.join("package.json"),
                "{\n  \"name\": \"infra\",\n  \"private\": true,\n  \"version\": \"0.1.0\"\n}\n",
            )
            .expect("write package.json");
            fs::write(
                root.join("cdk.json"),
                "{\n  \"app\": \"npx ts-node bin/infra.ts\"\n}\n",
            )
            .expect("write cdk.json");
        }
        ("web", "actix-web") => {
            fs::write(
                root.join("Cargo.toml"),
                "[package]\nname = \"webapp\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nactix-web = \"4\"\n",
            )
            .expect("write Cargo.toml");
            fs::create_dir_all(root.join("src")).expect("create src");
            fs::write(
                root.join("src").join("main.rs"),
                "use actix_web::{get, App, HttpServer, Responder};\n\n#[get(\"/api/health\")]\nasync fn health() -> impl Responder {\n    \"{\\\"status\\\":\\\"ok\\\"}\"\n}\n\n#[actix_web::main]\nasync fn main() -> std::io::Result<()> {\n    HttpServer::new(|| App::new().service(health))\n        .bind((\"0.0.0.0\", 4315))?\n        .run()\n        .await\n}\n",
            )
            .expect("write main.rs");
        }
        ("web", "echo") => {
            fs::write(
                root.join("go.mod"),
                "module example.com/webapp\n\ngo 1.22\n\nrequire github.com/labstack/echo/v5 v5.4.0\n",
            )
            .expect("write go.mod");
            fs::write(
                root.join("main.go"),
                "package main\n\nimport \"github.com/labstack/echo/v5\"\n\nfunc main() {\n\te := echo.New()\n\te.GET(\"/api/health\", func(c echo.Context) error {\n\t\treturn c.JSON(200, map[string]string{\"status\": \"ok\"})\n\t})\n\te.Start(\":4315\")\n}\n",
            )
            .expect("write main.go");
        }
        ("web", "fiber") => {
            fs::write(
                root.join("go.mod"),
                "module example.com/webapp\n\ngo 1.22\n\nrequire github.com/gofiber/fiber/v3 v3.6.1\n",
            )
            .expect("write go.mod");
            fs::write(
                root.join("main.go"),
                "package main\n\nimport \"github.com/gofiber/fiber/v3\"\n\nfunc main() {\n\tapp := fiber.New()\n\tapp.Get(\"/api/health\", func(c fiber.Ctx) error {\n\t\treturn c.JSON(fiber.Map{\"status\": \"ok\"})\n\t})\n\tapp.Listen(\":4315\")\n}\n",
            )
            .expect("write main.go");
        }
        ("web", "gin") => {
            fs::write(
                root.join("go.mod"),
                "module example.com/webapp\n\ngo 1.22\n\nrequire github.com/gin-gonic/gin v1.12.0\n",
            )
            .expect("write go.mod");
            fs::write(
                root.join("main.go"),
                "package main\n\nimport \"github.com/gin-gonic/gin\"\n\nfunc main() {\n\tr := gin.Default()\n\tr.GET(\"/api/health\", func(c *gin.Context) {\n\t\tc.JSON(200, gin.H{\"status\": \"ok\"})\n\t})\n\tr.Run(\":4315\")\n}\n",
            )
            .expect("write main.go");
        }
        ("web", "flask") => {
            fs::write(
                root.join("pyproject.toml"),
                "[project]\nname = \"webapp\"\nversion = \"0.1.0\"\ndependencies = [\"flask>=3.0\"]\n",
            )
            .expect("write pyproject");
            fs::write(
                root.join("main.py"),
                "from flask import Flask\n\napp = Flask(__name__)\n\n\n@app.get(\"/\")\ndef root() -> dict[str, str]:\n    return {\"service\": \"webapp\"}\n\n\n@app.get(\"/api/health\")\ndef health() -> dict[str, str]:\n    return {\"status\": \"ok\"}\n\n\nif __name__ == \"__main__\":\n    app.run(host=\"0.0.0.0\", port=4315)\n",
            )
            .expect("write main.py");
        }
        ("web", "astro") => {
            fs::write(
                root.join("package.json"),
                "{\n  \"name\": \"webapp\",\n  \"version\": \"0.1.0\",\n  \"scripts\": {\n    \"dev\": \"astro dev\",\n    \"build\": \"astro build\"\n  },\n  \"dependencies\": {\n    \"astro\": \"^5.0.0\"\n  }\n}\n",
            )
            .expect("write package.json");
            fs::create_dir_all(root.join("src/pages")).expect("create pages");
            fs::write(
                root.join("src/pages/index.astro"),
                "---\nconst title = \"MagiCore + Astro\";\n---\n<html><body><h1>{title}</h1></body></html>\n",
            )
            .expect("write astro page");
        }
        ("web", "fastify") => {
            fs::write(
                root.join("package.json"),
                "{\n  \"name\": \"webapp\",\n  \"version\": \"0.1.0\",\n  \"scripts\": {\n    \"dev\": \"node server.js\",\n    \"build\": \"echo build\"\n  },\n  \"dependencies\": {\n    \"fastify\": \"^5.0.0\"\n  },\n  \"devDependencies\": {\n    \"tsx\": \"^4.0.0\"\n  }\n}\n",
            )
            .expect("write package.json");
            fs::create_dir_all(root.join("src")).expect("create src");
            fs::write(
                root.join("src/server.ts"),
                "import Fastify from \"fastify\";\n\nconst app = Fastify();\napp.get(\"/api/health\", async () => ({ status: \"ok\" }));\n",
            )
            .expect("write server");
            fs::write(
                root.join("src/index.ts"),
                "import Fastify from \"fastify\";\n\nconst app = Fastify();\napp.get(\"/api/health\", async () => ({ status: \"ok\" }));\napp.listen({ port: 3415, host: \"0.0.0.0\" });\n",
            )
            .expect("write entrypoint");
            write_node_runnable_server(&root);
        }
        ("web", "hono") => {
            fs::write(
                root.join("package.json"),
                "{\n  \"name\": \"webapp\",\n  \"version\": \"0.1.0\",\n  \"scripts\": {\n    \"dev\": \"node server.js\",\n    \"build\": \"echo build\"\n  },\n  \"dependencies\": {\n    \"hono\": \"^4.0.0\",\n    \"@hono/node-server\": \"^1.0.0\"\n  }\n}\n",
            )
            .expect("write package.json");
            fs::create_dir_all(root.join("src")).expect("create src");
            fs::write(
                root.join("src/index.ts"),
                "import { Hono } from \"hono\";\n\nconst app = new Hono();\napp.get(\"/api/health\", (c) => c.json({ status: \"ok\" }));\n",
            )
            .expect("write server");
            write_node_runnable_server(&root);
        }
        ("web", "trpc") => {
            fs::write(
                root.join("package.json"),
                "{\n  \"name\": \"webapp\",\n  \"version\": \"0.1.0\",\n  \"scripts\": {\n    \"dev\": \"node server.js\",\n    \"build\": \"echo build\"\n  },\n  \"dependencies\": {\n    \"@trpc/server\": \"^11.0.0\",\n    \"express\": \"^4.0.0\"\n  }\n}\n",
            )
            .expect("write package.json");
            fs::create_dir_all(root.join("src")).expect("create src");
            fs::write(
                root.join("src/index.ts"),
                "import express from \"express\";\nimport { createExpressMiddleware } from \"@trpc/server/adapters/express\";\n\nconst app = express();\napp.get(\"/api/health\", (_req, res) => res.json({ status: \"ok\" }));\napp.use(\"/trpc\", createExpressMiddleware({ router: {} as AppRouter }));\n\nexport type AppRouter = Record<string, never>;\n",
            )
            .expect("write server");
            write_node_runnable_server(&root);
        }
        ("web", _) | ("cloud", _) => {
            fs::write(
                root.join("package.json"),
                "{\n  \"name\": \"webapp\",\n  \"private\": true,\n  \"version\": \"0.1.0\",\n  \"scripts\": {\n    \"build\": \"echo build\",\n    \"dev\": \"echo dev\"\n  }\n}\n",
            )
            .expect("write package.json");
            fs::write(
                root.join("index.html"),
                "<!doctype html><title>webapp</title>\n",
            )
            .expect("write index.html");
        }
        _ => {}
    }

    LocalSource { root }
}

/// Write a dependency-free Node entrypoint so `mgc dev` can serve a request.
/// Ghi entrypoint Node không phụ thuộc để `mgc dev` phục vụ được request.
///
/// The framework manifest above still proves the framework identity; this file
/// only exists so the dev-server lifecycle test exercises the real spawn,
/// health probe and shutdown path instead of depending on an npm install.
/// Manifest framework phía trên vẫn chứng minh danh tính framework; file này
/// chỉ để test dev-server chạy đúng đường spawn, probe và shutdown thật thay vì
/// phụ thuộc npm install.
fn write_node_runnable_server(root: &Path) {
    fs::write(
        root.join("server.js"),
        "const http = require(\"node:http\");\nconst port = Number(process.env.PORT || 4315);\nhttp.createServer((req, res) => {\n  res.writeHead(200, { \"content-type\": \"application/json\" });\n  res.end(JSON.stringify({ path: req.url, status: \"ok\" }));\n}).listen(port, \"0.0.0.0\");\n",
    )
    .expect("write runnable server.js");
}
