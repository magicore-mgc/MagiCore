#!/usr/bin/env python3
"""Test-scope honesty gate.

A test whose name claims a universal scope ("every", "all", "each") must
either iterate a real source of truth or be listed here with a reason.

Why this gate exists: three separate tests in this repository said "all" or
"every" while covering a hand-written subset — `all_core_parity` checked 4 of 9
cores, `framework_registry_parity` checked 2 of 4 web tables (48 ids
unasserted), `web_framework_package_ops` covered 16 of 24 advertised
frameworks. Each was individually harmless and together they meant the
advertised surface could drift without a red test.

HOW TO EXTEND: when a test genuinely cannot iterate the registry (external
toolchain required, network-only lane, platform-specific), add it to
`REVIEWED_NARROW` with a one-line reason. The review must justify why the
claim is still meaningful — "the list is short" is not a reason.

Cổng trung thực phạm vi test: test có tên nói "every/all/each" phải lặp trên
nguồn sự thật thật, hoặc được liệt kê ở đây kèm lý do. Ba test từng nói "all"
mà chỉ kiểm một tập con viết tay — đã sửa từng cái, giờ cổng này chặn tái phát.
"""

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# Directories that hold tests.
TEST_DIRS = [
    "cli/tests",
    "core/crates",
    "adapters",
    "tests/e2e",
]

# Names that claim a universal scope over a COLLECTION of product entities.
#
# Deliberately excludes:
# - `all_<N>_...` (a stated count, e.g. `all_three_manifests` — the name is exact)
# - `all_<verb>` (a result, e.g. `all_tasks_success` — describes an outcome)
# Those were false positives; a gate that cries wolf gets muted.
# (Loại trừ `all_<số>` (đếm chính xác) và `all_<động từ>` (mô tả kết quả).)
UNIVERSAL = re.compile(
    r"\bfn\s+(?:test_)?(?:"
    r"every_[a-z0-9_]+"
    r"|each_[a-z0-9_]+"
    r"|all_(?!three_|two_|four_|five_|six_|tasks_|available_|unsupported_)[a-z0-9_]+"
    r")\("
)

# A test body that iterates a real source of truth contains at least one of
# these. Two shapes both qualify:
#   (a) a registry/capability constant — coverage follows the product;
#   (b) a match on a Rust type's variants — the compiler fails the test when a
#       variant is added without a case, which is the same guarantee.
# A frozen list of string literals satisfies neither, which is the shape that
# drifted in the three real findings.
ITERATES_SOURCE = re.compile(
    r"WEB_FRONTEND|WEB_BACKEND|WEB_FULLSTACK|WEB_ALL|WEB_ALIAS"
    r"|framework_registry::|CAPABILITIES_ALL|CAPABILITIES_PYTHON"
    r"|supported_frameworks\(|is_supported\("
    r"|DetectedRuntime::|Capability::|Ecosystem::ALL"
    r"|for\s+\w+\s+in\s+\w+\.iter\(\)"
)

# Variant coverage: the case list names a Rust enum variant, so `match`/array
# exhaustiveness (or the enum itself) drives it rather than a copy-paste list.
VARIANT_COVERAGE = re.compile(r"\b[A-Z][A-Za-z0-9]*::[A-Z][A-Za-z0-9_]*\b")

# A test that claims the whole advertised surface must touch every table; one
# that names a single family need only touch that family.
# (Test claim toàn bộ bề mặt phải chạm mọi bảng; test nêu một họ thì chỉ cần họ đó.)
CLAIMS_WHOLE_SURFACE = re.compile(r"advertised|_all_|all_")

# Web registry tables a parity test file is expected to iterate.
WEB_TABLE = re.compile(r"framework_registry::(WEB_[A-Z]+)")
WEB_TABLES_BY_FILE = {
    "cli/tests/framework_registry_parity.rs": (
        "WEB_ALIAS",
        "WEB_BACKEND",
        "WEB_FRONTEND",
        "WEB_FULLSTACK",
    ),
}

