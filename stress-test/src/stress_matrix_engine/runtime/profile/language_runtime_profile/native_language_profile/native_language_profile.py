"""Owns native language profile probes.
Sở hữu probe native language profile riêng.
"""

from __future__ import annotations

import os
import sys

def native_language_profile(language: str, marker: str) -> tuple[str, str, str, tuple[str, ...]] | None:
    """Build one dependency-free runtime probe.
    Tạo một runtime probe không phụ thuộc package ngoài.
    """
    if language == "Zig":
        filename = ".mgc-stress-language.zig"
        source = (
            'const std = @import("std");\n'
            "pub fn main() void {\n"
            f'    std.debug.print("{marker}\\n", .{{}});\n'
            "}\n"
        )
        return "zig", filename, source, (f"zig run {filename}",)
    if language == "C":
        filename = ".mgc-stress-language.c"
        source = (
            "#include <stdio.h>\n"
            f'int main(void) {{ puts("{marker}"); return 0; }}\n'
        )
        output = ".mgc-stress-language-c.exe" if os.name == "nt" else ".mgc-stress-language-c"
        executable_path = f"./{output}"
        compiler = "clang" if sys.platform == "darwin" else "gcc"
        return (
            compiler,
            filename,
            source,
            (
                f"{compiler} {filename} -o {output}",
                executable_path,
            ),
        )
    if language == "C++":
        filename = ".mgc-stress-language.cpp"
        source = (
            "#include <iostream>\n"
            f'int main() {{ std::cout << "{marker}" << std::endl; return 0; }}\n'
        )
        output = ".mgc-stress-language-cpp.exe" if os.name == "nt" else ".mgc-stress-language-cpp"
        executable_path = f"./{output}"
        compiler = "clang++" if sys.platform == "darwin" else "g++"
        return (
            compiler,
            filename,
            source,
            (
                f"{compiler} {filename} -o {output}",
                executable_path,
            ),
        )
    if language == "Objective-C" and sys.platform == "darwin":
        filename = ".mgc-stress-language.m"
        source = (
            "#import <Foundation/Foundation.h>\n"
            "#include <stdio.h>\n"
            "int main(void) { @autoreleasepool { "
            f'printf("%s\\n", "{marker}"); '
            "} return 0; }\n"
        )
        output = ".mgc-stress-language-objc"
        return (
            "clang",
            filename,
            source,
            (
                f"clang -x objective-c {filename} -framework Foundation -o {output}",
                f"./{output}",
            ),
        )
    if language == "Objective-C++" and sys.platform == "darwin":
        filename = ".mgc-stress-language.mm"
        source = (
            "#import <Foundation/Foundation.h>\n"
            "#include <iostream>\n"
            "int main() { @autoreleasepool { "
            f'std::cout << "{marker}" << std::endl; '
            "} return 0; }\n"
        )
        output = ".mgc-stress-language-objcxx"
        return (
            "clang++",
            filename,
            source,
            (
                f"clang++ -x objective-c++ {filename} -framework Foundation -o {output}",
                f"./{output}",
            ),
        )
    return None

__all__ = ["native_language_profile"]
