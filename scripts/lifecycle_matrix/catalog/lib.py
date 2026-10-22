"""Lifecycle lane profiles for the lib core."""

from __future__ import annotations

LANES = [
    {
        "core": "lib",
        "language": "rust",
        "framework_ids": ["rust"],
        "framework_id": "rust",
        "scaffold": ["create-lib", "rust", "test-lib"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        # cargo owns fetch/test/build; mgc orchestrates via mgc.lock.
        # `run` is an npm-script concept — honest absent for lib/rust.
        # cargo giữ fetch/test/build; mgc điều phối qua mgc.lock. `run`
        # là khái niệm npm-script — absent trung thực cho lib/rust.
        "delegated": [],
        # Dimensions this lane MUST pass for "orchestration-lifecycle-passed" —
        # the verdict contract is per-lane, not global (P0 finding #1).
        # Dimension lane này BẮT BUỘC pass để "orchestration-lifecycle-passed" —
        # hợp đồng verdict theo từng lane, không theo tập toàn cục (P0
        # finding #1).
        "required_dims": ["create", "install", "test", "build"],
        # Phase 2 (2026-09-16): lib/rust resolves+fetches+materializes via
        # the native crates.io engine (mgc-resolver) — mgc OWNS the install
        # lifecycle, no `cargo fetch` spawn. Verdict stays honest (evidence:
        # unit + mockito; runtime lane not yet proven in this matrix).
        # (Phase 2: lib/rust resolve+fetch+materialize qua engine crates.io
        # native (mgc-resolver) — mgc GIỮ lifecycle install, không spawn
        # `cargo fetch`. Verdict giữ trung thực (evidence: unit + mockito;
        # runtime lane chưa chứng minh trong matrix này).)
        "install_owner": "native-engine",
        # Native crates.io engine exists (unit + mockito evidence).
        # (Engine crates.io native tồn tại (evidence unit + mockito).)
        "dependency_owner": "mgc-native",
        "note": "native-engine: exists, evidence: unit+mockito",
        "owner_by_operation": {
            "resolve": "mgc",                # mgc sparse-index engine resolves
            "lock": "mgc",                   # mgc.lock v3 (native-resolve)
            "fetch": "mgc",                  # mgc downloads the .crate
            "verify": "mgc",                 # mgc checks crates.io checksum — mgc xác minh checksum crates.io
            "store": "magicore-shared-cas",  # blake3 CAS import
            "materialize": "mgc",            # mgc writes registry/{cache,src} layout
        },
    },
    {
        "core": "lib",
        "language": "python",
        "framework_ids": ["python"],
        "framework_id": "python",
        "scaffold": ["create-lib", "python", "test-lib"],
        # lib/python needs pytest (mgc test) and build (python -m build)
        # provisioned before its lifecycle steps run.
        # lib/python cần pytest (mgc test) và build (python -m build)
        # được provision trước khi chạy các bước lifecycle.
        "pre_steps": [
            "mgc_add_python_test_build_tools",
        ],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        # Phase 2 (2026-09-16): lib/python resolves+fetches+materializes via
        # the native PyPI engine (mgc-resolver) — mgc OWNS the install
        # lifecycle, no `uv sync`/`pip install` spawn. Verdict stays honest
        # (evidence: unit + mockito).
        # (Phase 2: lib/python resolve+fetch+materialize qua engine PyPI
        # native (mgc-resolver) — mgc GIỮ lifecycle install, không spawn
        # `uv sync`/`pip install`. Verdict giữ trung thực (unit + mockito).)
        "install_owner": "native-engine",
        # Native PyPI engine exists (unit + mockito evidence).
        # (Engine PyPI native tồn tại (evidence unit + mockito).)
        "dependency_owner": "mgc-native",
        "note": "native-engine: exists, evidence: unit+mockito",
        "owner_by_operation": {
            "resolve": "mgc",                # mgc JSON API engine resolves
            "lock": "mgc",                   # mgc.lock v3 (native-resolve)
            "fetch": "mgc",                  # mgc downloads wheel/sdist
            "verify": "mgc",                 # mgc checks PyPI wheel digests — mgc xác minh digest wheel PyPI
            "store": "magicore-shared-cas",  # blake3 CAS import
            "materialize": "mgc",            # mgc writes pypi/wheels layout
        },
    },
    {
        "core": "lib",
        "language": "typescript",
        "framework_ids": ["ts"],
        "framework_id": "ts",
        "scaffold": ["create-lib", "typescript", "test-lib"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        # TypeScript rides the WEB engine (mgc resolve+lock+fetch+CAS)
        # — a native-engine candidate (P0-D).
        # (TypeScript đi trên engine WEB (mgc resolve+lock+fetch+CAS)
        # — ứng viên native-engine (P0-D).)
        "install_owner": "native-engine",
        "recovery_probe": True,
        # P0-D: the web engine IS mgc — lib/typescript is mgc-native.
        # (P0-D: engine web CHÍNH LÀ mgc — lib/typescript là mgc-native.)
        "dependency_owner": "mgc-native",
        "owner_by_operation": {
            "resolve": "mgc",                # web engine resolves the graph
            "lock": "mgc",                   # mgc.lock
            "fetch": "mgc",                  # mgc fetcher + CAS
            "verify": "mgc",                 # web engine checks registry integrity — web engine xác minh integrity registry
            "store": "magicore-shared-cas",  # global shared CAS
            "materialize": "mgc",            # mgc materializes node_modules
        },
    },
    {
        "core": "lib",
        "language": "go",
        "framework_ids": ["go"],
        "framework_id": "go",
        "scaffold": ["create-lib", "go", "test-lib"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        # No delegated lifecycle steps (native install; test/build run
        # under mgc's own exec, same precedent as lib/python).
        "delegated": [],
        # Phase 2 native (2026-09-16): the GoMod proxy engine
        # (protocols/go.rs) resolves/downloads/verifies modules — install
        # is mgc-owned now; go toolchain remains for build/test only.
        # (Phase 2 native: engine GoMod proxy (protocols/go.rs) tự
        # resolve/download/verify module — install là của mgc; go toolchain
        # chỉ còn build/test.)
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {
            "resolve": "mgc",                # GoMod proxy engine resolves the graph
            "lock": "mgc",                   # mgc.lock v3 (native-resolve)
            "fetch": "mgc",                  # mgc downloads @v zips
            "verify": "mgc",                 # mgc checks ziphash/sumdb — mgc xác minh ziphash/sumdb
            "store": "magicore-shared-cas",  # blake3 CAS import
            "materialize": "mgc",            # GOMODCACHE layout for offline builds
        },
    },
    {
        "core": "lib",
        "language": "java",
        "framework_ids": ["java"],
        "framework_id": "java",
        # Maven dependency resolution is native, but Java test/build execution
        # has no MGC-owned backend yet; retain its evidence without release claims.
        # (Resolver Maven đã native, nhưng Java chưa có backend test/build do MGC giữ.)
        "evidence_only": True,
        "scaffold": ["create-lib", "java", "test-lib"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        # No delegated lifecycle steps (native install; test/build run
        # under mgc's own exec, same precedent as lib/python).
        "delegated": [],
        # Phase 2 native (2026-09-16): the Maven engine (protocols/maven.rs)
        # resolves POM graphs and verifies jars (sha256/sha1) — install is
        # mgc-owned for pom.xml projects; gradle stays a build-lane tool.
        # (Phase 2 native: engine Maven (protocols/maven.rs) tự resolve graph
        # POM và verify jar (sha256/sha1) — install là của mgc cho project
        # pom.xml; gradle chỉ là tool build-lane.)
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {
            "resolve": "mgc",                # Maven engine resolves the POM graph
            "lock": "mgc",                   # mgc.lock v3 (native-resolve)
            "fetch": "mgc",                  # mgc downloads jars
            "verify": "mgc",                 # mgc checks published SHA-256/SHA-1 — mgc xác minh SHA-256/SHA-1 đã publish
            "store": "magicore-shared-cas",  # sha256/sha1-verified CAS import
            "materialize": "mgc",            # M2 repository layout for offline builds
        },
    },
    {
        "core": "lib",
        "language": "dotnet",
        "framework_ids": ["dotnet"],
        "framework_id": "dotnet",
        # NuGet resolve/fetch/install is native, but build/test cannot run
        # without MSBuild's restore-generated assets. MGC does not generate
        # that state, and §5.1 does not allow the dotnet executable.
        # Resolve/fetch/install NuGet là native, nhưng build/test cần assets
        # do MSBuild restore tạo. MGC chưa sinh state này và §5.1 chưa cho
        # phép chạy executable dotnet.
        "evidence_only": True,
        "scaffold": ["create-lib", "dotnet", "test-lib"],
        "steps": [
            ("install", ["install"]),
        ],
        "lifecycle_owner_overrides": {
            "test": "unsupported",
            "build": "unsupported",
        },
        "delegated": [],
        # Phase 2 native (2026-09-16): the NuGet v3 engine
        # (protocols/nuget.rs) resolves flat-container versions and
        # verifies SHA-512 nupkgs — install is mgc-owned.
        # (Phase 2 native: engine NuGet v3 (protocols/nuget.rs) tự resolve
        # flat-container versions và verify SHA-512 nupkg — install là của
        # mgc.)
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {
            "resolve": "mgc",               # NuGet v3 engine resolves
            "lock": "mgc",                  # mgc.lock v3 (native-resolve)
            "fetch": "mgc",                 # mgc downloads nupkgs
            "verify": "mgc",                # mgc checks NuGet hashes — mgc xác minh hash NuGet
            "store": "magicore-shared-cas", # sha512-verified CAS import
            "materialize": "mgc",           # global-packages layout for offline restore
        },
    },
]

__all__ = ["LANES"]
