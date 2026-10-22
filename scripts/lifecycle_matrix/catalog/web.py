"""Lifecycle lane profiles for the web core."""

from __future__ import annotations

LANES = [
    {
        "core": "web",
        "language": "javascript",
        "scaffold": ["create-web", "react", "test-web"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
            # `dev` is PROBED (dev_probe below), not stepped: a
            # long-lived server can only be proven by SERVING + HMR
            # roundtrip, never by `--help` exit-0 (which only proves
            # Clap wiring — Tech Lead P0-mới-1 vòng-6/7).
            # `dev` được PROBE (dev_probe bên dưới), không chạy bước đơn:
            # server dài hạn chỉ chứng minh được bằng PHỤC VỤ + HMR
            # roundtrip, không bao giờ bằng exit-0 của `--help` (chỉ
            # chứng minh Clap wiring — Tech Lead P0-mới-1).
        ],
        "delegated": [],
        # Web carries the product's flagship promise — run + dev are part
        # of the lane's own lifecycle contract (P0 finding #1: the
        # verdict must weigh every dimension the lane declares).
        # Web giữ lời hứa chủ lực của product — run + dev thuộc hợp đồng
        # lifecycle của chính lane (P0 finding #1: verdict phải cân mọi
        # dimension lane tuyên bố).
        "required_dims": ["create", "install", "test", "build", "dev", "run"],
        # The WEB ENGINE (mgc itself) owns resolve+lock+fetch+CAS — the
        # only native-engine install today (P0-D taxonomy).
        # (Engine WEB (chính mgc) giữ resolve+lock+fetch+CAS — install
        # native-engine duy nhất hôm nay (taxonomy P0-D).)
        "install_owner": "native-engine",
        "lifecycle_owner_overrides": {"dev": "mgc-native"},
        "recovery_probe": True,
        # P0-D: the web engine IS mgc — the flagship mgc-native lane.
        # (P0-D: engine web CHÍNH LÀ mgc — lane mgc-native chủ lực.)
        "dependency_owner": "mgc-native",
        "owner_by_operation": {
            "resolve": "mgc",                # web engine resolves the graph
            "lock": "mgc",                   # mgc.lock
            "fetch": "mgc",                  # mgc fetcher + CAS
            "verify": "mgc",                 # web engine checks registry integrity — web engine xác minh integrity registry
            "store": "magicore-shared-cas",  # global shared CAS
            "materialize": "mgc",            # mgc materializes node_modules
        },
        # `run` proves the production entry (`mgc run start` → vite
        # preview): a long-lived server, so the probe starts it, waits
        # for an HTTP roundtrip, then kills it cleanly — the dimension
        # passes when the server SERVED, not merely started.
        # `run` chứng minh entry production (`mgc run start` → vite
        # preview): server dài hạn nên probe khởi động, chờ HTTP
        # roundtrip, rồi tắt sạch — dimension pass khi server ĐÃ PHỤC
        # VỤ, không chỉ khởi động.
        "run_probe": {
            "script": "start",
            "port": 4315,
        },
        # `dev` probe (P0-mới-1, Tech Lead vòng-6/7): the OLD step was
        # `["dev", "--help"]` — exit-0 proved ONLY the Clap wiring while
        # the lane still claimed a `dev` dimension. The real probe spawns
        # `mgc dev`, waits for the port to bind + HTTP 200, edits a
        # source file, expects NEW server output (rebuild), checks the
        # HMR websocket endpoint, then kills the tree and sweeps the
        # port. dev passes only when the server SERVED + rebuilt.
        # Probe `dev` (P0-mới-1): bước cũ `["dev","--help"]` — exit-0 chỉ
        # chứng minh Clap wiring trong khi lane vẫn claim dimension dev.
        # Probe thật spawn `mgc dev`, chờ port bind + HTTP 200, sửa file
        # source, chờ output mới (rebuild), kiểm tra websocket HMR, rồi
        # kill cây process và quét port. dev chỉ pass khi server ĐÃ PHỤC
        # VỤ + rebuild.
        "dev_probe": {
            "port": 4315,
            "hmr_path": "/@magicore/hmr",
            "source_file": "src/App.tsx",
            "source_edit": "// dev-probe touch — marker HMR rebuild\n",
        },
    },
    {
        "core": "web",
        "language": "vanilla",
        "framework_ids": ["vanilla"],
        "framework_id": "vanilla",
        "scaffold": ["create-web", "vanilla", "test-web-vanilla"],
        "required_markers": ["index.html"],
        "steps": [("install", ["install"]), ("test", ["test"]), ("build", ["build"])],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {},
    },
    {
        "core": "web",
        "language": "ts",
        "framework_ids": ["ts"],
        "framework_id": "ts",
        "scaffold": ["create-web", "vanilla", "test-web-ts", "--ts"],
        "project_dir": "test-web-ts",
        "required_markers": ["index.html", "src/main.ts", "tsconfig.json"],
        "steps": [("install", ["install"]), ("test", ["test"]), ("build", ["build"])],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {},
    },
    {
        "core": "web",
        "language": "node",
        "framework_ids": ["node"],
        "framework_id": "node",
        "scaffold": ["create-web", "express", "test-web-node"],
        "steps": [("install", ["install"]), ("test", ["test"]), ("build", ["build"])],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {},
    },
]

__all__ = ["LANES"]
