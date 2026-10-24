# Local project reuse

## Purpose

Owns reusing a project already on the user's machine as the offline scaffold source. Only framework source is copied; `mgc.toml`, lockfiles, `.magicore/` and `.mgc.core` are MagiCore-owned and are removed so the caller writes a fresh identity. Generated trees and `.env` are never copied.

## Vietnamese

Sở hữu việc tái dùng project đã có trên máy user làm nguồn scaffold offline. Chỉ copy source framework; `mgc.toml`, lockfile, `.magicore/` và `.mgc.core` thuộc MagiCore nên bị xoá để caller ghi identity mới. Không bao giờ copy cây sinh và `.env`.
