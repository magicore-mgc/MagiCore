"""Owns language runtime profile dispatch.
Sở hữu điều phối language runtime profile.
"""

from __future__ import annotations

from .web_language_profile.web_language_profile import web_language_profile
from .native_language_profile.native_language_profile import native_language_profile
from .managed_language_profile.managed_language_profile import managed_language_profile
from .script_language_profile.script_language_profile import script_language_profile

def language_runtime_profile(
    language: str,
) -> tuple[str, str, str, tuple[str, ...]] | None:
    """Return a dependency-free MagiCore-run probe for qualified languages.
    Trả probe độc lập dependency để chạy runtime ngôn ngữ qua MagiCore.
    """
    marker = f"MGC_STRESS_LANGUAGE_RUNTIME:{language}"
    profile = web_language_profile(language, marker)
    if profile is not None:
        return profile
    profile = native_language_profile(language, marker)
    if profile is not None:
        return profile
    profile = managed_language_profile(language, marker)
    if profile is not None:
        return profile
    profile = script_language_profile(language, marker)
    if profile is not None:
        return profile
    return None

__all__ = ["language_runtime_profile"]
