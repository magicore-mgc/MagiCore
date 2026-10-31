//! Dev-server request-security E2E (P1-1/P1-2/P1-3, fresh-context review
//! 2026-09-15): the source/static route joined the raw request path onto
//! the project root WITHOUT a traversal gate, and the deps route joined
//! the raw package name onto node_modules — both served files OUTSIDE the
//! project on `GET /../...`. This file drives the REAL MgDevServer over
//! the real network stack and pins the fail-closed contract:
//!   - traversal paths → 403/404, NEVER the outside file's bytes;
//!   - percent-encoded traversal (%2e%2e) → blocked identically;
//!   - deps route traversal (`/@magicore/deps/../../...`) → blocked;
//!   - the CSS template-literal escape (`${`) → pinned at unit level
//!     (css_template_literal_is_escaped).
//!
//! Port DYNAMIC (bind :0) per RULE §13 (tests never fight over 4315).
//! (Bảo mật request dev-server: route source/static nối path request thô
//! vào root project KHÔNG cổng traversal, route deps nối tên package thô
//! vào node_modules — cả hai phục vụ file NGOÀI project với
//! `GET /../...`. File này chạy MgDevServer THẬT qua network stack thật
//! và ghim hợp đồng fail-closed. Port ĐỘNG theo RULE §13.)

#![allow(clippy::unwrap_used)]

use std::time::Duration;

use mgc::bundler::dev_server::{DevServerConfig, MgDevServer};

/// Pick a free port by binding :0 — never the RULE §13 fixed defaults.
/// (Chọn port tự do bằng bind :0 — không bao giờ port cố định RULE §13.)
fn free_port() -> u16 {
    std::net::TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

/// Await server readiness on the dynamic port (bounded poll: 10s —
///
/// full-workspace parallel runs can starve a 5s budget and flake).
/// (Chờ server sẵn sàng — poll 10s vì chạy song song full suite đói CPU.)
async fn wait_ready(port: u16) {
    for _ in 0..200 {
        if reqwest::get(format!("http://127.0.0.1:{port}/"))
            .await
            .is_ok()
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("dev server must become ready on the dynamic port");
}

/// One adversarial dev-server fixture: a project root with a SECRET
/// file OUTSIDE the root (parent dir) that a traversal request must
/// NEVER be able to read, plus an in-root src file for the happy path.
/// (Một fixture dev-server đối kháng: project root kèm file BÍ MẬT nằm
/// NGOÀI root (thư mục cha) mà request traversal KHÔNG BAO GIỜ được
/// đọc, cùng file src trong root cho đường hợp lệ.)
struct AdversarialSite {
    _tmp: tempfile::TempDir,
    root: std::path::PathBuf,
    // Underscore-prefixed: the file's existence is exercised via the
    // traversal REQUEST, not by reading this field directly.
    // (Prefix gạch-dưới: sự tồn tại của file được kiểm qua REQUEST
    // traversal, không đọc field này trực tiếp.)
    _secret_path: std::path::PathBuf,
    secret_marker: String,
}

impl AdversarialSite {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("site");
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("index.html"), "<html><body>ok</body></html>").unwrap();
        std::fs::write(
            root.join("src").join("main.ts"),
            "export const MARKER = 'in-root-marker';\n",
        )
        .unwrap();
        // SECRET outside the root: traversal targets must never reach it.
        // (File BÍ MẬT ngoài root: path traversal không bao giờ được tới.)
        let secret_marker = "TOP-SECRET-OUTSIDE-ROOT-4315".to_string();
        let secret_path = tmp.path().join("outside-secret.txt");
        std::fs::write(&secret_path, &secret_marker).unwrap();
        Self {
            _tmp: tmp,
            root,
            _secret_path: secret_path,
            secret_marker,
        }
    }
}

