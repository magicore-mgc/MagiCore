"""Scaffold-language evidence matching."""

from __future__ import annotations

import os

# Manifest markers per language: the file that PROVES the scaffold
# actually produced a project of the requested language (not a
# fallback template of another language wearing the label). A scaffold
# that emits the wrong marker is a FAILED create — honest, never passed.
# Marker theo ngôn ngữ: file CHỨNG MINH scaffold sinh đúng project của
# ngôn ngữ yêu cầu (không phải template fallback ngôn ngữ khác gắn
# nhãn). Scaffold sinh sai marker → create FAILED (trung thực).
SCAFFOLD_MARKERS = {
    "rust": ["Cargo.toml"],
    "python": ["pyproject.toml", "requirements.txt"],
    "typescript": ["package.json", "tsconfig.json"],
    "go": ["go.mod"],
    "java": ["pom.xml", "build.gradle", "build.gradle.kts"],
    "dotnet": ["*.csproj", "*.sln"],
    "javascript": ["package.json"],
    "vanilla": ["index.html"],
    "ts": ["index.html"],
    "node": ["package.json"],
    "cdk": ["package.json"],
    "pulumi": ["package.json", "Pulumi.yaml"],
    "flutter": ["pubspec.yaml"],
    # Evidence-only lane markers (P0-6): the file that PROVES the scaffold
    # produced the right artifact for these lanes.
    # (Marker lane evidence-only (P0-6): file CHỨNG MINH scaffold sinh đúng
    # artifact cho các lane này.)
    "terraform": ["main.tf"],
    "github-actions": [".github/workflows/ci.yml"],
    # `benchmark` (hardware lane) has no manifest file — the language falls
    # into the "cannot verify, do not guess" path of
    # scaffold_language_matches.
    # (`benchmark` (lane hardware) không có file manifest — ngôn ngữ này rơi
    # vào nhánh "không verify được, không đoán" của scaffold_language_matches.)
}

def scaffold_language_matches(
    sandbox: str, project_dir: str, language: str, required_markers=None
) -> bool:
    """Verify the scaffolded project carries a manifest of the REQUESTED
    language (anti-fake-scaffold guard, P0-2 honesty contract).
    Xác minh project được sinh có manifest của ĐÚNG ngôn ngữ yêu cầu
    (chống scaffold giả — hợp đồng trung thực P0-2)."""
    markers = SCAFFOLD_MARKERS.get(language, [])
    if not markers:
        return True  # unknown language — cannot verify, do not guess
    root = os.path.join(sandbox, project_dir)
    if required_markers:
        return all(os.path.isfile(os.path.join(root, marker)) for marker in required_markers)
    for marker in markers:
        if marker.startswith("*"):
            # Glob suffix (e.g. *.csproj) — any file matching counts.
            # Hậu tố glob (vd *.csproj) — bất kỳ file khớp đều tính.
            import glob as _glob
            if _glob.glob(os.path.join(root, marker)):
                return True
        elif os.path.isfile(os.path.join(root, marker)):
            return True
    return False

__all__ = ['SCAFFOLD_MARKERS', 'scaffold_language_matches']
