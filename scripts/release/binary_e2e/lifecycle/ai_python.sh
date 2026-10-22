#!/usr/bin/env bash
# Exercise the installed binary through the Python AI lifecycle.
# Chạy binary đã cài qua vòng đời AI/Python.
set -euo pipefail
if [[ "$RUNNER_OS" == "Windows" ]]; then MGCBIN="$HOME/.magicore-test/bin/mgc.exe"; else MGCBIN="$HOME/.magicore-test/bin/mgc"; fi
WORK="$(mktemp -d)"
cd "$WORK"
"$MGCBIN" create-ai python-agent test-ai
cd test-ai
# These pure-Python tools are resolved and installed into the MGC
# store; neither test nor build may use globally pip-installed tools.
# Compiled wheels/sdists remain fail-closed in this current lane.
"$MGCBIN" add-ai six@1.17.0
"$MGCBIN" add-ai pytest@9.1.1
"$MGCBIN" add-ai build@1.6.1
"$MGCBIN" add-ai setuptools@84.0.0
"$MGCBIN" install
test -f mgc.lock
test ! -f uv.lock
mkdir -p tests
cat > tests/test_mgc_release_e2e.py <<'PYTEST'
def test_generated_python_runtime_is_usable():
    import sys
    assert sys.version_info >= (3, 12)
PYTEST
"$MGCBIN" test
"$MGCBIN" build
export MGC_AUDIT_STRICT=1
set +e
"$MGCBIN" audit
rc=$?
set -e
if [[ "$rc" -ne 0 && "$rc" -ne 1 ]]; then
  echo "ai audit exited $rc (0/1 allowed, 2=unverified is a FAIL)" >&2
  exit 1
fi
unset MGC_AUDIT_STRICT
