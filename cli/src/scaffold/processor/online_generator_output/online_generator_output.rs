// Copy online generator output under strict bounds — sao chép output generator online theo giới hạn chặt.
// Bounded and link-safe generator output.
use super::*;

const MAX_GENERATED_TEMPLATE_FILES: usize = 20_000;
const MAX_GENERATED_TEMPLATE_BYTES: u64 = 256 * 1024 * 1024;
const MAX_GENERATED_TEMPLATE_DEPTH: usize = 128;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn copy_online_generated_template(
        source: &Path,
        target: &Path,
    ) -> Result<()> {
        fn copy_directory(
            source_root: &Path,
            source: &Path,
            target_root: &Path,
            relative: &Path,
            depth: usize,
            file_count: &mut usize,
            total_bytes: &mut u64,
        ) -> Result<()> {
            if depth > MAX_GENERATED_TEMPLATE_DEPTH {
                return Err(crate::scaffold::error::web_generator_output_too_large());
            }
            for entry in std::fs::read_dir(source)? {
                let entry = entry?;
                let name = entry.file_name();
                let Some(name) = name.to_str() else {
                    return Err(
                        crate::scaffold::error::template_materialization_path_invalid(
                            &entry.path().display().to_string(),
                        ),
                    );
                };
                if matches!(name, ".git" | "node_modules") {
                    continue;
                }
                let child_relative = relative.join(name);
                let child_relative_text = child_relative
                    .to_str()
                    .ok_or_else(|| {
                        crate::scaffold::error::template_materialization_path_invalid(
                            &entry.path().display().to_string(),
                        )
                    })?
                    .replace(std::path::MAIN_SEPARATOR, "/");
                let destination = checked_template_path(target_root, &child_relative_text)?;
                let metadata = std::fs::symlink_metadata(entry.path())?;
                let linked = metadata.file_type().is_symlink();
                #[cfg(windows)]
                let linked = {
                    use std::os::windows::fs::MetadataExt;
                    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
                    linked || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
                };
                if linked {
                    return Err(crate::error::web_generator_output_unsafe(&entry.path()));
                }
                if metadata.is_dir() {
                    std::fs::create_dir_all(&destination)?;
                    copy_directory(
                        source_root,
                        &entry.path(),
                        target_root,
                        &child_relative,
                        depth + 1,
                        file_count,
                        total_bytes,
                    )?;
                } else if metadata.is_file() {
                    *file_count = file_count.saturating_add(1);
                    *total_bytes = total_bytes.saturating_add(metadata.len());
                    if *file_count > MAX_GENERATED_TEMPLATE_FILES
                        || *total_bytes > MAX_GENERATED_TEMPLATE_BYTES
                    {
                        return Err(crate::scaffold::error::web_generator_output_too_large());
                    }
                    if let Some(parent) = destination.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::copy(entry.path(), destination)?;
                } else {
                    return Err(crate::error::web_generator_output_unsafe(&entry.path()));
                }
            }
            let canonical = source.canonicalize()?;
            if !canonical.starts_with(source_root) {
                return Err(crate::error::web_generator_output_unsafe(source));
            }
            Ok(())
        }

        let metadata = std::fs::symlink_metadata(source)?;
        let linked = metadata.file_type().is_symlink();
        #[cfg(windows)]
        let linked = {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
            linked || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
        };
        if linked || !metadata.is_dir() {
            return Err(crate::error::web_generator_output_unsafe(source));
        }
        let canonical_source = source.canonicalize()?;
        std::fs::create_dir_all(target)?;
        let mut file_count = 0;
        let mut total_bytes = 0;
        copy_directory(
            &canonical_source,
            &canonical_source,
            target,
            Path::new(""),
            0,
            &mut file_count,
            &mut total_bytes,
        )
    }
    pub(in crate::scaffold::processor) fn ensure_web_test_runner(
        target: &Path,
        name: &str,
    ) -> Result<()> {
        let package_path = target.join("package.json");
        let mut manifest = if package_path.is_file() {
            serde_json::from_slice::<serde_json::Value>(&std::fs::read(&package_path)?)?
        } else {
            serde_json::json!({
                "name": slugify(name),
                "private": true,
                "version": "0.1.0"
            })
        };
        let Some(package) = manifest.as_object_mut() else {
            return Err(crate::error::package_json_root_object());
        };
        let scripts = package
            .entry("scripts")
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
        let Some(scripts) = scripts.as_object_mut() else {
            return Err(crate::scaffold::error::package_json_scripts_object());
        };
        match scripts.get("test") {
            Some(serde_json::Value::String(_)) => return Ok(()),
            Some(_) => return Err(crate::scaffold::error::package_json_scripts_object()),
            None => {}
        }
        scripts.insert(
            "test".to_string(),
            serde_json::Value::String("node --test test/scaffold.test.cjs".to_string()),
        );
        // This file stays in the claimed staging tree until atomic publication.
        // File này ở trong cây staging đã claim cho tới khi publish nguyên tử.
        std::fs::write(&package_path, serde_json::to_vec_pretty(&manifest)?)?;

        let smoke_test = target.join("test/scaffold.test.cjs");
        if !smoke_test.exists() {
            Self::write_file(
                &smoke_test,
                "const test = require('node:test');\nconst assert = require('node:assert/strict');\nconst fs = require('node:fs');\nconst path = require('node:path');\n\nconst root = path.resolve(__dirname, '..');\nconst htmlPath = path.join(root, 'index.html');\nif (fs.existsSync(htmlPath)) {\n  test('HTML document has valid local script entries', () => {\n    const html = fs.readFileSync(htmlPath, 'utf8');\n    assert.match(html, /<html\\b/i);\n    for (const [, entry] of html.matchAll(/<script[^>]+src=[\\\"']([^\\\"']+)[\\\"']/gi)) {\n      const url = new URL(entry, 'http://magicore.local');\n      if (url.origin !== 'http://magicore.local') continue;\n      const relative = decodeURIComponent(url.pathname).replace(/^\\/+/, '');\n      const localPath = path.resolve(root, relative);\n      assert.ok(localPath === root || localPath.startsWith(root + path.sep), 'script entry escapes project: ' + entry);\n      assert.ok(fs.existsSync(localPath), 'missing local script entry: ' + entry);\n    }\n  });\n} else {\n  const frameworkEntries = ['app.vue', 'app/app.vue', 'pages/index.vue', 'src/pages/index.astro', 'src/routes/+page.svelte', 'src/root.tsx', 'app/root.tsx', 'app/page.tsx', 'app/page.jsx', 'src/app/page.jsx', 'app/routes/_index.tsx'];\n  const sourceEntries = ['src/server', 'src/index', 'src/main', 'src/app', 'server', 'index', 'main', 'app'].flatMap((base) => ['.js', '.mjs', '.cjs', '.ts', '.mts', '.cts'].map((extension) => base + extension));\n  const entry = frameworkEntries.concat(sourceEntries).map((file) => path.join(root, file)).find((file) => fs.existsSync(file));\n  test('web project contains a source entrypoint', () => {\n    assert.ok(entry, 'no framework source entrypoint found');\n    assert.ok(fs.readFileSync(entry, 'utf8').trim().length > 0, 'framework source entrypoint is empty');\n  });\n}\n",
            )?;
        }
        Ok(())
    }
}
