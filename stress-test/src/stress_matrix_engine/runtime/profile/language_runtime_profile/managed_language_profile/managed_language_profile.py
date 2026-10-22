"""Owns managed language profile probes.
Sở hữu probe managed language profile riêng.
"""

from __future__ import annotations

def managed_language_profile(language: str, marker: str) -> tuple[str, str, str, tuple[str, ...]] | None:
    """Build one dependency-free runtime probe.
    Tạo một runtime probe không phụ thuộc package ngoài.
    """
    if language == "C#":
        filename = ".mgc-stress-dotnet/Program.cs"
        source = f'Console.WriteLine("{marker}");\n'
        command = f"dotnet run --no-restore --file {filename}"
        return "dotnet", filename, source, (command,)
    if language == "Java":
        filename = "MgcStressLanguageProbe.java"
        source = (
            "class MgcStressLanguageProbe {\n"
            "  public static void main(String[] args) {\n"
            f'    System.out.println("{marker}");\n'
            "  }\n"
            "}\n"
        )
        command = (
            "java -XX:-UsePerfData -Djava.io.tmpdir=.mgc-java-tmp "
            f"{filename}"
        )
        return "java", filename, source, (command,)
    if language == "Kotlin":
        filename = ".mgc-stress-language.kt"
        source = f'fun main() {{ println("{marker}") }}\n'
        jar = ".mgc-stress-language-kotlin.jar"
        compile_command = f"kotlinc {filename} -include-runtime -d {jar}"
        execute_command = (
            f"java -XX:-UsePerfData -Djava.io.tmpdir=.mgc-java-tmp -jar {jar}"
        )
        return "kotlinc", filename, source, (compile_command, execute_command)
    return None

__all__ = ["managed_language_profile"]
