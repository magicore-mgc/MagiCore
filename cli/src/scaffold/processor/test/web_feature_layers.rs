// Web feature-gated and framework rejection tests.
// Kiểm thử feature Web và từ chối framework không hỗ trợ.

#[test]
fn test_web_typescript_feature_switches_extensions() {
    if !template_layer_ready("web/frontend/react-vite") {
        eprintln!(
            "skipped: web/frontend/react-vite template layer not available offline (registry-first)"
        );
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let project_dir = root.path().join("react-ts");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["react-vite".to_string()],
        project_name: project_dir.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };

    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.join("tsconfig.json").exists());
    assert!(out.join("vite.config.ts").exists());
    assert!(
        out.join("crates")
            .join("engine")
            .join("Cargo.toml")
            .exists()
    );
    assert!(out.join("src").join("main.tsx").exists());
    assert!(out.join("src").join("App.tsx").exists());
    assert!(out.join("src").join("bridges").join("engine.ts").exists());
    assert!(!out.join("src").join("main.jsx").exists());
}
#[test]
fn test_unknown_frameworks_fail_fast() {
    let root = tempfile::tempdir().unwrap();

    let unsupported_dir = root.path().join("ember-app");
    let unsupported = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["ember".to_string()],
        project_name: unsupported_dir.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let unsupported_err = Scaffolder::scaffold(&unsupported).unwrap_err();
    assert!(unsupported_err.to_string().contains("Web template path"));

    let broken_mono_dir = root.path().join("broken-mono");
    let broken_mono = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "monorepo".to_string(),
        frameworks: vec!["ember".to_string(), "fastify".to_string()],
        project_name: broken_mono_dir.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let broken_mono_err = Scaffolder::scaffold(&broken_mono).unwrap_err();
    assert!(
        broken_mono_err
            .to_string()
            .contains("Scaffold for monorepo frontend framework 'ember' is not implemented yet")
    );
}
#[test]
fn test_web_feature_gated_templates_materialize_only_when_active() {
    if !template_layer_ready("web/frontend/nextjs") {
        eprintln!(
            "skipped: web/frontend/nextjs template layer not available offline (registry-first)"
        );
        return;
    }
    let root = tempfile::tempdir().unwrap();

    // Next.js with prisma + tailwindcss + eslint + prettier + vitest
    let with_features = root.path().join("next-features");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["nextjs".to_string()],
        project_name: with_features.to_string_lossy().to_string(),
        features: vec![
            "typescript".to_string(),
            "prisma".to_string(),
            "tailwindcss".to_string(),
            "daisyui".to_string(),
            "eslint".to_string(),
            "prettier".to_string(),
            "vitest".to_string(),
        ],
        template_dir: PathBuf::new(),
    };
    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.join("prisma").join("schema.prisma").exists());
    assert!(out.join("tailwind.config.ts").exists());
    assert!(out.join("postcss.config.mjs").exists());
    assert!(out.join(".eslintrc.json").exists());
    assert!(out.join(".prettierrc").exists());
    assert!(out.join("vitest.config.ts").exists());

    // Next.js without features — feature files must NOT exist
    let no_features = root.path().join("next-bare");
    let config_bare = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["nextjs".to_string()],
        project_name: no_features.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let out_bare = Scaffolder::scaffold(&config_bare).unwrap();
    assert!(!out_bare.join("prisma").join("schema.prisma").exists());
    assert!(!out_bare.join("tailwind.config.ts").exists());
    assert!(!out_bare.join(".eslintrc.json").exists());
    assert!(!out_bare.join(".prettierrc").exists());
    assert!(!out_bare.join("vitest.config.ts").exists());
}
#[test]
fn test_web_docker_templates_materialize_in_base_layer() {
    if !template_layer_ready("web/frontend/nextjs") {
        eprintln!(
            "skipped: web/frontend/nextjs template layer not available offline (registry-first)"
        );
        return;
    }
    let root = tempfile::tempdir().unwrap();

    // Frontend with docker feature
    let docker_dir = root.path().join("docker-app");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["nextjs".to_string()],
        project_name: docker_dir.to_string_lossy().to_string(),
        features: vec!["docker".to_string()],
        template_dir: PathBuf::new(),
    };
    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.join("Dockerfile").exists());
    assert!(out.join("docker-compose.yml").exists());
    assert!(out.join(".dockerignore").exists());

    // Without docker — no docker files
    let no_docker = root.path().join("no-docker");
    let config_no = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["nextjs".to_string()],
        project_name: no_docker.to_string_lossy().to_string(),
        features: vec!["typescript".to_string()],
        template_dir: PathBuf::new(),
    };
    let out_no = Scaffolder::scaffold(&config_no).unwrap();
    assert!(!out_no.join("Dockerfile").exists());
    assert!(!out_no.join("docker-compose.yml").exists());
}
#[test]
fn test_web_postgres_env_template_materializes_with_feature() {
    if !template_layer_ready("web/frontend/nextjs") {
        eprintln!(
            "skipped: web/frontend/nextjs template layer not available offline (registry-first)"
        );
        return;
    }
    let root = tempfile::tempdir().unwrap();

    let pg_dir = root.path().join("pg-app");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["nextjs".to_string()],
        project_name: pg_dir.to_string_lossy().to_string(),
        features: vec!["postgres".to_string()],
        template_dir: PathBuf::new(),
    };
    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.join(".env").exists());

    let no_pg = root.path().join("no-pg");
    let config_no = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["nextjs".to_string()],
        project_name: no_pg.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
    };
    let out_no = Scaffolder::scaffold(&config_no).unwrap();
    assert!(!out_no.join(".env").exists());
}
