"""Powershell implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations

PS_PROCESS_APIS = ("Start-Process", "Start-Job", "Invoke-Expression", "Invoke-Command")

# Built-in PowerShell aliases that can evaluate or start commands. Keep aliases
# separate so findings identify the underlying process boundary, not a tool name.
# Alias PowerShell dựng sẵn có thể chạy hoặc đánh giá lệnh; ánh xạ về cmdlet gốc.
PS_PROCESS_ALIASES = {
    "start": "Start-Process",
    "saps": "Start-Process",
    "iex": "Invoke-Expression",
    "icm": "Invoke-Command",
    "sajb": "Start-Job",
}

# Whole-source hashes bind reviewed PowerShell process sites to the audited
# scripts, including their guards and installer checksum checks.
# Hash toàn file ràng buộc các lệnh process với guard và checksum đã rà soát.
POWERSHELL_SOURCE_PROCESS_REVIEWS = {
    "scripts/ci/lifecycle/swift/install_windows.ps1": {
        "source_sha256": "498a5a1442ad9b117e4ce6ab78c07511be72ddda0071cf110a7f87b502fd4a82",
        "lines": {22: "Version-probes the fixed Swift installation selected after validating its version."},
    },
    "scripts/ci/lifecycle/swift/preserve_python_windows.ps1": {
        "source_sha256": "ffaba5a1a6b041686811f496106069221372167a106084f6f4f1a33da35df04b",
        "lines": {3: "Version-probes the Python executable supplied by the pinned setup-python action.", 14: "Version-probes the preserved Python executable copied from the validated setup-python installation."},
    },
    "scripts/ci/stress/toolchains/gcc/expose_windows.ps1": {
        "source_sha256": "be7aac72b9ab3c514845c62987aa7a119605ceedf748cbdeb52205e8d21348bf",
        "lines": {12: "Version-probes GCC from the selected directory only after confirming both compiler executables exist.", 13: "Version-probes G++ from the selected directory only after confirming both compiler executables exist."},
    },
    "scripts/ci/stress/toolchains/godot/install_windows.ps1": {
        "source_sha256": "8a411f959cf121ce05bbd07cf9c4aca7744bca59d1d56ca02554dca1c75dadf3",
        "lines": {16: "Version-probes Godot after validating the official archive checksum."},
    },
    "scripts/ci/stress/toolchains/kotlin/install_windows.ps1": {
        "source_sha256": "c43dac28d3d207e0029942fa12f34475b9ecc5db12a4d2985872b783f20b25f7",
        "lines": {13: "Version-probes Kotlin from the installed official compiler distribution."},
    },
    "scripts/ci/stress/toolchains/swift/install_windows.ps1": {
        "source_sha256": "8c326bf911b73d8de233987079130877aad1470627d322458055aa213c77afd3",
        "lines": {20: "Version-probes the Swift installation after validating the installer version."},
    },
    "scripts/ci/stress/toolchains/terraform/install_windows.ps1": {
        "source_sha256": "ef9b421628b8eaeb9edf50b097f32434d68bfe21ac9d98550cb49dca53778386",
        "lines": {10: "Version-probes Terraform after validating the downloaded archive checksum."},
    },
    "scripts/release/artifact_smoke/web_install.ps1": {
        "source_sha256": "99f66f7119ae423dfac72eb7279996d898ae617d4eebe00209dbd2a813128808",
        "lines": {8: "Runs the release artifact selected by the smoke matrix with the fixed web-install command."},
    },
    "scripts/release/binary_e2e/toolchain/install_swift_windows.ps1": {
        "source_sha256": "a12b9bb34bf7998f85a5801273827f4e2c243b916aeb3212259d724533caeb3e",
        "lines": {11: "Executes the official Swift installer only after checksum and Authenticode signature verification.", 32: "Version-probes the Swift binary from the verified installation directory."},
    },
}

__all__ = []
