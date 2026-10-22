//! Native Web backend starters — scaffold backend Web bằng processor native.

use std::path::Path;

use anyhow::{Result, bail};

use super::write_file;

pub struct WebProcessor;

impl WebProcessor {
    pub(crate) fn supports_native_project(sub_type: &str, framework: &str) -> bool {
        matches!(
            (sub_type, framework),
            (
                "backend",
                "axum"
                    | "actix-web"
                    | "echo"
                    | "fastapi"
                    | "fastify"
                    | "fiber"
                    | "flask"
                    | "gin"
                    | "hono"
                    | "trpc"
            ) | ("frontend", "astro")
        )
    }

    pub(crate) fn write_project(
        target: &Path,
        name: &str,
        sub_type: &str,
        framework: &str,
    ) -> Result<()> {
        match (sub_type, framework) {
            (
                "backend",
                "axum" | "actix-web" | "echo" | "fastapi" | "fastify" | "fiber" | "flask" | "gin"
                | "hono" | "trpc",
            ) => Self::write_backend(target, name, framework),
            ("frontend", "astro") => Self::write_astro_frontend(target, name),
            _ => bail!("unsupported native Web project: {sub_type}/{framework}"),
        }?;
        write_file(
            &target.join(".gitignore"),
            "/node_modules/\n/target/\n/dist/\n.env\n",
        )?;
        write_file(
            &target.join("README.md"),
            &format!("# {name}\n\n{framework} project scaffolded by MagiCore.\n"),
        )
    }

    fn write_backend(target: &Path, name: &str, framework: &str) -> Result<()> {
        let package_name = package_name(name);
        match framework {
            "echo" | "fiber" | "gin" => Self::write_go_backend(target, name, framework),
            "fastify" => Self::write_fastify_backend(target, name),
            "hono" => Self::write_hono_backend(target, name),
            "trpc" => Self::write_trpc_backend(target, name),
            "actix-web" => Self::write_actix_backend(target, name),
            "axum" => {
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
            "fastapi" => {
                write_file(
                    &target.join("pyproject.toml"),
                    &format!(
                        "[project]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nrequires-python = \">=3.12\"\ndependencies = [\"fastapi>=0.115,<1\", \"uvicorn[standard]>=0.30,<1\"]\n"
                    ),
                )?;
                write_file(
                    &target.join("main.py"),
                    "from fastapi import FastAPI\n\napp = FastAPI()\n\n\n@app.get(\"/api/health\")\ndef health() -> dict[str, str]:\n    return {\"status\": \"ok\"}\n",
                )
            }
            "flask" => {
                write_file(
                    &target.join("pyproject.toml"),
                    &format!(
                        "[project]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nrequires-python = \">=3.12\"\ndependencies = [\"flask>=3.0\"]\n"
                    ),
                )?;
                write_file(
                    &target.join("main.py"),
                    "import os\n\nfrom flask import Flask, jsonify\n\napp = Flask(__name__)\n\n\n@app.get(\"/\")\ndef index():\n    return jsonify({\"name\": \"MagiCore Flask\", \"status\": \"ok\"})\n\n\n@app.get(\"/api/health\")\ndef health():\n    return jsonify({\"status\": \"ok\"})\n\n\nif __name__ == \"__main__\":\n    app.run(host=\"0.0.0.0\", port=int(os.environ.get(\"PORT\", \"4315\")))\n",
                )
            }
            _ => bail!("unsupported native Web backend framework: {framework}"),
        }
    }

    fn write_go_backend(target: &Path, name: &str, framework: &str) -> Result<()> {
        let module_name = format!("example.com/{}", package_name(name));
        let (module, version, source) = match framework {
            "echo" => (
                "github.com/labstack/echo/v5",
                "v5.4.0",
                "package main\n\nimport (\n\t\"net/http\"\n\t\"os\"\n\n\t\"github.com/labstack/echo/v5\"\n)\n\nfunc main() {\n\tapp := echo.New()\n\tapp.GET(\"/\", func(c *echo.Context) error {\n\t\treturn c.JSON(http.StatusOK, map[string]string{\"name\": \"MagiCore Echo\", \"status\": \"ok\"})\n\t})\n\tapp.GET(\"/api/health\", func(c *echo.Context) error {\n\t\treturn c.JSON(http.StatusOK, map[string]string{\"status\": \"ok\"})\n\t})\n\tport := os.Getenv(\"PORT\")\n\tif port == \"\" {\n\t\tport = \"4315\"\n\t}\n\tif err := app.Start(\"0.0.0.0:\" + port); err != nil && err != http.ErrServerClosed {\n\t\tpanic(err)\n\t}\n}\n",
            ),
            "fiber" => (
                "github.com/gofiber/fiber/v3",
                "v3.6.1",
                "package main\n\nimport (\n\t\"os\"\n\n\t\"github.com/gofiber/fiber/v3\"\n)\n\nfunc main() {\n\tapp := fiber.New()\n\tapp.Get(\"/\", func(c fiber.Ctx) error {\n\t\treturn c.JSON(map[string]string{\"name\": \"MagiCore Fiber\", \"status\": \"ok\"})\n\t})\n\tapp.Get(\"/api/health\", func(c fiber.Ctx) error {\n\t\treturn c.JSON(map[string]string{\"status\": \"ok\"})\n\t})\n\tport := os.Getenv(\"PORT\")\n\tif port == \"\" {\n\t\tport = \"4315\"\n\t}\n\tif err := app.Listen(\"0.0.0.0:\" + port); err != nil {\n\t\tpanic(err)\n\t}\n}\n",
            ),
            "gin" => (
                "github.com/gin-gonic/gin",
                "v1.12.0",
                "package main\n\nimport (\n\t\"net/http\"\n\t\"os\"\n\n\t\"github.com/gin-gonic/gin\"\n)\n\nfunc main() {\n\tapp := gin.New()\n\tapp.Use(gin.Logger(), gin.Recovery())\n\tapp.GET(\"/\", func(c *gin.Context) {\n\t\tc.JSON(http.StatusOK, gin.H{\"name\": \"MagiCore Gin\", \"status\": \"ok\"})\n\t})\n\tapp.GET(\"/api/health\", func(c *gin.Context) {\n\t\tc.JSON(http.StatusOK, gin.H{\"status\": \"ok\"})\n\t})\n\tport := os.Getenv(\"PORT\")\n\tif port == \"\" {\n\t\tport = \"4315\"\n\t}\n\tif err := app.Run(\"0.0.0.0:\" + port); err != nil {\n\t\tpanic(err)\n\t}\n}\n",
            ),
            _ => bail!("unsupported native Go Web framework: {framework}"),
        };
        write_file(
            &target.join("go.mod"),
            &format!("module {module_name}\n\ngo 1.26.0\n\nrequire {module} {version}\n"),
        )?;
        write_file(&target.join("main.go"), source)
    }

    fn write_actix_backend(target: &Path, name: &str) -> Result<()> {
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
        )?;
        Ok(())
    }

