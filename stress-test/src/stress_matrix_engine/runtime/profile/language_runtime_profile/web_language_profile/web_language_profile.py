"""Owns web language profile probes.
Sở hữu probe web language profile riêng.
"""

from __future__ import annotations

import os

def web_language_profile(language: str, marker: str) -> tuple[str, str, str, tuple[str, ...]] | None:
    """Build one dependency-free runtime probe.
    Tạo một runtime probe không phụ thuộc package ngoài.
    """
    if language == "TypeScript":
        filename = ".mgc-stress-language.ts"
        source = f'const evidence: string = "{marker}";\nconsole.log(evidence);\n'
        return "node", filename, source, (f"node --experimental-strip-types {filename}",)
    if language == "JavaScript":
        filename = ".mgc-stress-language.mjs"
        source = f'console.log("{marker}");\n'
        return "node", filename, source, (f"node {filename}",)
    if language == "Python":
        filename = ".mgc-stress-language.py"
        source = f'print("{marker}")\n'
        interpreter = "python" if os.name == "nt" else "python3"
        return interpreter, filename, source, (f"{interpreter} {filename}",)
    if language == "Go":
        filename = "mgc_stress_language.go"
        source = (
            "package main\n\nimport \"fmt\"\n\n"
            f"func main() {{ fmt.Println(\"{marker}\") }}\n"
        )
        return "go", filename, source, (f"GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local go run -mod=readonly {filename}",)
    if language == "Dart":
        filename = ".mgc-stress-language.dart"
        source = f'void main() {{ print("{marker}"); }}\n'
        return "dart", filename, source, (f"dart {filename}",)
    if language == "Rust":
        filename = ".mgc-stress-runtime-rust/Cargo.toml"
        source = (
            "[package]\nname = \"mgc-stress-runtime-probe\"\n"
            "version = \"0.0.0\"\nedition = \"2021\"\n"
        )
        command = f"cargo run --quiet --locked --offline --manifest-path {filename}"
        return "cargo", filename, source, (command,)
    if language == "Swift":
        filename = ".mgc-stress-language.swift"
        source = f'print("{marker}")\n'
        command = (
            "swift -module-cache-path .mgc-swift-module-cache "
            "-sdk-module-cache-path .mgc-swift-sdk-module-cache "
            f"{filename}"
        )
        return "swift", filename, source, (command,)
    return None

__all__ = ["web_language_profile"]
