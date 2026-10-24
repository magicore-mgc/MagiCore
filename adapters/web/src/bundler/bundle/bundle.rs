//! esbuild-rs operation for a Web project.
//! Thao tác esbuild-rs cho dự án Web.

use super::super::types::{BundleResult, BundlerConfig};

/// Runs the native frontend bundler with the established project settings.
/// Chạy frontend bundler native với thiết lập dự án hiện hành.
pub struct Bundler {
    config: BundlerConfig,
}

impl Bundler {
    /// Creates a bundler for one Web build request.
    /// Tạo bundler cho một yêu cầu build Web.
    pub fn new(config: BundlerConfig) -> Self {
        Self { config }
    }

    /// Builds the configured entry and returns the emitted byte count.
    /// Build entry đã cấu hình và trả số byte output.
    pub async fn bundle(&self) -> Result<BundleResult, anyhow::Error> {
        std::fs::create_dir_all(&self.config.output_dir)?;
        let prepared = super::super::workspace::prepare_workspace(&self.config.entry)?;

        let mut builder = esbuild_rs::BuildOptionsBuilder::new();
        builder.entry_points = vec![prepared.entry.to_string_lossy().to_string()];
        builder.bundle = true;
        builder.outdir = self.config.output_dir.to_string_lossy().to_string();
        builder.abs_working_dir = prepared.working_dir.to_string_lossy().to_string();
        builder.preserve_symlinks = false;
        if self.config.backend {
            // A backend bundle runs on Node: marking `node:*` external is what
            // keeps esbuild from trying to inline runtime builtins.
            // Bundle backend chạy trên Node: đánh dấu `node:*` là external để
            // esbuild không cố nhúng builtin runtime.
            builder.platform = esbuild_rs::Platform::Node;
            builder.external = vec!["node:*".to_string()];
            builder.main_fields = vec!["module".to_string(), "main".to_string()];
        } else {
            builder.platform = esbuild_rs::Platform::Browser;
            builder.main_fields = vec![
                "browser".to_string(),
                "module".to_string(),
                "main".to_string(),
            ];
        }
        builder.format = esbuild_rs::Format::ESModule;
        builder.resolve_extensions = vec![
            ".tsx".to_string(),
            ".ts".to_string(),
            ".jsx".to_string(),
            ".js".to_string(),
            ".css".to_string(),
            ".json".to_string(),
        ];
        builder
            .loader
            .insert(".svg".to_string(), esbuild_rs::Loader::DataURL);
        builder
            .loader
            .insert(".png".to_string(), esbuild_rs::Loader::DataURL);
        builder
            .loader
            .insert(".jpg".to_string(), esbuild_rs::Loader::DataURL);
        builder
            .loader
            .insert(".jpeg".to_string(), esbuild_rs::Loader::DataURL);
        builder
            .loader
            .insert(".gif".to_string(), esbuild_rs::Loader::DataURL);
        builder
            .loader
            .insert(".webp".to_string(), esbuild_rs::Loader::DataURL);
        builder
            .loader
            .insert(".ico".to_string(), esbuild_rs::Loader::File);
        builder.target = target_from_str(&self.config.target);
        builder.minify_whitespace = self.config.minify;
        builder.minify_identifiers = self.config.minify;
        builder.minify_syntax = self.config.minify;
        builder.source_map = if self.config.sourcemap {
            esbuild_rs::SourceMap::Linked
        } else {
            esbuild_rs::SourceMap::None
        };
        builder.public_path = self.config.public_path.clone();
        builder.write = true;

        let options = builder.build();
        let result = esbuild_rs::build(options).await;

        if !result.errors.as_slice().is_empty() {
            let messages: Vec<String> = result
                .errors
                .as_slice()
                .iter()
                .map(|error| error.to_string())
                .collect();
            return Err(super::super::error::esbuild_failed(&messages));
        }

        let size: usize = result
            .output_files
            .as_slice()
            .iter()
            .map(|file| file.data.as_str().len())
            .sum();

        Ok(BundleResult { size })
    }
}

/// Preserves the build's explicit target table and historical ES2020 fallback.
/// Giữ bảng target tường minh và fallback ES2020 hiện hành.
fn target_from_str(target: &str) -> esbuild_rs::Target {
    match target {
        "es5" => esbuild_rs::Target::ES5,
        "es2015" => esbuild_rs::Target::ES2015,
        "es2016" => esbuild_rs::Target::ES2016,
        "es2017" => esbuild_rs::Target::ES2017,
        "es2018" => esbuild_rs::Target::ES2018,
        "es2019" => esbuild_rs::Target::ES2019,
        "es2020" => esbuild_rs::Target::ES2020,
        "es2021" => esbuild_rs::Target::ES2021,
        "esnext" => esbuild_rs::Target::ESNext,
        _ => esbuild_rs::Target::ES2020,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
