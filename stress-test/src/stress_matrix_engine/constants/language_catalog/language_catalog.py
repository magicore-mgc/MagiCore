"""Owns the language catalog responsibility in the stress-matrix runner.
Sở hữu trách nhiệm language catalog trong stress-matrix runner.
"""

from __future__ import annotations

SOURCE_SUFFIXES = {
    ".c", ".cc", ".cpp", ".cs", ".css", ".dart", ".gd", ".go", ".h",
    ".hpp", ".html", ".java", ".js", ".jsx", ".kt", ".m", ".mm", ".php",
    ".astro", ".py", ".rs", ".sh", ".swift", ".tf", ".ts", ".tsx", ".vue", ".zig",
}

SOURCE_LANGUAGE_BY_SUFFIX = {
    ".c": "C",
    ".cc": "C++",
    ".cpp": "C++",
    ".cs": "C#",
    ".css": "CSS",
    ".dart": "Dart",
    ".gd": "GDScript",
    ".go": "Go",
    ".h": "C/C++ header",
    ".hpp": "C++",
    ".html": "HTML",
    ".java": "Java",
    ".js": "JavaScript",
    ".jsx": "JavaScript",
    ".kt": "Kotlin",
    ".m": "Objective-C",
    ".mm": "Objective-C++",
    ".php": "PHP",
    ".astro": "Astro component",
    ".py": "Python",
    ".rs": "Rust",
    ".sh": "Shell",
    ".swift": "Swift",
    ".tf": "Terraform HCL",
    ".ts": "TypeScript",
    ".tsx": "TypeScript",
    ".vue": "Vue SFC",
    ".zig": "Zig",
}

AUTOMATION_PATHS = {
    ".github/workflows/ci.yml",
    "cloudbuild.yaml",
    "buildspec.yml",
    "argocd/application.yaml",
}

RUNTIME_PROBE_LANGUAGE_ORDER = (
    "TypeScript",
    "JavaScript",
    "Python",
    "Go",
    "Dart",
    "Rust",
    "Swift",
    "C",
    "C++",
    "C#",
    "Java",
    "Kotlin",
    "PHP",
    "Shell",
    "GDScript",
    "Objective-C",
    "Objective-C++",
    "Zig",
    "Terraform HCL",
)

NON_STANDALONE_RUNTIME_FORMATS = frozenset(
    {
        "Astro component",
        "C/C++ header",
        "CI workflow YAML",
        "CSS",
        "HTML",
        "JSON hardware descriptor",
        "Vue SFC",
    }
)

__all__ = ['SOURCE_SUFFIXES', 'SOURCE_LANGUAGE_BY_SUFFIX', 'AUTOMATION_PATHS', 'RUNTIME_PROBE_LANGUAGE_ORDER', 'NON_STANDALONE_RUNTIME_FORMATS']
