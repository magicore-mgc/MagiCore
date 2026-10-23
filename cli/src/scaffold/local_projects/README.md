# Local project discovery

## Purpose

Owns offline scaffold discovery. Projects are identified by `mgc.toml` plus a framework marker (`manage.py`, `artisan`, `pubspec.yaml`, …), never by folder name. Copying excludes `node_modules`, virtualenvs, `target`, `dist`, `build`, `.git`, `.magicore` and `.env`; symlinked roots and out-of-project link targets are refused.

## Vietnamese

Sở hữu việc dò project để scaffold offline. Project được nhận diện bằng `mgc.toml` cùng marker framework (`manage.py`, `artisan`, `pubspec.yaml`, …), không bao giờ bằng tên thư mục. Khi copy loại `node_modules`, virtualenv, `target`, `dist`, `build`, `.git`, `.magicore` và `.env`; từ chối project root là symlink và đích link trỏ ra ngoài project.
