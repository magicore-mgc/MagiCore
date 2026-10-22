//! Keyword-based package core classification.
//! Phân loại core package dựa trên keyword.

/// Classify a package name and description using the existing ordered heuristics.
/// Phân loại tên và mô tả package theo thứ tự heuristic hiện tại.
pub(in crate::commands::info) fn detect_core_label(name: &str, desc: &str) -> String {
    let combined = format!("{} {}", name, desc).to_lowercase();

    if combined.contains("react")
        || combined.contains("vue")
        || combined.contains("vite")
        || combined.contains("next")
        || combined.contains("nuxt")
        || combined.contains("svelte")
        || combined.contains("astro")
        || combined.contains("angular")
        || combined.contains("frontend")
        || combined.contains("ui library")
        || combined.contains("css")
        || combined.contains("tailwind")
    {
        return "[Core: Web / Frontend]".to_string();
    }

    if combined.contains("express")
        || combined.contains("fastify")
        || combined.contains("hono")
        || combined.contains("koa")
        || combined.contains("backend")
        || combined.contains("server")
        || combined.contains("rest api")
        || combined.contains("graphql")
    {
        return "[Core: Web / Backend]".to_string();
    }

    if combined.contains("langchain")
        || combined.contains("openai")
        || combined.contains("llm")
        || combined.contains("embedding")
        || combined.contains("agent")
        || combined.contains("machine learning")
        || combined.contains("tensorflow")
        || combined.contains("pytorch")
    {
        return "[Core: AI / ML]".to_string();
    }

    if combined.contains("pulumi")
        || combined.contains("terraform")
        || combined.contains("cloud")
        || combined.contains("aws")
        || combined.contains("gcp")
        || combined.contains("azure")
    {
        return "[Core: Cloud (clo)]".to_string();
    }

    if combined.contains("github-actions")
        || combined.contains("ci/cd")
        || combined.contains("pipeline")
        || combined.contains("deploy")
    {
        return "[Core: CI/CD]".to_string();
    }

    if combined.contains("bevy")
        || combined.contains("godot")
        || combined.contains("game")
        || combined.contains("3d engine")
        || combined.contains("rendering")
    {
        return "[Core: Game]".to_string();
    }

    if combined.contains("flutter")
        || combined.contains("mobile")
        || combined.contains("electron")
        || combined.contains("tauri")
        || combined.contains("desktop app")
    {
        return "[Core: App (Mobile/Desktop)]".to_string();
    }

    "[Core: General / Library]".to_string()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
