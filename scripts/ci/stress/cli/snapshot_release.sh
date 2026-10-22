python - <<'PYTHON'
import os
import shutil
from pathlib import Path

source = Path("target/release/mgc.exe" if os.name == "nt" else "target/release/mgc")
destination = Path(os.environ["RUNNER_TEMP"]) / "mgc-stress-binary" / source.name
destination.parent.mkdir(parents=True, exist_ok=True)
shutil.copy2(source, destination)
with open(os.environ["GITHUB_ENV"], "a", encoding="utf-8") as environment:
    environment.write(f"MGC_STRESS_CLI_SNAPSHOT={destination}\n")
PYTHON
