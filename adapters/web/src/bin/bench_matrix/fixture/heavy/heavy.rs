use crate::bench_matrix::fixture::package_id;
use crate::bench_matrix::fixture::resolved_pkg;
// Heavy benchmark fixture
// Định nghĩa workload heavy và package graph đã đóng băng.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn heavy_fixture() -> FixtureSpec {
    let react = package_id("react", "18.2.0");
    let react_dom = package_id("react-dom", "18.2.0");
    let vite = package_id("vite", "5.4.10");
    let typescript = package_id("typescript", "5.5.4");
    let tailwind = package_id("tailwindcss", "4.3.2");
    let query = package_id("@tanstack/react-query", "5.59.0");
    let admin_shell = package_id("@workspace/admin-shell", "1.0.0");
    let legacy_tool = package_id("legacy-tool", "1.0.0");

    let scheduler = package_id("scheduler", "0.23.2");
    let design_system = package_id("@workspace/design-system", "1.4.0");
    let react_router = package_id("react-router", "7.0.1");
    let postcss = package_id("postcss", "8.4.47");
    let query_core = package_id("@tanstack/query-core", "5.59.0");
    let semver7 = package_id("semver", "7.8.5");
    let shared_util_v2 = package_id("@workspace/shared-util", "2.1.0");
    let semver6 = package_id("semver", "6.3.1");

    let source_map = package_id("source-map-js", "1.2.1");
    let color_tokens = package_id("@workspace/color-tokens", "1.0.2");
    let icon_pack = package_id("@workspace/icon-pack", "2.3.0");
    let history = package_id("history", "5.3.0");
    let tslib = package_id("tslib", "2.7.0");
    let shared_util_v1 = package_id("@workspace/shared-util", "1.8.4");
    let autoprefixer = package_id("autoprefixer", "10.4.20");
    let browserslist = package_id("browserslist", "4.24.0");
    let caniuse = package_id("caniuse-lite", "1.0.30001667");

    let direct_dependencies = vec![
        react.clone(),
        react_dom.clone(),
        vite.clone(),
        typescript.clone(),
        tailwind.clone(),
        query.clone(),
        admin_shell.clone(),
        legacy_tool.clone(),
    ];

    let graph = ResolvedGraph {
        packages: vec![
            resolved_pkg(react.clone(), vec![scheduler.clone()], true, 48),
            resolved_pkg(
                react_dom.clone(),
                vec![react.clone(), scheduler.clone()],
                true,
                48,
            ),
            resolved_pkg(
                vite.clone(),
                vec![typescript.clone(), semver7.clone()],
                true,
                48,
            ),
            resolved_pkg(typescript.clone(), vec![tslib.clone()], true, 48),
            resolved_pkg(
                tailwind.clone(),
                vec![postcss.clone(), autoprefixer.clone()],
                true,
                48,
            ),
            resolved_pkg(query.clone(), vec![query_core.clone()], true, 48),
            resolved_pkg(
                admin_shell.clone(),
                vec![
                    design_system.clone(),
                    react_router.clone(),
                    shared_util_v2.clone(),
                    color_tokens.clone(),
                ],
                true,
                48,
            ),
            resolved_pkg(
                legacy_tool.clone(),
                vec![shared_util_v1.clone(), semver6.clone()],
                true,
                48,
            ),
            resolved_pkg(scheduler.clone(), vec![], false, 48),
            resolved_pkg(
                design_system.clone(),
                vec![
                    color_tokens.clone(),
                    icon_pack.clone(),
                    shared_util_v1.clone(),
                ],
                false,
                48,
            ),
            resolved_pkg(
                react_router.clone(),
                vec![history.clone(), react.clone()],
                false,
                48,
            ),
            resolved_pkg(
                postcss.clone(),
                vec![source_map.clone(), browserslist.clone()],
                false,
                48,
            ),
            resolved_pkg(query_core.clone(), vec![tslib.clone()], false, 48),
            resolved_pkg(semver7.clone(), vec![], false, 48),
            resolved_pkg(shared_util_v2.clone(), vec![tslib.clone()], false, 48),
            resolved_pkg(semver6.clone(), vec![], false, 48),
            resolved_pkg(source_map.clone(), vec![], false, 48),
            resolved_pkg(color_tokens.clone(), vec![], false, 48),
            resolved_pkg(icon_pack.clone(), vec![shared_util_v1.clone()], false, 48),
            resolved_pkg(history.clone(), vec![], false, 48),
            resolved_pkg(tslib.clone(), vec![], false, 48),
            resolved_pkg(shared_util_v1.clone(), vec![], false, 48),
            resolved_pkg(
                autoprefixer.clone(),
                vec![browserslist.clone(), postcss.clone()],
                false,
                48,
            ),
            resolved_pkg(browserslist.clone(), vec![caniuse.clone()], false, 48),
            resolved_pkg(caniuse.clone(), vec![], false, 48),
        ],
    };

    FixtureSpec {
        direct_dependencies,
        graph,
        files_per_package: 48,
        scenario_prefix: "heavy",
    }
}
