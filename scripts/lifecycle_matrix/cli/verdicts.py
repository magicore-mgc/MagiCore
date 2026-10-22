"""Apply lifecycle and native-package-manager verdict rules."""

from __future__ import annotations

from ..contract.status import PASS_STATUSES, STATUS_FAILED, STATUS_UNSUPPORTED, STATUS_UNVERIFIED
from ..contract.operations import NATIVE_PM_USER_OPERATIONS
from ..validation.native_pm import native_pm_supported, native_pm_unavailable_verdict

def apply_lane_verdicts(results: list[dict]) -> None:
    """Add lifecycle and native-PM verdicts in place, without inferring evidence."""
    # Verdicts are SPLIT by claim ownership (P1-mới + P1-D, Tech Lead
    # vòng-6/7): one label can no longer cover two different claims.
    # - `orchestration-lifecycle-passed` (P0-D rename, adversarial review
    #   vòng-9): EVERY required dimension is satisfied by `passed` OR
    #   `delegated` — mgc orchestrates the lane end-to-end, regardless of
    #   who owns the cache. The bare word "supported" invited marketing
    #   to read delegation as full native support; the new name says
    #   exactly what the evidence proves: the ORCHESTRATION passed.
    # - `native-pm-supported`: every dependency-engine dimension in
    #   NATIVE_PM_REQUIRED_DIMENSIONS is native-pass, operation owners are
    #   MGC/CAS, and no package manager is delegated. General create/test/
    #   build passes alone cannot establish this claim.
    # A lane with any delegated install can still be an orchestrator but
    # is NEVER native-pm — the two claims gate different marketing and
    # different release gates.
    # Verdict TÁCH theo quyền sở hữu claim: một nhãn không thể phủ hai
    # claim khác nhau. `orchestration-lifecycle-passed` (đổi tên P0-D
    # vòng-9): MỌI dimension required thỏa bởi `passed` HOẶC `delegated`
    # — mgc điều phối lane trọn vẹn, bất kể ai giữ cache; từ "supported"
    # trần từng mời marketing đọc delegation thành native-support đầy
    # đủ, tên mới nói đúng cái bằng chứng chứng minh: ORCHESTRATION
    # pass. `native-pm-supported`: mọi dependency-engine dimension trong
    # NATIVE_PM_REQUIRED_DIMENSIONS phải native-pass, owner từng operation
    # là MGC/CAS, và không delegated package manager. Pass create/test/build
    # đơn thuần không chứng minh claim này. Đây là verdict DUY NHẤT đủ
    # chứng minh claim "native multi-language package manager". Lane có install delegated vẫn là
    # orchestrator nhưng KHÔNG BAO GIỜ native-pm — hai claim gate khác
    # nhau về marketing lẫn release.
    def _satisfied(status):
        return status in PASS_STATUSES

    for r in results:
        dims = r["dimensions"]
        required = r.pop("required_dimensions")
        r["required_dimensions"] = required
        native_pm_delegated = r.get("native_pm_delegated", [])
        install_owner = r.get("install_owner", "plain-delegation")
        # P0-6: an evidence-only lane whose external toolchain is absent
        # gets the honest `evidence-unverified` verdict — it neither passes
        # nor fails, it records an unprobed environment (never a gate).
        # (P0-6: lane evidence-only thiếu toolchain ngoài nhận verdict
        # trung thực `evidence-unverified` — không pass, không fail, ghi
        # môi trường chưa probe (không bao giờ gate).)
        if not r.get("toolchain_available", True):
            r["verdict"] = "evidence-unverified"
            r["native_pm_verdict"] = native_pm_unavailable_verdict(
                r.get("dependency_owner")
            )
            continue
        r["verdict"] = (
            "orchestration-lifecycle-passed"
            if required and all(_satisfied(dims[d]) for d in required)
            else "partial" if any(_satisfied(dims[d]) for d in required)
            else "unsupported"
        )
        # Native-PM verdict (P0-D gate, 2026-09-16): `native-pm-supported`
        # ONLY when the lane's dependency_owner is `mgc-native` — a
        # delegated toolchain can never back a native claim no matter how
        # the dimensions scored. Delegated lanes cap at the NEW
        # `compatibility-passed` verdict (toolchain compatibility proven,
        # ownership NOT mgc's); `scaffold-only`/`unsupported` lanes read
        # `unsupported`. The install-OWNER taxonomy (P0-D vòng-9) and the
        # Native dependency dimensions and per-operation ownership are also
        # required; broad lifecycle requirements alone are insufficient.
        # (Verdict native-PM (cổng P0-D): `native-pm-supported` CHỈ khi
        # dependency_owner của lane là `mgc-native` — toolchain ủy quyền
        # không bao giờ đủ cho claim native dù dimension điểm thế nào.
        # Lane delegated trần ở verdict MỚI `compatibility-passed` (chứng
        # minh tương thích toolchain, KHÔNG phải sở hữu của mgc);
        # `scaffold-only`/`unsupported` đọc `unsupported`; các chiều
        # dependency-engine và owner từng operation cũng phải được chứng minh.)
        dep_owner = r.get("dependency_owner", "")
        r["native_pm_verdict"] = (
            "native-pm-supported"
            if native_pm_supported(
                dep_owner,
                install_owner,
                native_pm_delegated,
                dims,
                r.get("owner_by_operation", {}),
            )
            else "compatibility-passed"
            if dep_owner == "delegated"
            and required
            and all(_satisfied(dims[d]) for d in required)
            else "unsupported"
            if dep_owner in ("scaffold-only", "unsupported")
            else "not-native-pm"
        )

__all__ = ["apply_lane_verdicts"]
