"""Lifecycle lane profiles for the app core."""

from __future__ import annotations

LANES = [
    {
        "core": "app",
        "language": "flutter",
        "scaffold": ["create-app", "flutter", "test-app"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        # Native (mgc.lock, no uv.lock): install runs the shared PyPI
        # engine (parse → resolve → verified fetch → mgc.lock); test and
        # build run under mgc's own exec. No uv anywhere on this lane.
        # (Native: install qua engine PyPI, không uv.)
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        # mgc owns the app/flutter dependency lifecycle (native).
        "dependency_owner": "mgc-native",
        "note": "native pub.dev engine owns resolve/fetch/install (mgc.lock + pub cache); test/build run under mgc exec",
        "owner_by_operation": {
            "resolve": "mgc",                # pub.dev JSON API engine resolves
            "lock": "mgc",                   # mgc.lock v3 (native-resolve)
            "fetch": "mgc",                  # mgc downloads archives
            "verify": "mgc",                 # mgc checks pub archive SHA-256 — mgc xác minh SHA-256 archive pub
            "store": "magicore-shared-cas",  # verified CAS import
            "materialize": "mgc",            # pub cache layout for offline builds
        },
    },
    # ===== Phase 2 native app lanes (2026-09-16) — evidence-only until
    # release E2E exists. SwiftPM engine (protocols/swift.rs), RN layered
    # engine (protocols/reactnative.rs: JS web delegate + gradle.lockfile
    # -> Maven + Podfile.lock -> CocoaPods CDN).
    # (Phase 2 native app lanes — chỉ-ghi-bằng-chứng cho tới khi có release
    # E2E. Engine SwiftPM (protocols/swift.rs), engine RN layered
    # (protocols/reactnative.rs: JS delegate web + gradle.lockfile ->
    # Maven + Podfile.lock -> CocoaPods CDN).) =====
    {
        "core": "app",
        "language": "swift",
        "evidence_only": True,
        "toolchain_probes": ["swift"],
        "scaffold": ["create-app", "swift", "test-app"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "required_dims": ["create", "install", "test", "build"],
        # The app CLI now routes registry dependencies through MGC's Swift
        # resolver/fetch/verify/materializer. Source/Git dependencies remain
        # explicitly unsupported; install ownership is not full ecosystem parity.
        # (CLI app dùng resolver/fetch/verify/materializer Swift của MGC;
        # dependency source/Git vẫn unsupported.)
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "note": "MGC-native Swift registry install; add/remove/update/list remain unsupported; Git dependencies are rejected",
        "owner_by_operation": {},
    },
    {
        "core": "app",
        "language": "objc",
        "evidence_only": True,
        "toolchain_probes": ["pod"],
        "scaffold": ["create-app", "objc", "test-app"],
        "steps": [("install", ["install"])],
        "required_dims": ["create", "install"],
        # Objective-C dependency install is rejected before external tools.
        # (Cài dependency Objective-C bị từ chối trước khi gọi tool ngoài.)
        "install_owner": "unsupported",
        "dependency_owner": "unsupported",
        "note": "unsupported: no MGC-native ObjC resolver/materializer and no compatibility spawn in this lane",
        "owner_by_operation": {},
    },
    {
        "core": "app",
        "language": "react-native",
        "evidence_only": True,
        "toolchain_probes": ["node"],
        "scaffold": ["create-app", "react-native", "test-app"],
        "steps": [("install", ["install"])],
        "required_dims": ["create", "install"],
        # C0 (T0.3, 2026-09-17): React Native has PER-TIER ownership (no
        # single-row native label): JS tier rides the web pipeline, Android
        # tier resolves via the native Maven engine, iOS tier verifies
        # Podfile.lock via the CocoaPods CDN engine — but the invoked
        # `install-app` lane has NO runner and errors before any spawn, so
        # the lane as-invoked supports no install lifecycle: unsupported.
        # (C0: React Native sở hữu PER-TIER (không nhãn native đơn dòng):
        # lane install không có runner nên lỗi — unsupported.)
        "install_owner": "unsupported",
        # P0-D/T0.4: no install lifecycle exists on the invoked lane.
        "dependency_owner": "unsupported",
        "note": "per-tier: js=web-pipeline, android=Maven-engine, ios=CocoaPods-CDN-verify; invoked lane has no runner and errors (fail-closed); iOS tier fail-closed without Podfile.lock",
        "owner_by_operation": {},
    },
]

__all__ = ["LANES"]
