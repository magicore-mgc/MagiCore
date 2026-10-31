//! Native (non-Node) server-half tests — kiểm thử nửa server native (không Node).
//! Each backend half must write a complete, parseable server before the dev lane
//! can start it; the writes are pure, so these tests never spawn a toolchain.
//! Mỗi nửa backend phải ghi server trọn vẹn, parse được trước khi lane dev khởi
//! động; phần ghi là thuần nên test không spawn toolchain nào.

use std::path::{Path, PathBuf};

// The native owner exposes only the composer surface (`has_half`/`write`); its
// templates stay private so no other owner can fetch raw source.
// (Owner native chỉ phơi bề mặt composer (`has_half`/`write`); template vẫn
// private để owner khác không lấy source thô.)
use super::super::native_backend::{has_half, write};

#[test]
fn every_supported_backend_reports_a_half() {
    for backend in ["fastapi", "flask", "django", "axum", "gin"] {
        assert!(has_half(backend), "{backend} must have a server half");
    }
    for backend in ["express", "hono", "spring-boot", "laravel", "echo"] {
        assert!(
            !has_half(backend),
            "{backend} is not handled by the native owner"
        );
    }
}

#[test]
fn fastapi_half_writes_a_server_and_its_requirements() {
    let root = project_dir("fastapi");
    write(&root, "fastapi", "react-fastapi").expect("fastapi half is written");

    let main = read(&root.join("server/main.py"));
    assert!(main.contains("FastAPI("));
    assert!(main.contains("/api/health"));
    assert_no_placeholder("main.py", &main);

    let manifest = read(&root.join("server/pyproject.toml"));
    assert!(manifest.contains("[project]"));
    assert!(manifest.contains("fastapi"));
    assert!(manifest.contains("uvicorn"));
    cleanup(&root);
}

#[test]
fn flask_half_writes_an_app_module() {
    let root = project_dir("flask");
    write(&root, "flask", "vue-flask").expect("flask half is written");

    let app = read(&root.join("server/app.py"));
    assert!(app.contains("Flask(__name__)"));
    assert_no_placeholder("app.py", &app);
    assert!(root.join("server/pyproject.toml").is_file());
    cleanup(&root);
}

#[test]
fn django_half_writes_manage_settings_urls_and_views() {
    let root = project_dir("django");
    write(&root, "django", "react-django").expect("django half is written");

    for relative in [
        "server/manage.py",
        "server/config/settings.py",
        "server/config/urls.py",
        "server/api/__init__.py",
        "server/api/views.py",
        "server/pyproject.toml",
    ] {
        assert!(root.join(relative).is_file(), "{relative} missing");
        assert_no_placeholder(relative, &read(&root.join(relative)));
    }
    // The settings module named by manage.py must exist and be importable-shaped.
    // (Module settings mà manage.py trỏ tới phải tồn tại và đúng dạng import được.)
    let manage = read(&root.join("server/manage.py"));
    assert!(manage.contains("DJANGO_SETTINGS_MODULE"));
    assert!(root.join("server/config/settings.py").is_file());
    cleanup(&root);
}

#[test]
fn django_module_name_is_always_importable() {
    let root = project_dir("django-name");
    write(&root, "django", "react-django").expect("django half is written");
    let manage = read(&root.join("server/manage.py"));
    let module_line = manage
        .lines()
        .find(|line| line.contains("DJANGO_SETTINGS_MODULE"))
        .expect("settings module is declared");
    assert!(module_line.contains(".settings\""), "{module_line}");
    // The module literal is everything between the pair of quotes.
    // (Literal module là phần nằm giữa cặp nháy kép.)
    // `setdefault("DJANGO_SETTINGS_MODULE", "<module>.settings")` — the second
    // quoted segment is the value.
    // (`setdefault("DJANGO_SETTINGS_MODULE", "<module>.settings")` — đoạn nháy
    // thứ hai là giá trị.)
    let quoted = module_line
        .split('"')
        .nth(3)
        .expect("the settings module is quoted");
    let module = quoted
        .strip_suffix(".settings")
        .unwrap_or_else(|| panic!("unexpected settings module: {quoted}"));
    assert!(
        module
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
        "{module} must be a plain Python module name"
    );
    assert!(
        !module.starts_with(|c: char| c.is_ascii_digit()),
        "{module} must not start with a digit"
    );
    cleanup(&root);
}

