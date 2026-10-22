//! go manifest recognition.
//! Nhận diện manifest go.
use super::super::shared::{signature_file_exists, text_contains_any};
use crate::project::read_regular_project_text;
use std::path::Path;
pub(in crate::project::project::core_detection) fn detect_go_core(
    project_root: &Path,
) -> anyhow::Result<Option<String>> {
    let path = project_root.join("go.mod");
    if !signature_file_exists(&path)? {
        return Ok(None);
    }
    let content = read_regular_project_text(&path, "Go manifest")?
        .ok_or_else(|| anyhow::anyhow!("Go manifest disappeared during core detection"))?;
    Ok(Some(
        if text_contains_any(
            &content,
            &[
                "github.com/gin-gonic/gin",
                "github.com/labstack/echo",
                "github.com/gofiber/fiber",
                "github.com/go-chi/chi",
                "github.com/gorilla/mux",
            ],
        ) {
            "web".to_string()
        } else {
            "lib".to_string()
        },
    ))
}
