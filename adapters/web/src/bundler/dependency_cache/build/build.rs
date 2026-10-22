//! Browser ESM dependency bundle operation.
//! Thao tác bundle dependency thành browser ESM.

use std::path::Path;

use super::super::{error::BundleError, types::CachedDep};
use super::metafile_inputs::validate_metafile_inputs;

/// Bundle one resolved package entry and return its JavaScript/CSS outputs.
/// Bundle một entry package đã tìm được và trả output JavaScript/CSS.
pub(in crate::bundler::dependency_cache) async fn bundle_dependency(
    entry: &Path,
    working_dir: &str,
    allowed_roots: &[std::path::PathBuf],
) -> Result<CachedDep, BundleError> {
    let mut builder = esbuild_rs::BuildOptionsBuilder::new();
    builder.entry_points = vec![entry.to_string_lossy().to_string()];
    builder.bundle = true;
    builder.abs_working_dir = working_dir.to_string();
    builder.platform = esbuild_rs::Platform::Browser;
    builder.format = esbuild_rs::Format::ESModule;
    builder.write = false;
    builder.metafile = true;
    builder.resolve_extensions = [".js", ".ts", ".jsx", ".tsx", ".json", ".css"]
        .into_iter()
        .map(str::to_string)
        .collect();
    builder.main_fields = ["browser", "module", "main"]
        .into_iter()
        .map(str::to_string)
        .collect();
    builder.minify_whitespace = false;
    builder.minify_syntax = true;
    builder.minify_identifiers = false;

    let result = esbuild_rs::build(builder.build()).await;
    if !result.errors.as_slice().is_empty() {
        let messages = result
            .errors
            .as_slice()
            .iter()
            .map(|error| error.to_string())
            .collect();
        return Err(BundleError::Esbuild(messages));
    }

    validate_metafile_inputs(
        result.metafile.as_str(),
        Path::new(working_dir),
        allowed_roots,
    )
    .map_err(BundleError::MetafileInputs)?;

    let js = result
        .output_files
        .as_slice()
        .iter()
        .find(|file| file.path.as_str().ends_with(".js"))
        .or_else(|| result.output_files.as_slice().first())
        .map(|file| file.data.as_str().to_string())
        .unwrap_or_default();
    let css = result
        .output_files
        .as_slice()
        .iter()
        .find(|file| file.path.as_str().ends_with(".css"))
        .map(|file| file.data.as_str().to_string());

    Ok(CachedDep { js, css })
}

#[cfg(test)]
#[path = "test/build.rs"]
mod tests;
