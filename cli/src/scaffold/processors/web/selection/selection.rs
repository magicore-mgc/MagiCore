//! Native project file selection for Web scaffolds.
//! Chọn framework native cho scaffold Web.

/// Whether the Web processor can materialize this sub-type/framework natively.
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
