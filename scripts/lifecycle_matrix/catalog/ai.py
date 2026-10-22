"""Lifecycle lane profiles for the ai core."""

from __future__ import annotations

LANES = [
    {
        "core": "ai",
        "language": "python",
        "scaffold": ["create-ai", "python-agent", "test-ai"],
        # Native (mgc.lock, no uv.lock): the lane adds a REAL direct
        # dependency (six, universal py2.py3-none-any wheel) via `mgc add-ai`
        # (native PyPI resolve-first + mgc-side pyproject edit) BEFORE install, so the
        # lockfile carries a genuine resolve + hash and `mgc install-ai`
        # must actually materialize it — no uv pre-steps, no uv.lock.
        # (Native: thêm dep thật bằng `mgc add-ai`, không uv.)
        "dependency_fixture": {
            "command": "add-ai",
            "package": "six@1.17.0",
        },
        "pre_steps": [
            "mgc_add_real_dependency",
            # Test/build frontends are declared through MGC before the
            # lifecycle. The Python runtime is provisioned as a toolchain;
            # packages must come from mgc.lock and the MGC store.
            # Khai báo pytest/build qua MGC trước lifecycle; Python là
            # runtime hệ thống, package phải thuộc mgc.lock và MGC store.
            "mgc_add_python_test_build_tools",
        ],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        # AI lane: the scaffold itself ships NO dependency set — the
        # install dimension is proven against a lane-injected fixture
        # (added via `mgc add-ai`, native).
        # Lane AI: scaffold không kèm dependency — dimension install
        # được chứng minh trên fixture lane tự thêm (qua `mgc add-ai`).
        "required_dims": ["create", "install", "test", "build"],
        "install_dim_name": "install-command-empty-project",
        # Native PyPI engine owns resolve/fetch/install (mgc.lock) —
        # managed-delegation record retired with the uv pre-steps.
        # (Engine PyPI native giữ resolve/fetch/install.)
        "install_owner": "native-engine",
        # mgc owns the ai/python dependency lifecycle (native).
        "dependency_owner": "mgc-native",
        "owner_by_operation": {
            "resolve": "mgc",                # PyPI JSON API engine resolves
            "lock": "mgc",                   # mgc.lock v3 (native-resolve)
            "fetch": "mgc",                  # mgc downloads wheels
            "verify": "mgc",                 # mgc checks wheel digest/RECORD — mgc xác minh digest/RECORD của wheel
            "store": "magicore-shared-cas",  # verified CAS import
            "materialize": "mgc",            # unpacked site dirs for import
        },
    },
]

__all__ = ["LANES"]
