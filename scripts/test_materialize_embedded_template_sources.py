"""Security tests for the compiled-template materializer. — Kiểm thử an toàn bộ giải nén template."""

from __future__ import annotations

import io
import tarfile
import tempfile
import unittest
from pathlib import Path

from materialize_embedded_template_sources import materialize


def _archive(path: str, kind: bytes = tarfile.REGTYPE) -> bytes:
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:gz") as archive:
        member = tarfile.TarInfo(path)
        member.type = kind
        member.linkname = "../../outside.txt" if kind == tarfile.SYMTYPE else ""
        member.size = 1 if kind == tarfile.REGTYPE else 0
        archive.addfile(member, io.BytesIO(b"x") if member.size else None)
    return buffer.getvalue()


def _manifest(layer: str = "web/probe", archive: str = "probe.tar.gz") -> str:
    return f'''map.insert(
            "{layer}".to_string(),
            EmbeddedLayer {{
                data: include_bytes!("../../embedded/{archive}"),
            }},
        );
'''


class MaterializeEmbeddedTemplateSourcesTests(unittest.TestCase):
    def _inputs(self, root: Path, archive: bytes, manifest: str) -> tuple[Path, Path, Path]:
        embedded = root / "embedded"
        embedded.mkdir()
        archive_path = embedded / "probe.tar.gz"
        archive_path.write_bytes(archive)
        manifest_path = root / "embedded.rs"
        manifest_path.write_text(manifest, encoding="utf-8")
        return embedded, manifest_path, root / "output"

    def test_rejects_archive_member_path_traversal_without_writing_outside_output(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = self._inputs(root, _archive("../outside.txt"), _manifest())
            with self.assertRaisesRegex(ValueError, "unsafe path"):
                materialize(*args)
            self.assertFalse((root / "outside.txt").exists())

    def test_rejects_windows_separator_path_traversal(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = self._inputs(root, _archive(r"sources\..\outside.txt"), _manifest())
            with self.assertRaisesRegex(ValueError, "unsafe path"):
                materialize(*args)

    def test_rejects_archive_symlink_entries(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = self._inputs(root, _archive("sources/link", tarfile.SYMTYPE), _manifest())
            with self.assertRaisesRegex(ValueError, "unsupported entry"):
                materialize(*args)

    def test_rejects_archive_reference_outside_embedded_directory(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = _manifest(archive="../../outside.tar.gz")
            embedded = root / "embedded"
            embedded.mkdir()
            manifest_path = root / "embedded.rs"
            manifest_path.write_text(manifest, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "unsafe embedded template archive path"):
                materialize(embedded, manifest_path, root / "output")

    def test_rejects_layer_key_that_is_not_a_canonical_relative_path(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = self._inputs(
                root,
                _archive("template.toml"),
                _manifest("web/../outside"),
            )
            with self.assertRaisesRegex(ValueError, "unsafe embedded template layer key"):
                materialize(*args)


if __name__ == "__main__":
    unittest.main()
