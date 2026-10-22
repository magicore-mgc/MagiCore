use crate::adapter::types::LibAdapter;
use crate::language::{LibLanguage, detect_language};
use anyhow::Result;
use std::path::Path;

// P0-4 (2026-09-15): Result<Option<_>> — Ok(None) means "not a lib
// project" (an absence, not an error); Err carries the typed
// registry-URL failure from the TS/web lane.
// (P0-4: Result<Option<_>> — Ok(None) nghĩa là "không phải project lib"
// (vắng mặt, không phải lỗi); Err mang lỗi registry-URL typed từ lane TS/web.)
pub fn adapter_for(
    root: &Path,
    registry_url: Option<String>,
    token: Option<String>,
) -> Result<Option<LibAdapter>> {
    let Some(language) = detect_language(root) else {
        return Ok(None);
    };
    Ok(Some(LibAdapter::for_language(
        language,
        root,
        registry_url,
        token,
    )?))
}

pub fn adapter_for_with_chain(
    root: &Path,
    registry_url: Option<String>,
    token: Option<String>,
    fallbacks: &[(String, Option<String>)],
) -> Result<Option<LibAdapter>> {
    let Some(language) = detect_language(root) else {
        return Ok(None);
    };
    Ok(Some(LibAdapter::for_language_with_chain(
        language,
        root,
        registry_url,
        token,
        fallbacks,
    )?))
}

/// Bind scoped npm credentials only to the embedded TypeScript engine.
/// Lane ngoài npm không dựng WebAdapter hoặc diễn giải credential npm.
pub fn adapter_for_with_scoped_chain(
    root: &Path,
    entries: Vec<mgc_config::chain::ScopedRegistry>,
) -> Result<Option<LibAdapter>> {
    let Some(language) = detect_language(root) else {
        return Ok(None);
    };
    let web = if language == LibLanguage::Ts {
        Some(mgc_web_adapter::WebAdapter::with_scoped_registry_chain(
            entries,
        )?)
    } else {
        None
    };
    Ok(Some(LibAdapter::from_web(language, root, web)?))
}

/// Construct a LibAdapter for an EXPLICIT language — the ai core's PyPI
/// lane needs the lib python engine regardless of root-detect heuristics
/// (an ai project's pyproject.toml must route to Python, never guess).
/// (Dựng LibAdapter theo ngôn ngữ TƯỜNG MINH — lane PyPI của core ai cần
/// engine python của lib bất chấp heuristic detect theo root.)
pub fn adapter_for_language(
    language: LibLanguage,
    root: &Path,
    registry_url: Option<String>,
    token: Option<String>,
) -> Result<LibAdapter> {
    LibAdapter::for_language(language, root, registry_url, token)
}

/// Construct the native library-language engine for a non-JS web backend.
/// The adapter keeps its implementation identity (`lib`) while reading web
/// security policy from the owning web project.
/// Dựng engine ngôn ngữ native cho backend web ngoài JS. Adapter giữ identity
/// implementation (`lib`) nhưng đọc security policy của project owner `web`.
pub fn adapter_for_web_backend(language: LibLanguage, root: &Path) -> Result<LibAdapter> {
    let mut adapter = LibAdapter::for_language(language, root, None, None)?;
    adapter.security_core = "web";
    Ok(adapter)
}

/// Construct the embedded Python engine with the AI core's security scope.
/// Dựng Python engine nhúng theo phạm vi bảo mật của core AI.
pub fn adapter_for_ai_python(root: &Path) -> Result<LibAdapter> {
    let mut adapter = LibAdapter::for_language(LibLanguage::Python, root, None, None)?;
    adapter.security_core = "ai";
    Ok(adapter)
}
