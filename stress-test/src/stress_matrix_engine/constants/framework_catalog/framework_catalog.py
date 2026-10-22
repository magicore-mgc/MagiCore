"""Owns the framework catalog responsibility in the stress-matrix runner.
Sở hữu trách nhiệm framework catalog trong stress-matrix runner.
"""

from __future__ import annotations

from ..evidence_categories.evidence_categories import FRAMEWORK_RUNTIME_CATEGORIES

WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK = {
    "nuxt": "web-fw-native-matrix",
    "vanilla": "web-fw-native-matrix",
    "ts": "web-fw-native-matrix",
    "node": "web-fw-native-matrix",
    "express": "web-fw-native-matrix",
    "fastify": "web-fw-native-matrix",
    "hono": "web-fw-native-matrix",
    "nestjs": "web-fw-native-matrix",
    "qwik": "web-fw-native-matrix",
    "solidjs": "web-fw-native-matrix",
    "trpc": "web-fw-native-matrix",
    "angular": "web-fw-frontend-template",
    "astro": "web-fw-frontend-template",
    "nextjs": "web-fw-frontend-template",
    "react-vite": "web-fw-frontend-template",
    "remix": "web-fw-frontend-template",
    "sveltekit": "web-fw-frontend-template",
    "vue-vite": "web-fw-frontend-template",
}

UNIVERSAL_FRAMEWORK_CATEGORIES = (
    "framework-scaffold",
    "framework-language-scaffold",
    "mgc-install-empty-graph",
    *FRAMEWORK_RUNTIME_CATEGORIES,
    "optimizer",
    "cache",
    "cache-runtime",
    "security-audit",
    "security-audit-nonempty",
    "shared-store-scope",
    "shared-store-runtime",
    "mgc-lock-v3-empty-graph",
    "mgc-lock-v3-nonempty",
    "mgc-lock-v4-empty-graph",
    "mgc-lock-v4-nonempty",
)

UNIVERSAL_GLOBAL_CATEGORIES = (
    "cli-command-help",
    "security-cli-command",
    "security-cli",
    "shared-store",
)

WEB_FRAMEWORK_ALIASES = {
    "ts": "vanilla",
    "frontend": "vanilla",
    "custom": "vanilla",
    "backend": "express",
    "node": "express",
    "fullstack": "remix",
    "go": "gin",
    "java": "spring-boot",
    "php": "laravel",
    "python": "fastapi",
    "rust": "axum",
    "dotnet": "dotnet-minimal",
}

IOT_BOARD_FRAMEWORKS = {
    "esp32": "esp32-rust",
    "esp32c3": "esp32-rust",
    "esp32s3": "esp32-rust",
    "esp32dev": "platformio",
    "nodemcu-32s": "platformio",
    "nrf52dk_nrf52832": "zephyr-arm",
    "stm32f4_disc": "zephyr-arm",
}

__all__ = ['WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK', 'UNIVERSAL_FRAMEWORK_CATEGORIES', 'UNIVERSAL_GLOBAL_CATEGORIES', 'WEB_FRAMEWORK_ALIASES', 'IOT_BOARD_FRAMEWORKS']
