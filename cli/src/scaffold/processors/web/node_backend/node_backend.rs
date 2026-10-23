//! Node/TypeScript Web backend starters (Fastify, Hono, tRPC).
//! Starter backend Web Node/TypeScript (Fastify, Hono, tRPC).

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;
use super::super::common::package_name;

const TSCONFIG: &str = "{\n  \"compilerOptions\": {\n    \"target\": \"ES2022\",\n    \"module\": \"NodeNext\",\n    \"moduleResolution\": \"NodeNext\",\n    \"rootDir\": \"./src\",\n    \"outDir\": \"dist\",\n    \"strict\": true,\n    \"esModuleInterop\": true,\n    \"skipLibCheck\": true,\n    \"types\": [\"node\"]\n  },\n  \"include\": [\"src/**/*.ts\"]\n}\n";

fn write_typescript_project(
    target: &Path,
    name: &str,
    entry: &str,
    scripts_dev: &str,
    source: &str,
) -> Result<()> {
    let manifest = serde_json::json!({
        "name": package_name(name),
        "private": true,
        "version": "0.1.0",
        "type": "module",
        "scripts": {
            "dev": scripts_dev,
            "build": "tsc -p tsconfig.json",
            "start": "node dist/index.js"
        }
    });
    write_file(
        &target.join("package.json"),
        &serde_json::to_string_pretty(&manifest)?,
    )?;
    write_file(&target.join("tsconfig.json"), TSCONFIG)?;
    write_file(&target.join("src").join(entry), source)
}

pub(crate) fn write_fastify(target: &Path, name: &str) -> Result<()> {
    write_typescript_project(
        target,
        name,
        "server.ts",
        "tsx watch src/server.ts",
        "import Fastify from 'fastify';\n\nconst app = Fastify({ logger: true });\n\napp.get('/', async () => ({ name: 'MagiCore Fastify', status: 'ok' }));\napp.get('/api/health', async () => ({ status: 'ok' }));\n\nconst port = Number(process.env.PORT ?? 4315);\nawait app.listen({ host: '0.0.0.0', port });\n",
    )
}

pub(crate) fn write_hono(target: &Path, name: &str) -> Result<()> {
    write_typescript_project(
        target,
        name,
        "index.ts",
        "tsx watch src/index.ts",
        "import { serve } from '@hono/node-server';\nimport { Hono } from 'hono';\n\nconst app = new Hono();\n\napp.get('/', (context) => context.json({ name: 'MagiCore Hono', status: 'ok' }));\napp.get('/api/health', (context) => context.json({ status: 'ok' }));\n\nserve({\n  fetch: app.fetch,\n  hostname: '0.0.0.0',\n  port: Number(process.env.PORT ?? 4315),\n});\n",
    )
}

pub(crate) fn write_trpc(target: &Path, name: &str) -> Result<()> {
    write_typescript_project(
        target,
        name,
        "index.ts",
        "tsx watch src/index.ts",
        "import express from 'express';\nimport { initTRPC } from '@trpc/server';\nimport { createExpressMiddleware } from '@trpc/server/adapters/express';\n\nconst t = initTRPC.create();\nconst appRouter = t.router({\n  health: t.procedure.query(() => ({ status: 'ok' })),\n});\nexport type AppRouter = typeof appRouter;\n\nconst app = express();\napp.get('/', (_request, response) => response.json({ name: 'MagiCore tRPC', status: 'ok' }));\napp.get('/api/health', (_request, response) => response.json({ status: 'ok' }));\napp.use('/api/trpc', createExpressMiddleware({ router: appRouter }));\n\nconst port = Number(process.env.PORT ?? 4315);\napp.listen(port, '0.0.0.0');\n",
    )
}
