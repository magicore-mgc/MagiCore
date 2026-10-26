#!/usr/bin/env bash
# Measure line coverage for the whole workspace.
# Đo độ phủ dòng cho toàn workspace.
#
# Coverage answers a question the test count cannot: 4,798 tests may all pass
# while a large part of the code is never executed. Without the number, "well
# tested" is a belief rather than a measurement.
# Độ phủ trả lời câu hỏi mà số lượng test không trả lời được: 4.798 test có thể
# đều qua trong khi phần lớn code chưa từng chạy. Không có con số, "được test
# tốt" là niềm tin chứ không phải phép đo.
#
# Usage / Dùng:
#   scripts/test/coverage.sh            # summary only
#   scripts/test/coverage.sh --html     # also write target/llvm-cov/html
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if ! command -v cargo-llvm-cov >/dev/null 2>&1 && ! cargo llvm-cov --version >/dev/null 2>&1; then
  echo "cargo-llvm-cov is required: cargo install cargo-llvm-cov --locked" >&2
  exit 1
fi

# The active rustc may come from Homebrew while llvm-tools live in rustup, so
# point at the rustup toolchain explicitly when its binaries are not on PATH.
# rustc đang dùng có thể từ Homebrew trong khi llvm-tools nằm ở rustup, nên chỉ
# rõ toolchain rustup khi binary của nó không có trên PATH.
if ! command -v llvm-cov >/dev/null 2>&1; then
  SYSROOT="$(rustup run stable rustc --print sysroot 2>/dev/null || true)"
  if [[ -n "$SYSROOT" && -d "$SYSROOT/lib/rustlib" ]]; then
    TOOLS="$(find "$SYSROOT/lib/rustlib" -maxdepth 3 -type d -name bin -print -quit)"
    if [[ -n "$TOOLS" && -x "$TOOLS/llvm-cov" ]]; then
      export LLVM_COV="$TOOLS/llvm-cov"
      export LLVM_PROFDATA="$TOOLS/llvm-profdata"
    fi
  fi
fi

if [[ "${1:-}" == "--html" ]]; then
  rustup run stable cargo llvm-cov --workspace --lib --html 2>/dev/null ||
    cargo llvm-cov --workspace --lib --html
  echo "HTML report: target/llvm-cov/html/index.html"
else
  rustup run stable cargo llvm-cov --workspace --lib --summary-only 2>/dev/null ||
    cargo llvm-cov --workspace --lib --summary-only
fi
