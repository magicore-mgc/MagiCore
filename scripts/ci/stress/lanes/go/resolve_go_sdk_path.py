"""Resolve the Go executable selected for the CI lane.
Tìm executable Go đã được chọn cho lane CI.
"""

from pathlib import Path
import shutil


def main() -> int:
    executable = shutil.which("go")
    if not executable or not Path(executable).is_file():
        raise SystemExit("Real Go SDK executable is missing")

    print(Path(executable).resolve())
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
