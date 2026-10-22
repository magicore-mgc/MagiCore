"""Ecosystem materialization marker checks."""

from __future__ import annotations

from ..process.process import (
    run_text_capture,
)
from typing import Optional
import os
import shutil
import subprocess

def _materialize_marker(
    project_dir: str, language: str, environment: Optional[dict[str, str]] = None
):
    """Return (label, path) whose non-emptiness PROVES the dependency tree
    landed after install, or (None, None) when this ecosystem has no
    observable marker — which records `unverified`, never a pass.
    (Trả về (nhãn, đường_dẫn) mà việc không-rỗng CHỨNG MINH cây dependency
    đã thật sự về sau install, hoặc (None, None) khi ecosystem không có
    marker quan sát được — ghi `unverified`, không bao giờ pass.)"""
    if language in ("javascript", "typescript", "pulumi"):
        return ("node_modules", os.path.join(project_dir, "node_modules"))
    if language == "python":
        return (".venv", os.path.join(project_dir, ".venv"))
    if language == "rust":
        # Rust deps never land in-project — cargo materializes them into
        # the MGC-managed global store ($HOME/.magicore/store/cargo, see
        # adapters/lib/src/install/shared_store.rs). That managed store is
        # the materialization surface for this lane.
        # (Deps Rust không bao giờ về trong project — cargo materialize
        # vào CARGO_HOME do mgc quản (~/.magicore/store/cargo). Store quản
        # lý đó là mặt materialize của lane này.)
        home = (environment or os.environ).get("HOME") or os.path.expanduser("~")
        return ("mgc-cargo-store", os.path.join(home, ".magicore", "store", "cargo"))
    if language == "go":
        # The go module cache lives at GOMODCACHE — observable only when
        # the go toolchain is on PATH; otherwise unverified.
        # (Module cache của go nằm ở GOMODCACHE — chỉ quan sát được khi go
        # có trên PATH; không thì unverified.)
        env = environment or os.environ
        go = shutil.which("go", path=env.get("PATH"))
        if go is None:
            return (None, None)
        try:
            proc = run_text_capture(
                [go, "env", "GOMODCACHE"], capture_output=True, text=True,
                timeout=30, env=env,
            )
        except (OSError, subprocess.TimeoutExpired):
            return (None, None)
        cache = (proc.stdout or "").strip()
        return ("gomodcache", cache) if cache else (None, None)
    if language == "flutter":
        return (".dart_tool", os.path.join(project_dir, ".dart_tool"))
    # java/dotnet: no reliable post-install marker without running the
    # toolchain build — honestly unobservable here.
    # (java/dotnet: không có marker đáng tin sau install nếu không chạy
    # build toolchain — trung thực là không quan sát được ở đây.)
    return (None, None)

__all__ = ['_materialize_marker']