#[test]
fn axum_half_writes_a_cargo_manifest_and_main() {
    let root = project_dir("axum");
    write(&root, "axum", "react-axum").expect("axum half is written");

    let manifest = read(&root.join("server/Cargo.toml"));
    assert!(manifest.contains("[package]"));
    assert!(manifest.contains("axum = "));
    assert!(manifest.contains("tokio"));
    let main = read(&root.join("server/src/main.rs"));
    assert!(main.contains("fn main()"));
    assert!(main.contains("/api/health"));
    assert_no_placeholder("src/main.rs", &main);
    cleanup(&root);
}

#[test]
fn gin_half_writes_a_go_module_and_main() {
    let root = project_dir("gin");
    write(&root, "gin", "vue-gin").expect("gin half is written");

    let module = read(&root.join("server/go.mod"));
    // The comment header precedes the directive, so the `module` line is matched
    // by its own line rather than by file position.
    // (Header comment đứng trước directive nên dòng `module` được khớp theo
    // chính nó, không theo vị trí file.)
    assert!(
        module
            .lines()
            .any(|line| line.trim_start().starts_with("module ")),
        "go.mod must declare its module: {module}"
    );
    assert!(module.contains("go 1."));
    assert!(module.contains("gin-gonic/gin"));
    let main = read(&root.join("server/main.go"));
    assert!(main.contains("func main()"));
    assert!(main.contains("/api/health"));
    assert_no_placeholder("main.go", &main);
    cleanup(&root);
}

#[test]
fn unsupported_backend_fails_closed_without_writing() {
    let root = project_dir("unsupported");
    let error = write(&root, "spring-boot", "react-spring")
        .expect_err("an unsupported backend has no half");
    let message = error.to_string();
    assert!(message.contains("react-spring"), "{message}");
    assert!(message.contains("'spring-boot' server half"), "{message}");
    assert!(!root.join("server").exists(), "nothing may be written");
    cleanup(&root);
}

/// Fail when a template placeholder survived rendering.
/// Báo lỗi khi một placeholder của template còn sót lại.
///
/// Decorator backend (`@app.get`, `#[tokio::main]`) is legitimate source; the
/// check is that no `@WORD@` token remains, because that shape is how every
/// template placeholder is spelled.
/// Decorator backend (`@app.get`, `#[tokio::main]`) là source hợp lệ; phép kiểm
/// là không còn token `@WORD@` nào, vì mọi placeholder của template đều viết dạng
/// đó.
fn assert_no_placeholder(file: &str, contents: &str) {
    let leftover: Vec<&str> = contents
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '@' || c == '_'))
        .filter(|token| {
            token.len() > 2
                && token.starts_with('@')
                && token.ends_with('@')
                && !token.trim_matches('@').is_empty()
                && !std::path::Path::new(file)
                    .extension()
                    .is_some_and(|ext| ext == "py")
        })
        .collect();
    assert!(
        leftover.is_empty(),
        "{file} kept template placeholders: {leftover:?}\n{contents}"
    );
    if std::path::Path::new(file)
        .extension()
        .is_some_and(|ext| ext == "py")
    {
        assert!(
            !contents.contains("@PROJECT@")
                && !contents.contains("@MODULE@")
                && !contents.contains("@PORT@")
                && !contents.contains("@HOST@"),
            "{file} kept template placeholders:\n{contents}"
        );
    }
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// A unique, existing directory used as the project root.
/// Thư mục duy nhất, đã tồn tại, dùng làm gốc project.
fn project_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mgc-native-half-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("project directory");
    dir
}

fn cleanup(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}
