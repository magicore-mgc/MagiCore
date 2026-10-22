"""MarkerDeclaration tests, kept adjacent to the scanner contract."""

from .support import *

class MarkerDeclaration(unittest.TestCase):
    """A DELEGATED marker documents debt but never waives the strict gate."""

    def test_marker_in_doc_block_declares_delegation(self):
        findings = scan_snippet(
            "cli/src/commands/model/mod.rs",
            '/// Quantize models.\n'
            '///\n'
            '/// DELEGATED: runs inside the external python package.\n'
            'fn quantize(path: &str) -> Result<()> {\n'
            '    let status = std::process::Command::new("python3")\n'
            '        .status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(
            findings[0]["status"],
            "violation",
            "documenting a dependency-manager spawn must not make it pass",
        )
        self.assertTrue(findings[0]["declared_delegation"])

__all__ = ['MarkerDeclaration']
