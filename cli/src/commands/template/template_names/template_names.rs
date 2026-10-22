/// Tên package trên registry: `mgc-create-<core>-<name>` (Bun/pnpm naming, plan r3, T5).
/// Name chứa dấu / (layer rel như backend/go/gin) được dash-hoá;
/// nếu name đã bắt đầu "<core>/" thì core không lặp (publish-all truyền rel đầy đủ).
pub fn registry_package_name(core: &str, name: &str) -> String {
    let prefix = format!("{core}/");
    let inner = name.strip_prefix(&prefix).unwrap_or(name);
    format!("mgc-create-{core}-{}", inner.replace('/', "-"))
}

/// Rel path của layer trong templates/ — name flat (react-vite → web/frontend/react-vite)
/// hoặc full rel (backend/go/gin → giữ nguyên).
/// Cache cũng nằm theo rel này để TemplateRoot::resolve thấy.
pub fn layer_rel(core: &str, name: &str) -> String {
    if name.starts_with(&format!("{core}/")) {
        return name.to_string();
    }
    if name.contains('/') {
        return format!("{core}/{name}");
    }
    if core == "web" {
        format!("web/frontend/{name}")
    } else {
        format!("{core}/{name}")
    }
}
