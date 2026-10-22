"""Owns script language profile probes.
Sở hữu probe script language profile riêng.
"""

from __future__ import annotations

def script_language_profile(language: str, marker: str) -> tuple[str, str, str, tuple[str, ...]] | None:
    """Build one dependency-free runtime probe.
    Tạo một runtime probe không phụ thuộc package ngoài.
    """
    if language == "PHP":
        filename = ".mgc-stress-language.php"
        source = f'<?php echo "{marker}\\n";\n'
        return "php", filename, source, (f"php -n {filename}",)
    if language == "Shell":
        filename = ".mgc-stress-language.sh"
        source = f'#!/usr/bin/env bash\nprintf \'%s\\n\' "{marker}"\n'
        return "bash", filename, source, (f"bash {filename}",)
    if language == "GDScript":
        filename = ".mgc-stress-godot/probe.gd"
        source = (
            "extends SceneTree\n"
            "func _initialize():\n"
            f'    print("{marker}")\n'
            "    quit()\n"
        )
        command = (
            "godot --headless --path .mgc-stress-godot "
            "--script probe.gd --quit"
        )
        return "godot", filename, source, (command,)
    if language == "Terraform HCL":
        filename = ".mgc-stress-language.tf"
        source = (
            'output "mgc_stress_language_runtime" {\n'
            f'  value = "{marker}"\n'
            '}\n'
        )
        return "terraform", filename, source, (f"terraform fmt -check {filename}",)
    return None

__all__ = ["script_language_profile"]
