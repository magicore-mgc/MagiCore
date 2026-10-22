use crate::allowlist::*;

#[test]
fn rejects_forbidden_pm_anywhere_in_script() {
    for script in [
        "vite --host localhost --port 4315 && npm install",
        "env PNPM_HOME=/tmp pnpm add zod",
        "node ./x.js; /usr/bin/yarn install",
        "\"bun\" install",
        "cmd /c npx vite",
        "bunx create-react-app demo",
    ] {
        let err = reject_forbidden_pm_script(script).unwrap_err();
        assert!(
            err.to_string().contains("forbidden package manager"),
            "unexpected error for {script}: {err}"
        );
    }
}

#[test]
fn rejects_package_manager_javascript_entrypoints() {
    use crate::allowlist::find_forbidden_tool_in_script;

    for (script, expected) in [
        ("node ./node_modules/pnpm/bin/pnpm.cjs install", "pnpm"),
        ("node ./node_modules/npm/bin/npm-cli.js install", "npm"),
        ("node .\\node_modules\\yarn\\bin\\yarn.js install", "yarn"),
    ] {
        assert_eq!(
            find_forbidden_tool_in_script(script),
            Some(expected),
            "must detect package-manager entrypoint in {script}"
        );
    }
}

#[test]
fn rejects_package_manager_invoked_from_inline_javascript() {
    use crate::allowlist::find_forbidden_tool_in_script;

    let script = r#"node -e "const cmd='pnpm install'; require('child_process').execSync(cmd)""#;
    assert_eq!(
        find_forbidden_tool_in_script(script),
        Some("pnpm"),
        "must detect a package manager named inside inline JavaScript"
    );
}

#[test]
fn allows_framework_local_scripts_without_pm_wrapper() {
    reject_forbidden_pm_script("vite --host localhost --port 4315").unwrap();
    reject_forbidden_pm_script("next dev --hostname localhost --port 4315").unwrap();
    reject_forbidden_pm_script("node ./node_modules/.bin/vite --host localhost").unwrap();
}
