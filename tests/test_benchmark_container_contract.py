"""Contract tests for the Docker benchmark's MagiCore binary wiring.

Kiểm tra benchmark Docker truyền đúng binary MagiCore vào runner.
"""

from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]


class BenchmarkContainerContractTests(unittest.TestCase):
    def test_mgc_runner_uses_the_read_only_binary_mounted_into_container(self):
        compose = (ROOT / "benchmark" / "docker-compose.yml").read_text(encoding="utf-8")
        runner = (ROOT / "benchmark" / "scripts" / "run_benchmark.sh").read_text(
            encoding="utf-8"
        )

        self.assertIn("../target/release/mgc:/benchmark/mgc:ro", compose)
        self.assertIn("- MGC_BIN=/benchmark/mgc", compose)
        self.assertIn('mgc) [ -x "$MGC_BIN" ]', runner)
        self.assertIn('mgc) "$MGC_BIN" install ;;', runner)


if __name__ == "__main__":
    unittest.main()
