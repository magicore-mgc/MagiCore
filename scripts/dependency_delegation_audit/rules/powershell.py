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
        "source_sha256": "a316accc4ffec26366b501e3707036bc969088e9336999bf41a044125cac076f",
        "lines": {22: "Version-probes the fixed Swift installation selected after validating its version."},
    },
    "scripts/ci/lifecycle/swift/preserve_python_windows.ps1": {
        "source_sha256": "489d2fd6c0dcc91f8eb2bc75f00df72d5140a7793d49c503c040cb4121652b0a",
        "lines": {3: "Version-probes the Python executable supplied by the pinned setup-python action.", 14: "Version-probes the preserved Python executable copied from the validated setup-python installation."},
    },
    "scripts/ci/stress/toolchains/gcc/expose_windows.ps1": {
        "source_sha256": "f84491f869c1e904400442fb6da18ecb2be7918ecab70c2815132d55707b8659",
        "lines": {12: "Version-probes GCC from the selected directory only after confirming both compiler executables exist.", 13: "Version-probes G++ from the selected directory only after confirming both compiler executables exist."},
    },
    "scripts/ci/stress/toolchains/godot/install_windows.ps1": {
        "source_sha256": "89eb28f8907d488c97c76a0b7fcd01dce01370acdf12901f25ffaac704bf1b65",
        "lines": {16: "Version-probes Godot after validating the official archive checksum."},
    },
    "scripts/ci/stress/toolchains/kotlin/install_windows.ps1": {
        "source_sha256": "4cca0cc0c489297d2a701cca95a3ce669b0739982334db1cff59834786dc680e",
        "lines": {13: "Version-probes Kotlin from the installed official compiler distribution."},
    },
    "scripts/ci/stress/toolchains/swift/install_windows.ps1": {
        "source_sha256": "383cc237eb24c2084da11c10b3ab5e28ca144b2cb5ff0e3552e345ccb3319d85",
        "lines": {20: "Version-probes the Swift installation after validating the installer version."},
    },
    "scripts/ci/stress/toolchains/terraform/install_windows.ps1": {
        "source_sha256": "8e7c27b73f58b000173fa3f7372aa61976b5d6415bc63f94e3c1344f01c8c156",
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
