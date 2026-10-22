"""Lifecycle lane profiles for the iot core."""

from __future__ import annotations

LANES = [
    {
        "core": "iot",
        "language": "rust",
        "evidence_only": True,
        # esp32-rust lane: scaffold emits an embedded-rust Cargo.toml; the
        # full cross toolchain (xtensa/riscv target) may still be missing —
        # later steps fail honestly if so.
        # (lane esp32-rust: scaffold sinh embedded-rust Cargo.toml; cross
        # toolchain đầy đủ (target xtensa/riscv) có thể vẫn thiếu — các
        # bước sau fail trung thực nếu vậy.)
        "toolchain_probes": ["cargo"],
        "scaffold": ["create-iot", "esp32-rust", "test-iot"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        # ESP32-Rust's CLI dependency route uses MGC's Lib/Rust engine;
        # PlatformIO/Zephyr lanes remain unsupported.
        # (ESP32-Rust dùng engine Lib/Rust của MGC; PlatformIO/Zephyr
        # vẫn unsupported.)
        "dependency_owner": "mgc-native",
        "note": "ESP32-Rust dependencies route through MGC Lib/Rust resolver; board compiler/toolchain is a separate build boundary",
        "owner_by_operation": {},
    },
]

__all__ = ["LANES"]
