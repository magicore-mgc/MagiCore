//! Files and scripts of the Node.js server half in a split fullstack combo.
//! File và script của nửa server Node.js trong combo fullstack tách rời.
//!
//! Only Node.js servers whose starter MagiCore can describe completely are
//! listed here. Everything else has no half, so the caller reports the honest
//! two-half message instead of writing a server that cannot start.
//! Chỉ server Node.js mà MagiCore mô tả trọn vẹn starter mới có mặt ở đây.
//! Backend khác không có nửa nào, để caller báo đúng thông điệp hai nửa thay vì
//! ghi ra server không khởi động được.

use std::path::Path;

use anyhow::Result;

/// Default port the generated server listens on.
/// Cổng mặc định server sinh ra lắng nghe.
///
/// Fixed by the repository port convention (RULE §13: every port carries the
/// digits 4·3·1·5 exactly once). `PORT` in the environment still overrides it.
/// Cố định theo quy ước port của repo (§13: mọi port chứa đủ 4·3·1·5 đúng một
/// lần). Biến `PORT` trong môi trường vẫn ghi đè.
const SERVER_PORT: &str = "1543";

/// Directory that holds the server half inside the project.
/// Thư mục chứa nửa server trong project.
///
/// `detect_project_mode` treats a backend manifest under `server/` as
/// `FullstackSplit`, so this name is part of the project contract.
/// `detect_project_mode` coi manifest backend dưới `server/` là
/// `FullstackSplit`, nên tên này thuộc contract của project.
const SERVER_DIR: &str = "server";

/// Port of the file-name placeholders used by the templates below.
/// Placeholder tên file dùng trong template bên dưới.
const PROJECT_PLACEHOLDER: &str = "@PROJECT@";

/// One backend half: entry file, start script, and dependency sets.
/// Mỗi nửa backend: file entry, script start và các bộ dependency.
struct BackendHalf {
    /// Entry file (TypeScript) — file entry (TypeScript).
    typescript_entry: &'static str,
    /// Entry file (JavaScript) — file entry (JavaScript).
    javascript_entry: &'static str,
    /// Script body for `npm start` (TypeScript) — nội dung script `npm start`.
    typescript_start: &'static str,
    /// Script body for `npm start` (JavaScript) — nội dung script `npm start`.
    javascript_start: &'static str,
    /// Runtime dependencies with their catalog version ranges.
    /// Dependency runtime kèm range phiên bản trong catalog.
    dependencies: &'static [(&'static str, &'static str)],
    /// TypeScript development dependencies.
    /// Dev dependency cho TypeScript.
    dev_dependencies: &'static [(&'static str, &'static str)],
    /// Template rendered for a TypeScript project.
    /// Template sinh cho project TypeScript.
    typescript_source: &'static str,
    /// Template rendered for a JavaScript project.
    /// Template sinh cho project JavaScript.
    javascript_source: &'static str,
}

impl BackendHalf {
    /// Entry path for the selected language — path entry theo ngôn ngữ chọn.
    fn entry(&self, typescript: bool) -> &'static str {
        if typescript {
            self.typescript_entry
        } else {
            self.javascript_entry
        }
    }

    /// `npm start` script for the selected language.
    /// Script `npm start` theo ngôn ngữ chọn.
    fn start(&self, typescript: bool) -> &'static str {
        if typescript {
            self.typescript_start
        } else {
            self.javascript_start
        }
    }

    /// Source template for the selected language.
    /// Template source theo ngôn ngữ chọn.
    fn source(&self, typescript: bool) -> &'static str {
        if typescript {
            self.typescript_source
        } else {
            self.javascript_source
        }
    }
}

/// Catalog of composable backend halves.
/// Danh mục các nửa backend ghép được.
///
/// Fetching a range and locking it is `mgc install`'s job in the server
/// directory; the scaffold only declares ranges.
/// Lấy range và lock là việc của `mgc install` trong thư mục server; scaffold
/// chỉ khai range.
fn catalog(backend: &str) -> Option<BackendHalf> {
    match backend {
        "express" => Some(BackendHalf {
            typescript_entry: "server/src/server.ts",
            javascript_entry: "server/src/server.js",
            typescript_start: "node --experimental-strip-types src/server.ts",
            javascript_start: "node src/server.js",
            dependencies: &[("express", "^4.19.2"), ("cors", "^2.8.5")],
            dev_dependencies: &[("typescript", "^5.5.0"), ("@types/node", "^20.14.0")],
            typescript_source: EXPRESS_TS,
            javascript_source: EXPRESS_JS,
        }),
        _ => None,
    }
}

/// Whether MagiCore ships an offline starter for `backend`.
/// MagiCore có sẵn starter offline cho `backend` hay không.
///
/// The composer asks this before writing anything; the refusal message for a
/// backend nobody handles belongs to the composer, so exactly one message names
/// both halves.
/// Composer hỏi điều này trước khi ghi bất cứ gì; thông điệp từ chối cho backend
/// không owner nào xử lý thuộc composer, nên chỉ có đúng một thông điệp nêu rõ
/// cả hai nửa.
pub(super) fn has_half(backend: &str) -> bool {
    catalog(backend).is_some()
}

