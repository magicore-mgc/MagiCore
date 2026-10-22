# Web runtime owner

## Purpose

This owner preserves Deno/Bun/Node.js marker precedence. Shared Node lockfile selection lives in `package_manager`; Cloud detection has its own owner.

## Phạm vi

Owner này giữ nguyên thứ tự nhận diện Deno/Bun/Node.js. Việc chọn lockfile Node dùng owner `package_manager`; Cloud có owner riêng.
