//! Classify packages using the existing ordered search-specific heuristics.
//! Phân loại package bằng các heuristic riêng của search theo đúng thứ tự hiện tại.

pub(in crate::commands::search) fn detect_core_label(name: &str, desc: &str) -> String {
    let combined = format!("{} {}", name, desc).to_lowercase();

    if combined.contains("react")
        || combined.contains("vue")
        || combined.contains("vite")
        || combined.contains("next")
        || combined.contains("svelte")
        || combined.contains("astro")
        || combined.contains("tailwind")
        || combined.contains("angular")
        || combined.contains("frontend")
        || combined.contains("css")
    {
        return "[Core: Web / Frontend]".to_string();
    }
    if combined.contains("express")
        || combined.contains("fastify")
        || combined.contains("hono")
        || combined.contains("server")
        || combined.contains("rest api")
        || combined.contains("backend")
        || combined.contains("graphql")
    {
        return "[Core: Web / Backend]".to_string();
    }
    if combined.contains("langchain")
        || combined.contains("openai")
        || combined.contains("llm")
        || combined.contains("agent")
        || combined.contains("embedding")
        || combined.contains("machine learning")
    {
        return "[Core: AI / ML]".to_string();
    }
    if combined.contains("aws")
        || combined.contains("gcp")
        || combined.contains("azure")
        || combined.contains("cloud")
        || combined.contains("terraform")
        || combined.contains("pulumi")
    {
        return "[Core: Cloud (clo)]".to_string();
    }
    if combined.contains("github-action")
        || combined.contains("ci/cd")
        || combined.contains("pipeline")
        || combined.contains("deploy")
    {
        return "[Core: CI/CD]".to_string();
    }
    if combined.contains("bevy")
        || combined.contains("godot")
        || combined.contains("game")
        || combined.contains("rendering engine")
    {
        return "[Core: Game]".to_string();
    }
    if combined.contains("flutter")
        || combined.contains("mobile")
        || combined.contains("tauri")
        || combined.contains("electron")
    {
        return "[Core: App (Mobile/Desktop)]".to_string();
    }
    "[Core: Web / Node]".to_string()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