/// P1-1: `GET /../outside-secret.txt` (and every spelling of traversal)
/// must NEVER return the secret file's bytes — 403/404 before the fix
/// served them (path join follows `..`). Encoded traversal `%2e%2e` is
/// the CSRF-relevant spelling (browsers normalize, proxies don't).
/// (P1-1: `GET /../outside-secret.txt` và mọi cách viết traversal
/// KHÔNG BAO GIỜ trả bytes file bí mật — trước fix chúng được phục vụ
/// (path join theo `..`). Traversal mã hóa `%2e%2e` là dạng liên quan
/// CSRF — phải chặn y hệt.)
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn source_route_traversal_never_serves_outside_files() {
    let site = AdversarialSite::new();
    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    // Every traversal spelling of "read the parent-dir secret".
    // (Mọi cách viết traversal của "đọc file bí mật ở thư mục cha".)
    let attacks = [
        "/../outside-secret.txt",
        "/%2e%2e/outside-secret.txt",
        "/%2E%2E/outside-secret.txt",
        "/..%2foutside-secret.txt",
        "/src/../../outside-secret.txt",
        "/public/../../outside-secret.txt",
    ];
    for attack in attacks {
        let resp = reqwest::get(format!("http://127.0.0.1:{port}{attack}"))
            .await
            .unwrap();
        let status = resp.status();
        let body = resp.text().await.unwrap();
        assert!(
            !body.contains(&site.secret_marker),
            "traversal request {attack:?} leaked the OUTSIDE-ROOT secret (status {status}, body {body:?}) — P1-1 regression"
        );
        assert!(
            status == reqwest::StatusCode::FORBIDDEN || status == reqwest::StatusCode::NOT_FOUND,
            "traversal request {attack:?} must fail closed (403/404), got {status}"
        );
    }

    // The happy path still serves the IN-ROOT file (fail-closed, not
    // fail-broken: legitimate requests must keep working).
    // (Đường hợp lệ vẫn phục vụ file TRONG ROOT — fail-closed không có
    // nghĩa fail-vô-đụng: request chính đáng phải chạy tiếp.)
    let ok = reqwest::get(format!("http://127.0.0.1:{port}/src/main.ts"))
        .await
        .unwrap();
    assert!(ok.status().is_success());
    let body = ok.text().await.unwrap();
    assert!(
        body.contains("in-root-marker"),
        "in-root source must still be served after the traversal gate, body: {body:?}"
    );

    server.abort();
    server.await.ok();
}

/// P1-1 (deps side): `GET /@magicore/deps/<traversal>` joined the raw
/// package name onto node_modules — `..%2f..` escaped the store. The
/// route must fail closed and never serve outside files.
/// (P1-1 (phía deps): `GET /@magicore/deps/<traversal>` nối tên package
/// thô vào node_modules — `..%2f..` thoát ra ngoài store. Route phải
/// fail-closed, không bao giờ phục vụ file ngoài.)
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn deps_route_traversal_never_serves_outside_files() {
    let site = AdversarialSite::new();
    // A node_modules dir so the deps route has a real base to join.
    // (Thư mục node_modules để route deps có base thật để nối.)
    std::fs::create_dir_all(site.root.join("node_modules")).unwrap();
    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let attacks = [
        "/@magicore/deps/../../outside-secret.txt",
        "/@magicore/deps/..%2f..%2foutside-secret.txt",
        "/@magicore/deps/%2e%2e%2f%2e%2e%2foutside-secret.txt",
        "/@magicore/deps/../../../etc/passwd",
    ];
    for attack in attacks {
        let resp = reqwest::get(format!("http://127.0.0.1:{port}{attack}"))
            .await
            .unwrap();
        let status = resp.status();
        let body = resp.text().await.unwrap();
        assert!(
            !body.contains(&site.secret_marker),
            "deps traversal {attack:?} leaked the OUTSIDE-ROOT secret (status {status}, body {body:?}) — P1-1 deps regression"
        );
    }

    server.abort();
    server.await.ok();
}

/// P1-2 (deps `<stdout>` sibling bug): with `write=false` esbuild-rs may
/// report the single output entry's path as `<stdout>` — the old
/// `find(ends_with(".js"))` in deps_bundler matched NOTHING and the
/// served dep JS was EMPTY. The transpile path was already fixed; this
/// pins the unit-level selection helper for the deps path: prefer a real
/// `.js` output; else take the FIRST output; never default to empty
/// when outputs exist.
/// (P1-2 (bug anh em `<stdout>` của deps): với `write=false`, esbuild-rs
/// có thể báo path của output đơn là `<stdout>` — `find` cũ trong
/// deps_bundler khớp 0 phần tử và dep được phục vụ là JS RỖNG. Đường
/// transpile đã sửa; test này ghim helper chọn output mức unit cho đường
/// deps: ưu tiên `.js` thật, không có thì lấy output ĐẦU, không bao giờ
/// mặc định rỗng khi có output.)
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn deps_bundle_of_a_real_package_serves_nonempty_js() {
    // A tiny CJS package in node_modules — bundling it must yield
    // NON-EMPTY JS (the `<stdout>` bug would yield an empty string).
    // (Một package CJS nhỏ trong node_modules — bundle nó phải ra JS
    // KHÔNG RỖNG (bug `<stdout>` sẽ trả chuỗi rỗng).)
    let site = AdversarialSite::new();
    let pkg_dir = site.root.join("node_modules").join("tiny-dep");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(
        pkg_dir.join("package.json"),
        r#"{ "name": "tiny-dep", "version": "1.0.0", "main": "index.js" }"#,
    )
    .unwrap();
    std::fs::write(
        pkg_dir.join("index.js"),
        "module.exports = function tiny(){ return 'tiny-dep-payload'; };",
    )
    .unwrap();

    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/@magicore/deps/tiny-dep"))
        .await
        .unwrap();
    let status = resp.status();
    let body = resp.text().await.unwrap();
    assert!(
        status.is_success(),
        "deps bundle of an installed package must succeed, status {status}, body {body:?}"
    );
    assert!(
        !body.trim().is_empty(),
        "deps bundle must serve NON-EMPTY JS (the <stdout> sibling bug served empty output) — P1-2"
    );
    assert!(
        body.contains("tiny-dep-payload"),
        "deps bundle must carry the package's actual code, body: {body:?}"
    );

    server.abort();
    server.await.ok();
}

