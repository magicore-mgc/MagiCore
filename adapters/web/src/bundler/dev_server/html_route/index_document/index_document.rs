//! Serve existing or fallback HTML documents with the HMR client injected.
//! Phục vụ HTML sẵn có hoặc dự phòng và chèn HMR client.

use crate::bundler::dev_server::implementation::source_file_resolution::resolve_project_file;
use crate::bundler::dev_server::implementation::types::ServerState;
use axum::{extract::State, response::Html};

/// Minimal HTML-text escaping for the fallback page title.
/// Escape text HTML tối thiểu cho title của trang dự phòng.
pub(super) fn html_escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Serve index.html with the HMR script injected.
/// Phục vụ index.html có chèn script HMR.
pub(in crate::bundler::dev_server::implementation) async fn serve_index(
    State(state): State<ServerState>,
) -> Html<String> {
    let existing_index = match resolve_project_file(&state.config.root, "index.html").await {
        Ok(Some(index_path)) => std::fs::read_to_string(index_path).ok(),
        Ok(None) | Err(_) => None,
    };
    let mut html = match existing_index {
        Some(content) => content,
        None => {
            let title_raw = state
                .config
                .root
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("MagiCore App");
            let title = html_escape_text(title_raw);
            let relative_entry = state
                .config
                .entry
                .strip_prefix(&state.config.root)
                .unwrap_or(&state.config.entry)
                .to_string_lossy()
                .replace('\\', "/");
            format!(
                r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title}</title>
</head>
<body>
  <div id="root"></div>
  <script type="module" src="/{relative_entry}"></script>
</body>
</html>"#
            )
        }
    };

    let hmr_tag = "<script type=\"module\" src=\"/@magicore/hmr.js\"></script>\n";
    if let Some(index) = html.find("</head>") {
        html.insert_str(index, hmr_tag);
    } else {
        html.push_str(hmr_tag);
    }

    Html(html)
}
