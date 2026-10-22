"""ConstArrayExclusion tests, kept adjacent to the scanner contract."""

from .support import *

class ConstArrayExclusion(unittest.TestCase):
    """Top-level deny-list arrays name tools without spawning them."""

    def test_top_level_deny_list_literals_are_not_findings(self):
        findings = scan_snippet(
            "core/crates/mgc-config/src/hooks.rs",
            "const DENY: &[&str] = &[\n"
            '    "npm", "npx",\n'
            '    "cargo",\n'
            "];\n"
            "\n"
            "pub fn run_hooks() -> Result<()> {\n"
            "    Ok(())\n"
            "}\n",
        )
        self.assertEqual(findings, [])

    def test_sql_queries_are_not_misclassified_as_process_command_tables(self):
        findings = scan_snippet(
            "core/crates/mgc-store/src/database.rs",
            'fn inspect_db(conn: &Connection) {\n'
            '    conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0));\n'
            '    conn.query_row("SELECT COUNT(*) FROM package_files", [], |row| row.get(0));\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_public_const_tool_inventory_is_not_a_spawn(self):
        findings = scan_snippet(
            "cli/src/commands/dep_gate.rs",
            "pub const SPLIT_LANGUAGES: &[&str] = &[\n"
            '    "python",\n'
            '    "go",\n'
            "];\n",
        )
        self.assertEqual(findings, [])

    def test_static_core_marker_table_is_not_a_process_descriptor(self):
        findings = scan_snippet(
            "core/crates/mgc-config/src/project.rs",
            "const NODE_CORE_MARKERS: &[(&[&str], &str)] = &[\n"
            '    (&["react-native", "expo"], "app"),\n'
            '    (&["next", "react"], "web"),\n'
            "];\n",
        )
        self.assertEqual(findings, [])

    def test_spawn_next_to_deny_list_still_flagged(self):
        findings = scan_snippet(
            "adapters/lib/src/install/mod.rs",
            "const DENY: &[&str] = &[\n"
            '    "npm",\n'
            "];\n"
            "\n"
            "pub async fn run_install() -> MgResult<()> {\n"
            '    let _ = mgc_exec::run::run("cargo", &args, &opts)?;\n'
            "    Ok(())\n"
            "}\n",
        )
        self.assertTrue(
            any(f["tool"] == "cargo" for f in findings),
            "call-shaped spawns stay measured beside deny lists",
        )

__all__ = ['ConstArrayExclusion']