# Tests that legitimately claim universality while iterating a local literal.
# Each entry: "relative/path::test_name" -> reason.
REVIEWED_NARROW = {
    "cli/tests/all_core_parity_test.rs::test_all_core_cli_surface_uniform": (
        "iterates the nine-core literal that IS the exhaustive set; a new core "
        "must also add itself to the CLI, and `mgc capabilities` would list it "
        "with no `create-` command"
    ),
    "core/crates/mgc-types/src/manifest/test/manifest.rs::all_dependencies_yields_all_groups": (
        "builds one manifest carrying every dependency group, then asserts the "
        "combined view; the groups are fields of one struct"
    ),
    "core/crates/mgc-oidc/src/claims/oidc_claims/test/oidc_claims.rs::each_identity_binding_claim_participates_in_comparison": (
        "mutates each binding field of a sample claim in turn; the loop body names "
        "the field it mutates, so a new field is visible in review"
    ),
    "core/crates/mgc-audit/src/output/sarif/driver/test/driver.rs::every_non_available_status_uses_the_unverified_driver_name": (
        "covers all four non-available variants via the constructors "
        "`tool_missing`/`unsupported_ecosystem`/`scanner_failed` plus `Partial`"
    ),
    "adapters/web/src/lockfile/checksum/test/integrity_flags_read_their_value/mod.rs::every_enabled_spelling_turns_strict_integrity_on": (
        "the enabled spellings are the parser's own table, exercised as "
        "one-variable pairs; the disabled partner test covers the complement"
    ),
    "adapters/web/src/lockfile/checksum/test/integrity_flags_read_their_value/mod.rs::every_disabled_spelling_turns_strict_integrity_off": (
        "the disabled spellings are the parser's own table; the enabled partner "
        "test covers the complement"
    ),
    "adapters/web/src/supply_chain/test/supply_chain.rs::all_age_gate_sources_disable_resolution_reuse": (
        "iterates the age-gate source table the resolver consults, so a new source "
        "without this behaviour fails here"
    ),
    "core/crates/mgc-types/src/capabilities/optimizer_contract/test/mod.rs::every_contract_core_is_a_canonical_ecosystem_name": (
        "iterates OPTIMIZER_CAPABILITY_CORES, the product's own admission constant "
        "that the sibling test pins to an exact list, and asserts each entry maps "
        "back to a canonical Ecosystem name via Ecosystem::from_str — a new entry "
        "that is not a real ecosystem fails here"
    ),
}

# --- Self-test corpora ---------------------------------------------------
# Positive shapes the gate must ACCEPT, so tightening cannot make it reject
# honest tests and get muted.
# (Dạng cổng phải CHẤP NHẬN, để siết không làm nó từ chối test trung thực.)
POSITIVE_CORPUS = {
    "registry_loop": """
    #[test]
    fn every_framework_is_checked() {
        for framework in mgc::scaffold::framework_registry::WEB_FRONTEND {
            assert!(is_supported("web", framework));
        }
    }
    """,
    "variant_coverage": """
    #[test]
    fn every_status_is_named() {
        for status in [
            ScannerStatus::Partial { scanned: 1, skipped: 1, reasons: vec![] },
            ScannerStatus::ToolMissing { tool: "t".into(), remediation: "r".into() },
        ] {
            assert!(!build(status).is_empty());
        }
    }
    """,
    "parser_table": """
    #[test]
    fn every_spelling_is_parsed() {
        let table = spellings();
        for value in table.iter() {
            assert!(parse(value));
        }
    }
    """,
}

# Negative shapes the gate must REJECT — one per drift class the gate exists for.
# (Dạng cổng phải TỪ CHỐI — mỗi lớp lệch một mục.)
NEGATIVE_CORPUS = {
    "hand_written_subset": """
    #[test]
    fn every_claimed_framework_runs() {
        for framework in ["vanilla", "react", "vue"] {
            assert!(ok(framework));
        }
    }
    """,
    "partial_registry_tables": """
    #[test]
    fn every_advertised_framework_pair_is_supported() {
        for framework in mgc::scaffold::framework_registry::WEB_BACKEND {
            assert!(is_supported("web", framework));
        }
    }
    """,
}


def strip_test_modules(source: str) -> str:
    """Remove `#[cfg(test)]` regions so production code does not trip the gate."""
    out = []
    depth = 0
    inside = False
    for line in source.splitlines():
        if not inside and re.search(r"#\[cfg\(test\)\]", line):
            inside = True
            depth = 0
            continue
        if inside:
            depth += line.count("{") - line.count("}")
            if depth <= 0 and "{" in line:
                inside = False
            continue
        out.append(line)
    return "\n".join(out)


