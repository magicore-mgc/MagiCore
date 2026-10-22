"""LaneGateCoverage tests, kept adjacent to the scanner contract."""

from .support import *

class LaneGateCoverage(unittest.TestCase):
    """Every dependency lane passes through dep_gate:: or GATE-EXEMPT."""

    def setUp(self):
        self.repo_root = REPO_ROOT

    def test_lane_with_gate_call_passes(self):
        with tempfile.TemporaryDirectory() as tmp:
            lane_dir = os.path.join(tmp, "cli", "src", "commands", "core", "install")
            os.makedirs(lane_dir)
            with open(os.path.join(lane_dir, "ai.rs"), "w", encoding="utf-8") as handle:
                handle.write(
                    "pub async fn install() -> Result<()> {\n"
                    "    crate::commands::dep_gate::gate(\n"
                    "        \"ai\",\n"
                    "        crate::commands::dep_gate::DepOp::Install,\n"
                    "        Some(tool),\n"
                    "        &compat,\n"
                    "        None,\n"
                    "    )?;\n"
                    "    Ok(())\n"
                    "}\n"
                )
            roots = gate.LANE_GATE_ROOTS
            gate.LANE_GATE_ROOTS = [
                os.path.relpath(
                    os.path.join(tmp, "cli", "src", "commands", "core", "install"), tmp
                )
            ]
            try:
                checked, missing = gate.check_lane_gate(tmp)
            finally:
                gate.LANE_GATE_ROOTS = roots
        self.assertEqual(missing, [])
        self.assertEqual(checked, 1)

    def test_lane_without_gate_or_exempt_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            lane_dir = os.path.join(tmp, "cli", "src", "commands", "core", "add")
            os.makedirs(lane_dir)
            with open(os.path.join(lane_dir, "ai.rs"), "w", encoding="utf-8") as handle:
                handle.write(
                    "pub async fn add() -> Result<()> {\n"
                    "    shared::ai_run_tool(&root, tool, &args)?;\n"
                    "    Ok(())\n"
                    "}\n"
                )
            roots = gate.LANE_GATE_ROOTS
            gate.LANE_GATE_ROOTS = [
                os.path.relpath(
                    os.path.join(tmp, "cli", "src", "commands", "core", "add"), tmp
                )
            ]
            try:
                checked, missing = gate.check_lane_gate(tmp)
            finally:
                gate.LANE_GATE_ROOTS = roots
        self.assertEqual(len(missing), 1)
        self.assertTrue(missing[0].endswith("ai.rs"))

    def test_pure_router_without_functions_is_exempt(self):
        with tempfile.TemporaryDirectory() as tmp:
            lane_dir = os.path.join(tmp, "cli", "src", "commands", "core", "list")
            os.makedirs(lane_dir)
            with open(os.path.join(lane_dir, "mod.rs"), "w", encoding="utf-8") as handle:
                handle.write("pub mod ai;\npub mod web;\n")
            roots = gate.LANE_GATE_ROOTS
            gate.LANE_GATE_ROOTS = [
                os.path.relpath(
                    os.path.join(tmp, "cli", "src", "commands", "core", "list"), tmp
                )
            ]
            try:
                checked, missing = gate.check_lane_gate(tmp)
            finally:
                gate.LANE_GATE_ROOTS = roots
        self.assertEqual(missing, [])

    def test_lane_gate_fails_closed_when_source_cannot_be_read(self):
        with tempfile.TemporaryDirectory() as tmp:
            lane_dir = os.path.join(tmp, "cli", "src", "commands", "core", "install")
            os.makedirs(lane_dir)
            source_path = os.path.join(lane_dir, "ai.rs")
            with open(source_path, "w", encoding="utf-8") as handle:
                handle.write("pub async fn install() -> Result<()> { Ok(()) }\n")

            roots = gate.LANE_GATE_ROOTS
            gate.LANE_GATE_ROOTS = [os.path.relpath(lane_dir, tmp)]
            real_open = open

            def deny_source_read(path, *args, **kwargs):
                if os.fspath(path) == source_path:
                    raise PermissionError("fixture unreadable source")
                return real_open(path, *args, **kwargs)

            try:
                with mock.patch("builtins.open", side_effect=deny_source_read):
                    checked, missing = gate.check_lane_gate(tmp)
            finally:
                gate.LANE_GATE_ROOTS = roots

        self.assertEqual(checked, 1)
        self.assertEqual(
            missing,
            [f"<scan-error:unreadable> {os.path.relpath(source_path, tmp)}"],
        )

    def test_lane_gate_fails_closed_when_root_is_missing(self):
        with tempfile.TemporaryDirectory() as tmp:
            roots = gate.LANE_GATE_ROOTS
            gate.LANE_GATE_ROOTS = ["cli/src/commands/core/install"]
            try:
                checked, missing = gate.check_lane_gate(tmp)
            finally:
                gate.LANE_GATE_ROOTS = roots

        self.assertEqual(checked, 1)
        self.assertEqual(
            missing,
            ["<scan-error:missing-root> cli/src/commands/core/install"],
        )

    def test_lane_gate_fails_closed_on_directory_walk_error(self):
        with tempfile.TemporaryDirectory() as tmp:
            lane_dir = os.path.join(tmp, "cli", "src", "commands", "core", "install")
            os.makedirs(lane_dir)
            roots = gate.LANE_GATE_ROOTS
            gate.LANE_GATE_ROOTS = [os.path.relpath(lane_dir, tmp)]

            def failed_walk(_path, onerror=None):
                error = PermissionError("fixture unreadable directory")
                error.filename = os.path.join(lane_dir, "blocked")
                onerror(error)
                return iter(())

            try:
                with mock.patch.object(gate.os, "walk", side_effect=failed_walk):
                    checked, missing = gate.check_lane_gate(tmp)
            finally:
                gate.LANE_GATE_ROOTS = roots

        self.assertEqual(checked, 1)
        self.assertEqual(
            missing,
            [
                "<scan-error:walk> "
                + os.path.join("cli", "src", "commands", "core", "install", "blocked")
            ],
        )

    def test_real_tree_has_no_missing_lane(self):
        checked, missing = gate.check_lane_gate(self.repo_root)
        self.assertEqual(missing, [], f"lanes without gate: {missing}")
        self.assertGreater(checked, 0)

__all__ = ['LaneGateCoverage']
