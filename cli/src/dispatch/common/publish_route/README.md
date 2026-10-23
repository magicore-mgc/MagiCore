# Publish route

## Purpose

Owns only the `CommonCommand::Publish` arm: it forwards every field into `PublishArgs` and calls `commands::publish::run` with the current `core`/`recursive` context. No validation, defaults or sequencing live here.

## Phạm vi

Chỉ sở hữu nhánh `CommonCommand::Publish`: chuyển từng field sang `PublishArgs` rồi gọi `commands::publish::run` với `core`/`recursive` hiện tại. Không chứa validation, default hay thứ tự xử lý.
