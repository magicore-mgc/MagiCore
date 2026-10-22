"""Shared scanner roots and common process policy.

Language-specific match tables live in their local rule owner."""

from __future__ import annotations

# Source roots covered by each language-aware pass. Rust roots include
# first-party adapters, CLI, crates, tests, and tools; Python roots are listed
# separately so new entry points cannot hide in only one implementation tree.
# (Root source được quét theo từng parser ngôn ngữ. Rust gồm adapter, CLI,
# crates, tests và tools; Python khai riêng để entry point không lọt vì chỉ
# quét một cây implementation.)
SCAN_ROOTS = [
    "adapters",
    # Scan all first-party Rust production and test source roots; selecting
    # only adapters or command subtrees could miss a new crate/entry point.
    # (Quét mọi source Rust first-party và test; chọn vài subtree có thể bỏ
    # sót crate hay entry point mới.)
    "cli",
    "core",
    "tests",
    "tools",
]

# Known blind spots remain explicit in the ledger. They keep
# `coverage_complete=false`, but do not override a clean direct-boundary gate.
# (Điểm mù vẫn được ghi trong ledger và giữ `coverage_complete=false`; chúng
# không đổi kết quả gate khi mọi ranh giới trực tiếp đã được rà.)
UNSCANNED_PROCESS_SURFACES = [
    {
        "path": "Python indirect aliases and third-party process wrappers",
        "reason": "AST inventory detects direct stdlib calls but does not perform dataflow or wrapper resolution",
    },
    {
        "path": "PowerShell and JavaScript indirect/computed process calls",
        "reason": "PowerShell detects direct cmdlets and common built-in process aliases but remains lexical and does not resolve arbitrary user-defined aliases/data-flow; JavaScript covers common import/computed/static aliases but not arbitrary alias/data-flow, template-literal expressions, reflective or module-loader wrappers",
    },
    {
        "path": "shell wrappers and non-listed executable commands",
        "reason": "the shell pass inventories direct known-tool command heads only; it does not resolve sourced files, functions, eval, or arbitrary executables",
    },
    {
        "path": "shell interpreter startup environment",
        "reason": "BASH_ENV and similar hooks may run before an entrypoint script starts; the invoking runner must sanitize startup variables before launching Bash, while this pass checks only child launches visible inside scanned scripts",
    },
    {
        "path": ".github/workflows",
        "reason": "immutable action-reference syntax has a dedicated CI gate; command provisioning and upstream SHA provenance remain incomplete",
    },
]

# The forbidden PM toolchains (V1.2 audit list) — spawning any of these in a
# dependency operation is delegation that must be declared. python/python3
# cover model-quantize passthrough; pip-audit/cargo-audit/govulncheck cover
# audit-via-external-scanner (advisory metadata is a core-owned operation).
# (Toolchain PM bị cấm — spawn trong operation dependency là uỷ quyền phải
# khai báo. python/python3 phủ quantize passthrough; pip-audit/cargo-audit/
# govulncheck phủ audit qua scanner ngoài (metadata advisory là việc cốt lõi
# MGC phải sở hữu).)
TOOLS = [
    "cargo", "uv", "pip", "pip3", "go", "gradle", "mvn", "dotnet", "flutter",
    "dart", "swift", "pod", "npm", "pnpm", "yarn", "bun", "deno", "pio",
    "west", "terraform", "git", "python", "python3", "pip-audit", "cargo-audit",
    "govulncheck", "composer", "pub", "npx", "bunx", "uvx", "pipx",
]

# Diagnostic probes may inspect installed tools, but product workflows may
# not silently hand build/test/run/dev/deploy/flash ownership to a CLI.
# Test and benchmark source paths are handled separately below.
# (Chỉ cho phép probe chẩn đoán; workflow production không được tự động
# giao quyền build/test/run/dev/deploy/flash cho CLI ngoài.)
ALLOWED_OPS = ("doctor",)

ECOSYSTEM_OWNED_OPS = ("build", "test", "run", "dev", "flash", "deploy")

# Dependency lifecycle operations — when one of these words names the
# function or a path segment, the file is in dependency scope even if the
# function name also carries a whitelist word (e.g. `run_install`). audit /
# scan / hook / quantize are core-owned operations too: advisory metadata,
# hook dispatch on dependency events, and model quantization must not
# silently shell out to external toolchains.
# (Operation lifecycle dependency — khi một từ này nằm trong tên hàm hay
# segment đường dẫn, file thuộc phạm vi dependency dù tên hàm có cả từ
# whitelist, vd `run_install`. audit/scan/hook/quantize cũng là việc cốt
# lõi: metadata advisory, điều phối hook trên event dependency và quantize
# model không được âm thầm gọi toolchain ngoài.)
DEPENDENCY_OPS = (
    "install", "add", "remove", "update", "resolve", "fetch", "lock", "cache",
    "audit", "scan", "hook", "quantize",
)

# The declaration marker (Việc 1 / P0-B convention).
# (Marker khai báo uỷ quyền (quy ước Việc 1 / P0-B).)
MARKER = "DELEGATED:"

# Message macros whose string arguments NEVER execute as processes
# (`format!("...deno...")` names the tool in prose, it does not spawn it).
# The weak bare-literal pattern is skipped inside these spans; every
# call-shaped pattern (exec_tool/run/Command::new/which/tool-tables)
# still matches inside them, so a real spawn can never hide behind a
# message macro.
# (Macro message mà đối số chuỗi KHÔNG BAO GIỜ chạy như process
# (`format!("...deno...")` chỉ nhắc tool trong văn bản, không spawn.
# Pattern bare-literal yếu được bỏ qua trong các span này; mọi pattern
# dạng-call vẫn khớp bên trong, nên spawn thật không thể núp sau macro
# message.)
MESSAGE_MACROS = (
    "format!", "eprintln!", "println!", "bail!", "anyhow!", "panic!",
    "assert!", "debug_assert!", "unreachable!", "todo!", "unimplemented!",
    "write!", "writeln!", "eprint!", "print!",
)

# Internal routing sentinels are values, not executable names. Keep the set
# exact so it cannot become a general bypass for unlisted process targets.
# (Sentinel điều phối nội bộ không phải tên executable; allowlist chính xác.)
NON_EXECUTABLE_SENTINELS = {"mgc-internal-run-script"}

NON_EXECUTABLE_SQL_PREFIXES = {
    "ALTER", "CREATE", "DELETE", "DROP", "INSERT", "PRAGMA", "SELECT",
    "UPDATE", "WITH",
}
