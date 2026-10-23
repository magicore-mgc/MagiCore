"""Lockfile discovery and entry counting."""

from __future__ import annotations

import json
import re

def _lockfile_candidates(project_dir: str, language: str) -> list:
    """Lockfile artifacts that can prove resolve+lock per language: a list
    of (name, kind) probes in evidence-preference order. kind is
    'mgc-json' (mgc.lock — top-level "package" list), 'toml-packages'
    ([[package]] sections), 'text-lines' (non-comment lines), 'pubspec'
    (pubspec.lock `packages:` section) or 'json-deps' (a dependencies
    object). An empty list means no known artifact — the lock/resolve
    dimensions stay `unverified` for that language, never a guessed pass.
    (Các artifact lockfile có thể chứng minh resolve+lock theo ngôn ngữ:
    danh sách (tên, kiểu) theo thứ tự ưu tiên bằng chứng. Danh sách rỗng =
    không có artifact nào biết — dimension lock/resolve giữ `unverified`
    cho ngôn ngữ đó, không bao giờ đoán pass.)"""
    if language in ("javascript", "typescript", "pulumi"):
        # mgc.lock is written as TOML `[[package]]` on disk (see
        # core/crates/mgc-lockfile/src/export — toml::to_string_pretty);
        # the JSON spelling is a legacy fallback probe, never a guess.
        # (mgc.lock ghi ra đĩa là TOML `[[package]]`; JSON là probe dự
        # phòng cho legacy, không phải đoán.)
        return [("mgc.lock", "mgc-lock"), ("package-lock.json", "json-deps")]
    if language == "rust":
        # mgc.lock carries the mgc-owned lock claim
        # (owner_by_operation.lock); Cargo.lock is cargo's own — both are
        # lock evidence, and the per-operation owner taxonomy names who
        # owns what.
        # (mgc.lock mang claim lock của mgc (owner_by_operation.lock);
        # Cargo.lock là của cargo — cả hai đều là bằng chứng lock, taxonomy
        # owner per-operation nêu rõ ai giữ gì.)
        return [("mgc.lock", "mgc-lock"), ("Cargo.lock", "toml-packages")]
    if language == "python":
        return [("uv.lock", "toml-packages")]
    if language == "go":
        return [("go.sum", "text-lines")]
    if language == "flutter":
        return [("pubspec.lock", "pubspec")]
    if language == "java":
        return [("gradle.lockfile", "text-lines")]
    if language == "dotnet":
        return [("packages.lock.json", "json-deps")]
    return []

def _lockfile_entry_count(path: str, kind: str):
    """Best-effort entry count for a lockfile probe; None when the content
    cannot be interpreted — which records `unverified`, never a pass.
    (Đếm mục lockfile theo kiểu probe; None khi không đọc hiểu được nội
    dung — khi đó ghi `unverified`, không bao giờ pass.)"""
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as f:
            content = f.read()
    except OSError:
        return None
    if kind == "mgc-lock":
        # On-disk mgc.lock is TOML with one `[[package]]` table per
        # resolved package; some legacy writers may emit JSON with a
        # top-level "package" list — accept either, count what parses.
        # (mgc.lock trên đĩa là TOML với một bảng `[[package]]` mỗi
        # package; writer legacy có thể ghi JSON với list "package" —
        # nhận cả hai, đếm đúng cái parse được.)
        toml_count = _lockfile_entry_count(path, "toml-packages")
        if toml_count:
            return toml_count
        json_count = _lockfile_entry_count(path, "mgc-json")
        return json_count
    if kind == "mgc-json":
        try:
            data = json.loads(content)
        except ValueError:
            return None
        pkgs = data.get("package") if isinstance(data, dict) else None
        return len(pkgs) if isinstance(pkgs, list) else None
    if kind == "toml-packages":
        # Cargo.lock / uv.lock carry one `[[package]]` array entry per
        # resolved package — a structural count that needs no TOML parser
        # (older Pythons have no stdlib tomllib; the regex marker is
        # stable across both lockfile formats).
        # (Cargo.lock / uv.lock có một mục `[[package]]` cho mỗi package
        # đã resolve — đếm cấu trúc không cần parser TOML; marker regex
        # ổn định trên cả hai format lockfile.)
        return len(re.findall(r"^\s*\[\[package\]\]", content, re.M))
    if kind == "text-lines":
        lines = [
            ln for ln in content.splitlines()
            if ln.strip() and not ln.strip().startswith("#")
        ]
        return len(lines) if lines else 0
    if kind == "pubspec":
        # pubspec.lock: a `packages:` section with one `  name:` line per
        # entry (two-space indent); nested keys indent deeper.
        # (pubspec.lock: mục `packages:` với một dòng `  tên:` mỗi entry
        # (thụt 2 dấu cách); khóa lồng nhau thụt sâu hơn.)"""
        if "packages:" not in content:
            return 0
        return len(re.findall(r"^ {2}[^ ].*:$", content, re.M))
    if kind == "json-deps":
        try:
            data = json.loads(content)
        except ValueError:
            return None
        if not isinstance(data, dict):
            return None
        deps = data.get("dependencies")
        if isinstance(deps, dict):
            return len(deps)
        if isinstance(deps, list):
            return len(deps)
        return None
    return None

__all__ = ['_lockfile_candidates', '_lockfile_entry_count']