def test_files():
    for rel in TEST_DIRS:
        base = os.path.join(ROOT, rel)
        for dirpath, dirnames, filenames in os.walk(base):
            dirnames[:] = [
                name
                for name in dirnames
                if name not in {"target", "node_modules", ".git"}
            ]
            for name in filenames:
                if not name.endswith(".rs"):
                    continue
                path = os.path.join(dirpath, name)
                relpath = os.path.relpath(path, ROOT)
                parts = relpath.split(os.sep)
                if "tests" not in parts and "test" not in parts:
                    continue
                yield relpath, path


def body_after(source: str, match_end: int) -> str:
    """Return the text of the function starting at `match_end` (best effort)."""
    brace = source.find("{", match_end)
    if brace == -1:
        return ""
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace : index + 1]
    return source[brace:]


def test_name_of(source: str) -> str:
    """The test's own name, used for scope claims like "advertised"."""
    match = re.search(r"fn\s+([a-z0-9_]+)\(", source)
    return match.group(1) if match else ""


def judge_body(body: str, relpath: str, test_name: str = "") -> str:
    """Return "" when the body is acceptable, else the reason it is not.

    The multi-table requirement applies only to names that claim the WHOLE
    advertised surface ("advertised", "all"). A test that says "every framework"
    and iterates `WEB_FRONTEND` is honest — it never claimed the backend tables.
    Applying the requirement to every name in the file rejected two honest
    corpus entries, which the self-test caught.
    (Ràng buộc đa-bảng chỉ áp cho tên nói về TOÀN BỘ bề mặt. Test nói "every
    framework" mà lặp `WEB_FRONTEND` là trung thực — nó chưa từng claim bảng
    backend. Áp cho mọi tên đã từ chối hai corpus trung thực, self-test bắt được.)
    """
    if not ITERATES_SOURCE.search(body):
        if len(VARIANT_COVERAGE.findall(body)) >= 2:
            return ""
        return "does not iterate a source of truth"
    offered = set(WEB_TABLES_BY_FILE.get(relpath, ()))
    if not offered:
        return ""
    if not CLAIMS_WHOLE_SURFACE.search(test_name):
        return ""
    missing = sorted(offered - set(WEB_TABLE.findall(body)))
    if missing:
        return f"missing {missing}"
    return ""


def self_test() -> int:
    """Prove the gate both accepts honest tests and catches every drift form."""
    failures = []
    parity_file = "cli/tests/framework_registry_parity.rs"

    for name, source in POSITIVE_CORPUS.items():
        match = UNIVERSAL.search(source)
        if not match:
            failures.append(f"{name}: corpus entry has no universal name to judge")
            continue
        reason = judge_body(
            body_after(source, match.end()), parity_file, test_name_of(source)
        )
        if reason:
            failures.append(f"{name}: honest test was rejected ({reason})")

    for name, source in NEGATIVE_CORPUS.items():
        match = UNIVERSAL.search(source)
        if not match:
            failures.append(f"{name}: corpus entry is not recognised as universal")
            continue
        reason = judge_body(
            body_after(source, match.end()), parity_file, test_name_of(source)
        )
        if not reason:
            failures.append(f"{name}: drift form was accepted — the gate is blind")

    if failures:
        print("FAIL: test-scope gate self-test")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print(
        f"OK: self-test accepts {len(POSITIVE_CORPUS)} honest shapes and rejects "
        f"{len(NEGATIVE_CORPUS)} drift forms"
    )
    return 0


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()

    failures = []
    checked = 0
    for relpath, path in test_files():
        with open(path, encoding="utf-8", errors="ignore") as handle:
            source = strip_test_modules(handle.read())
        for match in UNIVERSAL.finditer(source):
            checked += 1
            name_match = re.search(r"fn\s+([a-z0-9_]+)\(", match.group(0))
            if not name_match:
                continue
            name = name_match.group(1)
            key = f"{relpath}::{name}"
            if key in REVIEWED_NARROW:
                continue
            reason = judge_body(body_after(source, match.end()), relpath, name)
            if not reason:
                continue
            failures.append((key, name, reason))

    if failures:
        print("FAIL: universal-scope test names without a source-of-truth loop")
        for key, name, reason in failures:
            print(f"  {key}")
            print(f"    '{name}' claims every/all/each but {reason}.")
            print(
                "    Either iterate the real source (preferred) or add the test to"
                " REVIEWED_NARROW with a reason."
            )
        return 1

    print(
        f"OK: {checked} universal-scope test name(s) backed by a source-of-truth loop"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