/// Create the server half of `combo` (embedded backend `backend`) in `<root>/server`.
/// Tạo nửa server của `combo` (backend nhúng `backend`) trong `<root>/server`.
///
/// The project directory was just scaffolded by this same create, so every
/// write lands in a tree this process owns; this is a creation-only writer, not
/// an edit of an existing project. All inputs are validated first, so a refused
/// half leaves no partial server behind.
/// Thư mục project vừa do chính lệnh create này scaffold, nên mọi lần ghi đều
/// nằm trong cây do tiến trình này sở hữu; đây là writer tạo mới, không phải
/// chỉnh sửa project đã có. Mọi đầu vào được kiểm trước, nên nửa bị từ chối
/// không để lại server dở.
pub(super) fn write(root: &Path, backend: &str, combo: &str, typescript: bool) -> Result<()> {
    let half = catalog(backend).ok_or_else(|| no_half(combo, backend))?;
    let entry = root.join(half.entry(typescript));
    let Some(entry_parent) = entry.parent() else {
        return Err(no_half(combo, backend));
    };
    let name = crate::scaffold::processor::Scaffolder::display_name(root);
    // Render every byte before touching the filesystem.
    // (Kết xuất toàn bộ nội dung trước khi chạm filesystem.)
    let rendered_entry = half
        .source(typescript)
        .replace(PROJECT_PLACEHOLDER, &name)
        .replace("@PORT@", SERVER_PORT);
    let rendered_manifest = manifest_source(&half, &name, typescript);

    std::fs::create_dir_all(entry_parent)?;
    std::fs::write(&entry, rendered_entry)?;
    std::fs::write(
        root.join(SERVER_DIR).join("package.json"),
        rendered_manifest,
    )?;
    std::fs::write(root.join(".env.example"), format!("PORT={SERVER_PORT}\n"))?;
    Ok(())
}

/// Text of the server manifest, written by [`write`] into the tree it owns.
/// Nội dung manifest server, do [`write`] ghi vào cây nó sở hữu.
///
/// It carries no placeholders: `express` resolves them as real dependency names.
/// Không kèm placeholder vì `express` coi chúng là tên dependency thật.
fn manifest_source(half: &BackendHalf, name: &str, typescript: bool) -> String {
    let dependencies: serde_json::Map<String, serde_json::Value> = half
        .dependencies
        .iter()
        .map(|(package, range)| ((*package).to_string(), (*range).into()))
        .collect();
    let mut manifest = serde_json::json!({
        "name": format!("{name}-server"),
        "private": true,
        "version": "0.1.0",
        "type": "module",
        "scripts": { "start": half.start(typescript) },
        "dependencies": dependencies,
    });
    if typescript {
        let dev: serde_json::Map<String, serde_json::Value> = half
            .dev_dependencies
            .iter()
            .map(|(package, range)| ((*package).to_string(), (*range).into()))
            .collect();
        manifest["devDependencies"] = serde_json::Value::Object(dev);
    }
    let rendered = serde_json::to_string_pretty(&manifest)
        .unwrap_or_else(|_| String::from("{\"private\":true}"));
    format!("{rendered}\n")
}

/// Error for a half MagiCore cannot scaffold offline.
/// Lỗi cho nửa mà MagiCore không scaffold offline được.
fn no_half(combo: &str, backend: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "the '{combo}' combo needs both halves and MagiCore has no offline generator for its \
         '{backend}' server half. Scaffold each half with `mgc create-web <frontend> <name>` and \
         `mgc create-web {backend} <name>`, or supply the combo through the registry layer / \
         `--from-local <path>`. Nothing was created."
    )
}

/// Express starter for a JavaScript project.
/// Starter Express cho project JavaScript.
const EXPRESS_JS: &str = r#"// API server generated by MagiCore for a split fullstack project.
// Server API do MagiCore sinh cho project fullstack tách rời.
import express from "express";
import cors from "cors";

const app = express();
const port = Number(process.env.PORT ?? @PORT@);

app.use(cors());
app.use(express.json());

app.get("/api/health", (_request, response) => {
  response.json({ status: "ok", service: "@PROJECT@" });
});

app.listen(port, () => {
  // stderr keeps stdout clean for tooling that reads it.
  // ghi stderr để stdout trống cho công cụ đọc nó.
  console.error(`@PROJECT@ API listening on http://localhost:${port}`);
});
"#;

/// Express starter for a TypeScript project.
/// Starter Express cho project TypeScript.
const EXPRESS_TS: &str = r#"// API server generated by MagiCore for a split fullstack project.
// Server API do MagiCore sinh cho project fullstack tách rời.
import express, { type Express, type Request, type Response } from "express";
import cors from "cors";

const app: Express = express();
const port: number = Number(process.env.PORT ?? @PORT@);

app.use(cors());
app.use(express.json());

app.get("/api/health", (_request: Request, response: Response) => {
  response.json({ status: "ok", service: "@PROJECT@" });
});

app.listen(port, () => {
  // stderr keeps stdout clean for tooling that reads it.
  // ghi stderr để stdout trống cho công cụ đọc nó.
  console.error(`@PROJECT@ API listening on http://localhost:${port}`);
});
"#;
