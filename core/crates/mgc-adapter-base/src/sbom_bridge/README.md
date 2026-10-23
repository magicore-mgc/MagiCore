# Shared SBOM bridge

## Purpose

Owns the single `generate_sbom` implementation shared by every adapter: build an `SbomGenerator` from options, serialize the lockfile to JSON and map failures onto the adapter error surface. Adapters re-export this instead of duplicating the body.

## Vietnamese

Sở hữu implementation `generate_sbom` duy nhất dùng chung cho mọi adapter: dựng `SbomGenerator` từ options, serialize lockfile sang JSON và map lỗi vào bề mặt lỗi adapter. Adapter re-export thay vì lặp lại thân hàm.
