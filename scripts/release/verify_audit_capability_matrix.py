#!/usr/bin/env python3
"""Validate complete, fresh release audit evidence. / Xác minh evidence audit phát hành đầy đủ, mới."""
import json, os, sys

data = json.load(open("docs/specs/auditCapabilityMatrix.json"))
failures = []
required_kinds = {"clean_fixture", "vulnerable_fixture", "tool_failure"}

# Source provenance: a dirty-tree report cannot attest GITHUB_SHA.
# Provenance nguồn: report từ tree bẩn không thể chứng thực GITHUB_SHA.
if data.get("working_tree_clean") is not True:
    failures.append("matrix source tree is dirty or its cleanliness is unverified")

# 1. Format contract: a matrix without the expected manifest
#    (old schema / hand-made) cannot be evaluated — reject.
#    Hợp đồng format: matrix thiếu manifest kỳ vọng → chặn.
expected = data.get("expected_lanes")
if not expected:
    failures.append("matrix has no expected_lanes manifest — regenerate via scripts/audit/audit_capability_matrix.py")

# 2. Stale check: the matrix must be evidence at THIS commit.
#    Kiểm tra stale: matrix phải là bằng chứng tại commit này.
sha = os.environ.get("GITHUB_SHA", "")
if sha and data.get("commit") != sha:
    failures.append(f"matrix stale: commit {data.get('commit')} != GITHUB_SHA {sha}")

# 3. Every expected lane must exist AND be supported with the
#    full evidence set — partial/experimental/missing all block.
#    Mọi lane kỳ vọng phải tồn tại VÀ supported đủ evidence —
#    partial/experimental/thiếu đều bị chặn.
by_lane = {(l["core"], l["language"], l["scanner"]): l for l in data.get("capability", [])}
if expected:
    for lane in expected:
        core, lang, scanner = lane["core"], lane["language"], lane["scanner"]
        key = (core, lang, scanner)
        if key not in by_lane:
            failures.append(f"{core}/{lang} ({scanner}) = MISSING — required lane absent from matrix")
            continue
        l = by_lane[key]
        if l["status"] != "supported":
            failures.append(f"{core}/{lang} ({scanner}) = {l['status']} — release lanes must be supported with full evidence")
        elif not required_kinds <= set(l.get("evidence_kinds", [])):
            failures.append(f"{core}/{lang} ({scanner}) = supported without full evidence kinds")

# 4. Explicit missing-lane records (redundant with #3, but the
#    script names them — surface every reason).
#    Bản ghi lane thiếu được script nêu đích danh — hiển thị đủ.
for m in data.get("missing_lanes", []):
    failures.append(f"{m['core']}/{m['language']} ({m['scanner']}) = MISSING lane: {m.get('reason', '')}")

if failures:
    print("RELEASE GATE BLOCKED — audit capability not green:")
    print("\n".join(failures))
    sys.exit(1)
print(f"OK: {len(data.get('capability', []))} lanes — every expected release lane supported with full evidence at {sha[:12]}")
