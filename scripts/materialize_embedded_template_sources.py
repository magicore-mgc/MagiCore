"""Materialize compiled template layers for registry integration tests. — Giải nén layer sản phẩm để kiểm thử registry."""

from __future__ import annotations

import argparse
import re
import tarfile
from pathlib import Path, PurePosixPath, PureWindowsPath

MAX_ARCHIVE_BYTES = 16 * 1024 * 1024
MAX_EXPANDED_BYTES = 64 * 1024 * 1024
MAX_ARCHIVE_MEMBERS = 4096
EMBEDDED_REFERENCE = re.compile(
    r'include_bytes!\s*\(\s*"../../embedded/([^\"]+\.tar\.gz)"\s*\)'
)
EMBEDDED_LAYER = re.compile(
    r'map\.insert\(\s*"(?P<layer>[^"]+)"\.to_string\(\)\s*,\s*'
    r'EmbeddedLayer\s*\{(?P<body>.*?)\n\s*\},\s*\);',
    re.DOTALL,
)


def materialize(embedded_dir: Path, manifest: Path, output: Path) -> int:
    source = manifest.read_text(encoding="utf-8")
    references = EMBEDDED_REFERENCE.findall(source)
    layers = []
    for match in EMBEDDED_LAYER.finditer(source):
        archive_match = EMBEDDED_REFERENCE.search(match.group("body"))
        if archive_match is None:
            continue
        layer_name = match.group("layer")
        layer = PurePosixPath(layer_name)
        windows_layer = PureWindowsPath(layer_name)
        if (
            layer.is_absolute()
            or windows_layer.is_absolute()
            or windows_layer.drive
            or ".." in layer.parts
            or "\\" in layer_name
            or layer.as_posix() != layer_name
        ):
            raise ValueError(f"unsafe embedded template layer key: {layer}")
        if not layer.parts:
            raise ValueError("empty embedded template layer key")
        archive_name = archive_match.group(1)
        archive_relative = PurePosixPath(archive_name)
        windows_archive = PureWindowsPath(archive_name)
        if (
            archive_relative.is_absolute()
            or windows_archive.is_absolute()
            or windows_archive.drive
            or len(archive_relative.parts) != 1
            or "\\" in archive_name
            or archive_relative.parts[0] != archive_name
        ):
            raise ValueError(f"unsafe embedded template archive path: {archive_name}")
        layers.append((layer, archive_name))
    if not layers:
        raise ValueError(f"no compiled template archives found in {manifest}")
    if sorted(references) != sorted(archive_name for _, archive_name in layers):
        raise ValueError(f"embedded template references do not match layer entries in {manifest}")

    output.mkdir(parents=True, exist_ok=True)
    output = output.resolve(strict=True)
    count = 0
    for layer, archive_name in sorted(layers, key=lambda item: (item[0].as_posix(), item[1])):
        archive_path = embedded_dir / archive_name
        if archive_path.is_symlink() or not archive_path.is_file():
            raise ValueError(f"compiled template archive is missing or linked: {archive_path}")
        if archive_path.stat().st_size > MAX_ARCHIVE_BYTES:
            raise ValueError(f"compiled template archive exceeds size limit: {archive_path}")

        target = output.joinpath(*layer.parts)
        if not target.resolve(strict=False).is_relative_to(output):
            raise ValueError(f"unsafe embedded template layer key: {layer}")
        if target.exists() or target.is_symlink():
            raise ValueError(f"template output already exists: {target}")
        target.mkdir(parents=True)

        expanded = 0
        with tarfile.open(archive_path, mode="r:gz") as archive:
            members = archive.getmembers()
            if len(members) > MAX_ARCHIVE_MEMBERS:
                raise ValueError(f"compiled template archive has too many entries: {archive_path}")
            for member in members:
                relative = PurePosixPath(member.name)
                windows_relative = PureWindowsPath(member.name)
                if (
                    relative.is_absolute()
                    or windows_relative.is_absolute()
                    or windows_relative.drive
                    or ".." in relative.parts
                    or "\\" in member.name
                ):
                    raise ValueError(f"unsafe path in compiled template archive: {member.name}")
                parts = tuple(part for part in relative.parts if part not in ("", "."))
                if not parts:
                    if member.isdir():
                        continue
                    raise ValueError(f"invalid root entry in compiled template archive: {member.name}")
                destination = target.joinpath(*parts)
                if not destination.resolve(strict=False).is_relative_to(target):
                    raise ValueError(f"unsafe output path in compiled template archive: {member.name}")
                if member.isdir():
                    destination.mkdir(parents=True, exist_ok=True)
                    continue
                if not member.isfile() or member.size < 0:
                    raise ValueError(f"unsupported entry in compiled template archive: {member.name}")
                expanded += member.size
                if expanded > MAX_EXPANDED_BYTES:
                    raise ValueError(f"compiled template archive expands beyond size limit: {archive_path}")
                destination.parent.mkdir(parents=True, exist_ok=True)
                contents = archive.extractfile(member)
                if contents is None:
                    raise ValueError(f"cannot read compiled template archive entry: {member.name}")
                with contents, destination.open("xb") as destination_file:
                    remaining = member.size
                    while remaining:
                        chunk = contents.read(min(1024 * 1024, remaining))
                        if not chunk:
                            raise ValueError(f"truncated compiled template archive entry: {member.name}")
                        destination_file.write(chunk)
                        remaining -= len(chunk)
                destination.chmod(member.mode & 0o777)

        if not (target / "template.toml").is_file() or not (target / "sources").is_dir():
            raise ValueError(f"compiled template layer is incomplete: {target}")
        count += 1
    return count


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--embedded-dir", type=Path, required=True)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    count = materialize(args.embedded_dir, args.manifest, args.output)
    print(f"Materialized {count} compiled template layers")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, tarfile.TarError, ValueError) as error:
        raise SystemExit(f"ERROR: {error}") from error