    fn write_fastify_backend(target: &Path, name: &str) -> Result<()> {
        let manifest = serde_json::json!({
            "name": package_name(name),
            "private": true,
            "version": "0.1.0",
            "type": "module",
            "scripts": {
                "dev": "tsx watch src/server.ts",
                "build": "tsc -p tsconfig.json",
                "start": "node dist/server.js"
            }
        });
        write_file(
            &target.join("package.json"),
            &serde_json::to_string_pretty(&manifest)?,
        )?;
        write_file(
            &target.join("tsconfig.json"),
            "{\n  \"compilerOptions\": {\n    \"target\": \"ES2022\",\n    \"module\": \"NodeNext\",\n    \"moduleResolution\": \"NodeNext\",\n    \"rootDir\": \"./src\",\n    \"outDir\": \"dist\",\n    \"strict\": true,\n    \"esModuleInterop\": true,\n    \"skipLibCheck\": true,\n    \"types\": [\"node\"]\n  },\n  \"include\": [\"src/**/*.ts\"]\n}\n",
        )?;
        write_file(
            &target.join("src/server.ts"),
            "import Fastify from 'fastify';\n\nconst app = Fastify({ logger: true });\n\napp.get('/', async () => ({ name: 'MagiCore Fastify', status: 'ok' }));\napp.get('/api/health', async () => ({ status: 'ok' }));\n\nconst port = Number(process.env.PORT ?? 4315);\nawait app.listen({ host: '0.0.0.0', port });\n",
        )
    }