/// Hidden files stay private while public files and well-known assets work.
/// File ẩn vẫn riêng tư, còn file public và well-known vẫn dùng được.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn source_route_blocks_hidden_and_external_targets() {
    let site = AdversarialSite::new();
    std::fs::write(site.root.join(".env"), "TOP-SECRET-HIDDEN-ROOT").unwrap();
    std::fs::create_dir_all(site.root.join(".git")).unwrap();
    std::fs::write(site.root.join(".git/config"), "TOP-SECRET-GIT-CONFIG").unwrap();

    let public_file = site.root.join("public/logo.txt");
    std::fs::create_dir_all(public_file.parent().unwrap()).unwrap();
    std::fs::write(&public_file, "public-logo-marker").unwrap();
    let well_known = site.root.join("public/.well-known/security.txt");
    std::fs::create_dir_all(well_known.parent().unwrap()).unwrap();
    std::fs::write(&well_known, "contact=security@example.invalid").unwrap();

    #[cfg(unix)]
    {
        let alias = site.root.join("public/outside-secret.txt");
        std::os::unix::fs::symlink(&site._secret_path, &alias).unwrap();
    }

    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    for path in ["/.env", "/.git/config"] {
        let response = reqwest::get(format!("http://127.0.0.1:{port}{path}"))
            .await
            .unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();
        assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
        assert!(
            !body.contains("TOP-SECRET"),
            "hidden file response must not contain secret content"
        );
    }

    let public = reqwest::get(format!("http://127.0.0.1:{port}/logo.txt"))
        .await
        .unwrap();
    assert!(public.status().is_success());
    assert_eq!(public.text().await.unwrap(), "public-logo-marker");

    let well_known = reqwest::get(format!("http://127.0.0.1:{port}/.well-known/security.txt"))
        .await
        .unwrap();
    assert!(well_known.status().is_success());
    assert_eq!(
        well_known.text().await.unwrap(),
        "contact=security@example.invalid"
    );

    #[cfg(unix)]
    {
        let outside = reqwest::get(format!("http://127.0.0.1:{port}/outside-secret.txt"))
            .await
            .unwrap();
        let status = outside.status();
        let body = outside.text().await.unwrap();
        assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
        assert!(
            !body.contains(&site.secret_marker),
            "external symlink response must not contain outside-root bytes"
        );
    }

    server.abort();
    server.await.ok();
}

/// The root HTML route must apply the same project boundary as wildcard files.
/// Route HTML root phải áp dụng cùng ranh giới project như wildcard file.
#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_route_does_not_follow_external_symlinks() {
    let site = AdversarialSite::new();
    std::fs::remove_file(site.root.join("index.html")).unwrap();
    let outside_index = site._tmp.path().join("outside-index.html");
    std::fs::write(&outside_index, "INDEX-OUTSIDE-ROOT-SECRET").unwrap();
    std::os::unix::fs::symlink(&outside_index, site.root.join("index.html")).unwrap();

    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let response = reqwest::get(format!("http://127.0.0.1:{port}/"))
        .await
        .unwrap();
    assert!(response.status().is_success());
    let body = response.text().await.unwrap();
    assert!(body.contains("<title>site</title>"));
    assert!(!body.contains("INDEX-OUTSIDE-ROOT-SECRET"));

    server.abort();
    server.await.ok();
}

/// A package manifest must not turn the dependency route into a file reader.
/// Manifest package không được biến dependency route thành reader file tùy ý.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn deps_route_rejects_manifest_entries_outside_package_root() {
    let site = AdversarialSite::new();
    let pkg_dir = site.root.join("node_modules/untrusted-dep");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(
        pkg_dir.join("package.json"),
        r#"{ "name": "untrusted-dep", "main": "../../../outside-entry.js" }"#,
    )
    .unwrap();
    let outside_entry = site._tmp.path().join("outside-entry.js");
    std::fs::write(&outside_entry, "OUTSIDE-PACKAGE-ENTRY-SECRET").unwrap();

    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let response = reqwest::get(format!(
        "http://127.0.0.1:{port}/@magicore/deps/untrusted-dep"
    ))
    .await
    .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
    let body = response.text().await.unwrap();
    assert!(!body.contains("OUTSIDE-PACKAGE-ENTRY-SECRET"));

    server.abort();
    server.await.ok();
}

