"""Lifecycle lane profiles for the game core."""

from __future__ import annotations

LANES = [
    # ===== Lifecycle evidence-only lanes (P0-6, Tech Lead 2026-09-15) =====
    # These lanes record lifecycle evidence without gating lifecycle
    # promotion. Missing external toolchains produce `unverified` dimensions
    # and `evidence-unverified`, never a pass. Owner taxonomy remains declared;
    # MGC-native package claims still require the independent native-PM gate.
    # (Các lane này ghi lifecycle evidence nhưng không chặn lifecycle
    # promotion. Thiếu toolchain ngoài ghi dimension `unverified` và verdict
    # `evidence-unverified`, không giả pass. Owner vẫn phải khai; claim package
    # MGC-native vẫn phải qua gate native-PM riêng.)
    {
        "core": "game",
        "language": "rust",
        "evidence_only": True,
        # bevy lane: cargo is the whole toolchain (probe), the scaffold
        # writes a Cargo.toml (marker "rust" already proves it).
        # (lane bevy: cargo là toàn bộ toolchain (probe), scaffold ghi
        # Cargo.toml — marker "rust" chứng minh sẵn.)
        "toolchain_probes": ["cargo"],
        "scaffold": ["create-game", "bevy", "test-game"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        # The CLI routes dependency installation through MGC's Lib/Rust
        # engine; Cargo remains a compiler/build boundary, not the installer.
        # (CLI route cài dependency qua engine Lib/Rust của MGC; Cargo chỉ
        # là ranh giới compiler/build, không phải installer.)
        "dependency_owner": "mgc-native",
        "note": "CLI routes Bevy dependencies through MGC Lib/Rust resolver; build/test still use compiler toolchain",
        "owner_by_operation": {},
    },
]

__all__ = ["LANES"]
