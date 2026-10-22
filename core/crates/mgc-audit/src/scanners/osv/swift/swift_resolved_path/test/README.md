# Swift resolved-path tests

## Purpose

Tests the bounded Package.resolved search scope owned by the sibling locator implementation.

## Phạm vi

Kiểm thử phạm vi tìm Package.resolved có giới hạn do implementation locator cùng owner quản lý.

This test covers root, iOS, one parent level, and refusal to walk farther. It does not change scanner callers or the locator implementation.
