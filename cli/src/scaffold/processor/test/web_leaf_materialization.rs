// Web framework leaf-file materialization tests.
// Kiểm thử materialize file leaf theo framework Web.

#[test]
fn test_web_leaf_templates_materialize_framework_specific_files() {
    if ![
        "web/frontend/react-vite",
        "web/frontend/nextjs",
        "web/frontend/vue-vite",
        "web/frontend/vanilla",
        "web/frontend/solidjs",
        "web/fullstack/split/react-express",
    ]
    .iter()
    .all(|rel| template_layer_ready(rel))
    {
        eprintln!("skipped: web template layers not available offline (registry-first)");
        return;
    }
    let root = tempfile::tempdir().unwrap();

    let react_dir = root.path().join("react-vite-app");
    let react = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["react-vite".to_string()],
        project_name: react_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
    };
    let react_out = Scaffolder::scaffold(&react).unwrap();
    assert!(react_out.join("package.json").exists());
    assert!(react_out.join("vite.config.js").exists());
    assert!(react_out.join("index.html").exists());
    assert!(
        react_out
            .join("crates")
            .join("engine")
            .join("Cargo.toml")
            .exists()
    );
    assert!(react_out.join("src").join("main.jsx").exists());
    assert!(react_out.join("src").join("App.jsx").exists());
    assert!(
        react_out
            .join("src")
            .join("bridges")
            .join("engine.js")
            .exists()
    );
    assert!(
        react_out
            .join("src")
            .join("styles")
            .join("theme.css")
            .exists()
    );
    assert!(
        react_out
            .join("src")
            .join("assets")
            .join("magicore-grid.svg")
            .exists()
    );
    assert!(!react_out.join("tsconfig.json").exists());

    let next_dir = root.path().join("next-app");
    let next = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["nextjs".to_string()],
        project_name: next_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
    };
    let next_out = Scaffolder::scaffold(&next).unwrap();
    assert!(next_out.join("next.config.mjs").exists());
    assert!(
        next_out
            .join("crates")
            .join("engine")
            .join("Cargo.toml")
            .exists()
    );
    assert!(next_out.join("src").join("app").join("page.jsx").exists());
    assert!(
        next_out
            .join("src")
            .join("bridges")
            .join("engine.js")
            .exists()
    );
    assert!(next_out.join("jsconfig.json").exists());
    assert!(!next_out.join("src").join("main.tsx").exists());

    let vue_dir = root.path().join("vue-app");
    let vue = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["vue-vite".to_string()],
        project_name: vue_dir.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let vue_out = Scaffolder::scaffold(&vue).unwrap();
    assert!(vue_out.join("package.json").exists());
    assert!(vue_out.join("vite.config.ts").exists());
    assert!(vue_out.join("src").join("main.ts").exists());
    assert!(vue_out.join("src").join("App.vue").exists());
    assert!(
        vue_out
            .join("src")
            .join("components")
            .join("AppShell.vue")
            .exists()
    );
    assert!(
        vue_out
            .join("src")
            .join("router")
            .join("AppRouter.vue")
            .exists()
    );
    assert!(
        vue_out
            .join("src")
            .join("hooks")
            .join("useProjectLinks.ts")
            .exists()
    );
    assert!(
        !vue_out
            .join("src")
            .join("components")
            .join("AppShell.tsx")
            .exists()
    );

    let vanilla_dir = root.path().join("vanilla-app");
    let vanilla = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["vanilla".to_string()],
        project_name: vanilla_dir.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let vanilla_out = Scaffolder::scaffold(&vanilla).unwrap();
    assert!(vanilla_out.join("package.json").exists());
    assert!(vanilla_out.join("vite.config.ts").exists());
    assert!(vanilla_out.join("src").join("main.ts").exists());
    assert!(vanilla_out.join("src").join("App.ts").exists());
    assert!(
        vanilla_out
            .join("src")
            .join("components")
            .join("AppShell.ts")
            .exists()
    );
    assert!(
        vanilla_out
            .join("src")
            .join("router")
            .join("AppRouter.ts")
            .exists()
    );
    assert!(!vanilla_out.join("src").join("main.tsx").exists());

    let react_express_dir = root.path().join("react-express-app");
    let react_express = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "fullstack".to_string(),
        frameworks: vec!["react-express".to_string()],
        project_name: react_express_dir.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let react_express_out = Scaffolder::scaffold(&react_express).unwrap();
    assert!(react_express_out.join("package.json").exists());
    assert!(react_express_out.join("vite.config.ts").exists());
    assert!(react_express_out.join("src").join("main.tsx").exists());
    assert!(
        react_express_out
            .join("src")
            .join("styles")
            .join("theme.css")
            .exists()
    );
    assert!(
        react_express_out
            .join("server")
            .join("src")
            .join("server.ts")
            .exists()
    );

    let solid_dir = root.path().join("solid-app");
    let solid = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["solidjs".to_string()],
        project_name: solid_dir.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let solid_out = Scaffolder::scaffold(&solid).unwrap();
    assert!(solid_out.join("package.json").exists());
    assert!(solid_out.join("vite.config.ts").exists());
    assert!(solid_out.join("src").join("main.tsx").exists());
    assert!(solid_out.join("src").join("App.tsx").exists());
    assert!(
        solid_out
            .join("src")
            .join("components")
            .join("AppShell.tsx")
            .exists()
    );
    assert!(
        solid_out
            .join("src")
            .join("router")
            .join("AppRouter.tsx")
            .exists()
    );

    let fastify_dir = root.path().join("fastify-api");
    let fastify = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "backend".to_string(),
        frameworks: vec!["node".to_string(), "fastify".to_string()],
        project_name: fastify_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
    };
    let fastify_out = Scaffolder::scaffold(&fastify).unwrap();
    assert!(fastify_out.join("src").join("server.js").exists());
    assert!(
        fastify_out
            .join("src")
            .join("config")
            .join("app.js")
            .exists()
    );
    assert!(
        fastify_out
            .join("src")
            .join("routes")
            .join("health.js")
            .exists()
    );
    assert!(
        fastify_out
            .join("src")
            .join("services")
            .join("status.js")
            .exists()
    );
    assert!(!fastify_out.join("tsconfig.json").exists());
}
