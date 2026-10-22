"""Focused ProvenanceMatrixCleanliness test owner.

Owner test hẹp cho ProvenanceMatrixCleanliness.
"""

from .support import (
    Path,
    json,
    patch,
    tempfile,
    unittest,
    _matrix_sha_for_head,
)

class ProvenanceMatrixCleanliness(unittest.TestCase):
    def test_dirty_or_legacy_matrix_cannot_bind_to_current_head(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            matrix_path = Path(temp_dir) / "matrix.json"
            base = {"commit": "a" * 40, "working_tree_clean": True}
            for matrix, clean in (
                (base, False),
                ({"commit": "a" * 40}, True),
            ):
                with self.subTest(matrix=matrix, clean=clean):
                    matrix_path.write_text(json.dumps(matrix), encoding="utf-8")
                    with (
                        patch("provenance_chain.MATRIX_PATH", matrix_path),
                        patch("provenance_chain._working_tree_clean", return_value=clean),
                    ):
                        self.assertEqual(_matrix_sha_for_head("a" * 40), (None, False))
