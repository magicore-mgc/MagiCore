# Project detection test owner

Owns tests for framework marker parsing, adapter selection, and refusal to follow external marker symlinks. Unix-only cases remain gated with cfg(unix) at both wrapper and implementation.

Sở hữu test đọc marker framework, chọn adapter và từ chối symlink marker trỏ ra ngoài. Case Unix giữ cfg(unix) ở wrapper lẫn implementation.

Source: project_detection.rs. Shared fixture: ../support.
