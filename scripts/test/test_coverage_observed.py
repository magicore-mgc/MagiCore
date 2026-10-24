#!/usr/bin/env python3
"""Validate the observed coverage manifest against its schema.

The manifest must also keep at least one lane per claim family, so a broken
producer cannot turn evidence into silence.
Manifest phải còn ít nhất một lane cho mỗi nhóm claim, để producer hỏng không
biến bằng chứng thành im lặng.
"""

import json
import pathlib
import sys

try:
    import jsonschema
except ImportError:  # pragma: no cover - CI installs it
    print("jsonschema is required: pip install jsonschema", file=sys.stderr)
    raise SystemExit(1)

ROOT = pathlib.Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "ci" / "evidence" / "coverage-manifest.schema.json"
OBSERVED = ROOT / "ci" / "evidence" / "coverage-manifest.observed.json"

# Every claim family the manifest can make must be representable, but a family
# may legitimately be absent once its lanes are proven (for example no lane is
# left `unverified`). What must never vanish is positive evidence.
# Mọi nhóm claim phải biểu diễn được, nhưng một nhóm có thể vắng hợp lệ khi lane
# của nó đã có bằng chứng (ví dụ không còn lane `unverified`). Cái không được
# biến mất là bằng chứng tích cực.
REQUIRED_CLAIMS = {"supported", "native-experimental", "scaffold-only"}
KNOWN_CLAIMS = REQUIRED_CLAIMS | {"unverified", "unsupported", "delegated"}


def main() -> int:
    if not OBSERVED.is_file():
        print(f"missing {OBSERVED.relative_to(ROOT)}; run emit_coverage_observed.py", file=sys.stderr)
        return 1
    manifest = json.loads(OBSERVED.read_text())
    jsonschema.validate(manifest, json.loads(SCHEMA.read_text()))

    statuses = {lane["status"] for lane in manifest["lanes"]}
    unknown = statuses - KNOWN_CLAIMS
    if unknown:
        print(f"observed manifest has unknown statuses: {sorted(unknown)}", file=sys.stderr)
        return 1
    missing = REQUIRED_CLAIMS - statuses
    if missing:
        print(f"observed manifest dropped claim families: {sorted(missing)}", file=sys.stderr)
        return 1
    duplicates = len(manifest["lanes"]) - len(
        {(l["core"], l["framework"], l["operation"], l["target"]) for l in manifest["lanes"]}
    )
    if duplicates:
        print(f"observed manifest has {duplicates} duplicate lanes", file=sys.stderr)
        return 1
    print(f"observed coverage manifest OK: {len(manifest['lanes'])} lanes, {sorted(statuses)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
