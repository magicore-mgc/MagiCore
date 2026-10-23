//! Django starter contract tests.
//! Test hợp đồng starter Django.

use super::super::write_django;

fn temp_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mgc-django-{tag}-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

#[test]
fn writes_expected_django_tree() {
    let dir = temp_dir("tree");
    write_django(&dir, "my-app").expect("write django");

    for relative in [
        "pyproject.toml",
        "manage.py",
        "my_app/__init__.py",
        "my_app/settings.py",
        "my_app/urls.py",
        "my_app/wsgi.py",
        "my_app/asgi.py",
    ] {
        assert!(
            dir.join(relative).exists(),
            "missing generated file: {relative}"
        );
    }

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn settings_take_secret_from_environment_with_dev_fallback() {
    let dir = temp_dir("secret");
    write_django(&dir, "my-app").expect("write django");

    let settings = std::fs::read_to_string(dir.join("my_app/settings.py")).expect("settings");
    assert!(settings.contains("os.environ.get(\"DJANGO_SECRET_KEY\""));
    assert!(settings.contains("dev-only-insecure-key"));
    assert!(settings.contains("Production MUST provide DJANGO_SECRET_KEY"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn health_route_and_python_module_name_are_valid() {
    let dir = temp_dir("health");
    write_django(&dir, "9lives-app").expect("write django");

    let urls = std::fs::read_to_string(dir.join("_9lives_app/urls.py")).expect("urls");
    assert!(urls.contains("api/health"));
    assert!(urls.contains("\"status\": \"ok\""));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn does_not_generate_migrations() {
    let dir = temp_dir("migrations");
    write_django(&dir, "my-app").expect("write django");

    let has_migrations = dir
        .join("my_app")
        .read_dir()
        .expect("module dir")
        .flatten()
        .any(|entry| entry.file_name().to_string_lossy().contains("migrations"));
    assert!(
        !has_migrations,
        "migrations must stay with the project owner"
    );

    std::fs::remove_dir_all(&dir).ok();
}
