"""Resolve the Dart executable bundled with the selected Flutter SDK.
Tìm executable Dart nằm trong Flutter SDK đã chọn.
"""

import os
from pathlib import Path
import shutil


def main() -> int:
    flutter = shutil.which("flutter")
    if not flutter:
        raise SystemExit("Flutter SDK is missing")

    executable = "dart.exe" if os.name == "nt" else "dart"
    dart = Path(flutter).resolve().parent / "cache" / "dart-sdk" / "bin" / executable
    if not dart.is_file():
        raise SystemExit("Real Dart SDK executable is missing")

    print(dart)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
