# Multi-platform project context

## Responsibility
Computes the project slug and Kotlin package once, and carries them with the target/name to all platform output owners.

## Phạm vi
Tính slug và package Kotlin một lần, rồi truyền cùng target/name tới các owner output.

## Boundary
This type contains only data for one scaffold invocation. It owns no I/O and no global state.

## Ranh giới
Kiểu này chỉ giữ dữ liệu của một lần scaffold. Nó không làm I/O và không chứa state toàn cục.