/// Package subpath files must resolve through the actual dependency route.
/// File subpath phải resolve qua dependency route thật.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn deps_route_bundles_unscoped_package_subpath_files() {
    let site = AdversarialSite::new();
    let subpath_dir = site.root.join("node_modules/lodash/fp");
    std::fs::create_dir_all(&subpath_dir).unwrap();
    std::fs::write(
        subpath_dir.join("map.js"),
        "export const mgcSubpath = 'resolved-subpath';",
    )
    .unwrap();

    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let response = reqwest::get(format!(
        "http://127.0.0.1:{port}/@magicore/deps/lodash/fp/map.js"
    ))
    .await
    .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert!(response.text().await.unwrap().contains("resolved-subpath"));

    server.abort();
    server.await.ok();
}

/// Missing-package diagnostics must keep names as JavaScript data.
/// Diagnostic package thiếu phải giữ tên dưới dạng dữ liệu JavaScript.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn deps_route_escapes_unusual_package_names_in_javascript_errors() {
    let site = AdversarialSite::new();
    std::fs::create_dir_all(site.root.join("node_modules")).unwrap();
    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let response = reqwest::get(format!(
        "http://127.0.0.1:{port}/@magicore/deps/x%27);alert(1);"
    ))
    .await
    .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
    let body = response.text().await.unwrap();
    assert_eq!(
        body,
        r#"console.error('[MgDevServer] Failed to bundle dependency:', "x');alert(1);");"#
    );

    server.abort();
    server.await.ok();
}

/// P0 blank-screen regression (2026-10-31): the scaffolded `main.tsx`
/// imports `react-dom/client`, so the deps route MUST serve
/// `/@magicore/deps/react-dom/client` while `react-dom` 17 ships only a
/// root-level `client.js` (no `exports` entry for it). The route used to
/// 404 here, blanking every React app.
/// (Regression P0 màn hình trắng: `main.tsx` scaffold import
/// `react-dom/client`, nên route deps PHẢI phục vụ
/// `/@magicore/deps/react-dom/client` khi `react-dom` 17 chỉ có
/// `client.js` ở package root (không có `exports` cho nó). Trước đây
/// route trả 404, làm mọi app React trắng màn hình.)
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn deps_route_serves_subpath_without_file_extension() {
    let site = AdversarialSite::new();
    let pkg_dir = site.root.join("node_modules/react-dom");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(
        pkg_dir.join("package.json"),
        r#"{ "name": "react-dom", "version": "17.0.2", "main": "index.js" }"#,
    )
    .unwrap();
    std::fs::write(pkg_dir.join("index.js"), "module.exports = {};").unwrap();
    std::fs::write(
        pkg_dir.join("client.js"),
        "export const mgcDomClient = 'react-dom-client-entry';",
    )
    .unwrap();

    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let response = reqwest::get(format!(
        "http://127.0.0.1:{port}/@magicore/deps/react-dom/client"
    ))
    .await
    .unwrap();
    let status = response.status();
    let body = response.text().await.unwrap();
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "react-dom/client must be served, not 404 (P0 blank screen), body: {body:?}"
    );
    assert!(
        body.contains("react-dom-client-entry"),
        "react-dom/client must carry client.js code, body: {body:?}"
    );

    server.abort();
    server.await.ok();
}

/// A subpath that does not exist anywhere in the package stays a 404.
/// (Subpath không tồn tại ở bất kỳ đâu trong package vẫn là 404.)
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn deps_route_404s_genuinely_absent_subpath() {
    let site = AdversarialSite::new();
    let pkg_dir = site.root.join("node_modules/react-dom");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(
        pkg_dir.join("package.json"),
        r#"{ "name": "react-dom", "main": "index.js", "exports": { ".": "./index.js" } }"#,
    )
    .unwrap();
    std::fs::write(pkg_dir.join("index.js"), "module.exports = {};").unwrap();

    let port = free_port();
    let server_root = site.root.clone();
    let server = tokio::spawn(async move {
        let config = DevServerConfig {
            root: server_root,
            entry: "src/main.ts".into(),
            host: "localhost".to_string(),
            port,
        };
        MgDevServer::new(config).serve().await
    });
    wait_ready(port).await;

    let response = reqwest::get(format!(
        "http://127.0.0.1:{port}/@magicore/deps/react-dom/absent"
    ))
    .await
    .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);

    server.abort();
    server.await.ok();
}