    fn write_hono_backend(target: &Path, name: &str) -> Result<()> {
        let manifest = serde_json::json!({
            "name": package_name(name),
            "private": true,
            "version": "0.1.0",
            "type": "module",
            "scripts": {
                "dev": "tsx watch src/index.ts",
                "build": "tsc -p tsconfig.json",
                "start": "node dist/index.js"
            }
        });
        write_file(
            &target.join("package.json"),
            &serde_json::to_string_pretty(&manifest)?,
        )?;
        write_file(
            &target.join("tsconfig.json"),
            "{\n  \"compilerOptions\": {\n    \"target\": \"ES2022\",\n    \"module\": \"NodeNext\",\n    \"moduleResolution\": \"NodeNext\",\n    \"rootDir\": \"./src\",\n    \"outDir\": \"dist\",\n    \"strict\": true,\n    \"esModuleInterop\": true,\n    \"skipLibCheck\": true,\n    \"types\": [\"node\"]\n  },\n  \"include\": [\"src/**/*.ts\"]\n}\n",
        )?;
        write_file(
            &target.join("src/index.ts"),
            "import { serve } from '@hono/node-server';\nimport { Hono } from 'hono';\n\nconst app = new Hono();\n\napp.get('/', (context) => context.json({ name: 'MagiCore Hono', status: 'ok' }));\napp.get('/api/health', (context) => context.json({ status: 'ok' }));\n\nserve({\n  fetch: app.fetch,\n  hostname: '0.0.0.0',\n  port: Number(process.env.PORT ?? 4315),\n});\n",
        )
    }

    fn write_trpc_backend(target: &Path, name: &str) -> Result<()> {
        let manifest = serde_json::json!({
            "name": package_name(name),
            "private": true,
            "version": "0.1.0",
            "type": "module",
            "scripts": {
                "dev": "tsx watch src/index.ts",
                "build": "tsc -p tsconfig.json",
                "start": "node dist/index.js"
            }
        });
        write_file(
            &target.join("package.json"),
            &serde_json::to_string_pretty(&manifest)?,
        )?;
        write_file(
            &target.join("tsconfig.json"),
            "{\n  \"compilerOptions\": {\n    \"target\": \"ES2022\",\n    \"module\": \"NodeNext\",\n    \"moduleResolution\": \"NodeNext\",\n    \"rootDir\": \"./src\",\n    \"outDir\": \"dist\",\n    \"strict\": true,\n    \"esModuleInterop\": true,\n    \"skipLibCheck\": true,\n    \"types\": [\"node\"]\n  },\n  \"include\": [\"src/**/*.ts\"]\n}\n",
        )?;
        write_file(
            &target.join("src/index.ts"),
            "import express from 'express';\nimport { initTRPC } from '@trpc/server';\nimport { createExpressMiddleware } from '@trpc/server/adapters/express';\n\nconst t = initTRPC.create();\nconst appRouter = t.router({\n  health: t.procedure.query(() => ({ status: 'ok' })),\n});\nexport type AppRouter = typeof appRouter;\n\nconst app = express();\napp.get('/', (_request, response) => response.json({ name: 'MagiCore tRPC', status: 'ok' }));\napp.get('/api/health', (_request, response) => response.json({ status: 'ok' }));\napp.use('/api/trpc', createExpressMiddleware({ router: appRouter }));\n\nconst port = Number(process.env.PORT ?? 4315);\napp.listen(port, '0.0.0.0');\n",
        )
    }

    fn write_astro_frontend(target: &Path, name: &str) -> Result<()> {
        let manifest = serde_json::json!({
            "name": package_name(name),
            "private": true,
            "version": "0.1.0",
            "type": "module",
            "scripts": {
                "dev": "astro dev",
                "build": "astro build",
                "preview": "astro preview"
            }
        });
        write_file(
            &target.join("package.json"),
            &serde_json::to_string_pretty(&manifest)?,
        )?;
        write_file(
            &target.join("astro.config.mjs"),
            "import { defineConfig } from 'astro/config';\n\nexport default defineConfig({});\n",
        )?;
        write_file(
            &target.join("src/pages/index.astro"),
            "---\nconst title = 'MagiCore + Astro';\n---\n<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    <title>{title}</title>\n  </head>\n  <body>\n    <main><h1>{title}</h1><p>Your Astro project is ready.</p></main>\n  </body>\n</html>\n",
        )
    }
}

fn package_name(name: &str) -> String {
    let mut result = String::new();
    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            result.push(character.to_ascii_lowercase());
        } else if !result.is_empty() && !result.ends_with('-') {
            result.push('-');
        }
    }
    let result = result.trim_matches('-').to_string();
    if result.is_empty() {
        "mgc-web-app".to_string()
    } else {
        result
    }
}
