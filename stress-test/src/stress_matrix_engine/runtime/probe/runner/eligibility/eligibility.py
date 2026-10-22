"""Owns language probe eligibility checks.
Sở hữu kiểm tra điều kiện áp dụng language probe.
"""

from __future__ import annotations

import sys

from .....constants.language_catalog.language_catalog import NON_STANDALONE_RUNTIME_FORMATS
from .....result.result import Result

def probe_eligibility(subject: str, language: str) -> Result | None:
    """Return an explicit result when a standalone probe does not apply.
    Trả kết quả rõ ràng khi standalone probe không áp dụng.
    """
    if language == "Objective-C" and sys.platform != "darwin":
        return Result(
            "framework-language-runtime-probe", subject, True,
            "Objective-C runtime probe is applicable only on macOS",
            evidence_status="not_applicable",
        )
    if language == "Objective-C++" and sys.platform != "darwin":
        return Result(
            "framework-language-runtime-probe", subject, True,
            "Objective-C++ runtime probe is applicable only on macOS",
            evidence_status="not_applicable",
        )
    if language in NON_STANDALONE_RUNTIME_FORMATS:
        return Result(
            "framework-language-runtime-probe", subject, True,
            f"{language} requires its framework/toolchain lifecycle rather than a standalone language runtime",
            evidence_status="not_applicable",
        )
    return None

__all__ = ["probe_eligibility"]
