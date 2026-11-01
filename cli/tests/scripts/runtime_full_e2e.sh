#!/usr/bin/env bash
# NATIVE-RUNTIME BOUNDARY E2E
# Tests that compatibility flags do not create a delegated runtime lane:
#   - `mgc dev` native REFUSES a bun/deno script
#   - `mgc dev --compat-runtime bun|deno` also refuses without spawning
# Bun/Deno are fixture names only — neither is an MGC execution engine.

set -euo pipefail

echo "=== Compat-Runtime E2E (bun + deno) ==="

# Find mgc binary — honor the exact binary supplied by CI/callers.
PROJECT_ROOT="${PROJECT_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)}"

if [ -n "${MGC_BIN:-}" ]; then
    case "$MGC_BIN" in
        /*) ;;
        *) MGC_BIN="$PROJECT_ROOT/$MGC_BIN" ;;
    esac
    if [ ! -f "$MGC_BIN" ] && [ -f "$MGC_BIN.exe" ]; then
        MGC_BIN="$MGC_BIN.exe"
    fi
elif [ -f "$PROJECT_ROOT/target/release/mgc" ]; then
    MGC_BIN="$PROJECT_ROOT/target/release/mgc"
elif [ -f "$PROJECT_ROOT/target/release/mgc.exe" ]; then
    MGC_BIN="$PROJECT_ROOT/target/release/mgc.exe"
elif [ -f "$PROJECT_ROOT/target/debug/mgc" ]; then
    MGC_BIN="$PROJECT_ROOT/target/debug/mgc"
elif [ -f "$PROJECT_ROOT/target/debug/mgc.exe" ]; then
    MGC_BIN="$PROJECT_ROOT/target/debug/mgc.exe"
else
    echo "✗ FAIL: mgc binary not found (need cargo build first)"
    exit 1
fi

if [ ! -x "$MGC_BIN" ]; then
    echo "✗ FAIL: supplied mgc binary is not executable: $MGC_BIN"
    exit 1
fi

echo "Using mgc: $MGC_BIN"
echo

# Isolated temp workspace
# Preserve the installed Rust toolchain roots before redirecting HOME for the
# isolated MagiCore state. rustup stores its default toolchain under RUSTUP_HOME.
# Giữ root toolchain Rust trước khi đổi HOME để cô lập state MagiCore. rustup
# lưu toolchain mặc định trong RUSTUP_HOME.
ORIGINAL_HOME="$HOME"
export RUSTUP_HOME="${RUSTUP_HOME:-${ORIGINAL_HOME}/.rustup}"
export CARGO_HOME="${CARGO_HOME:-${ORIGINAL_HOME}/.cargo}"
TEMP_BASE=$(mktemp -d)
trap 'rm -rf "$TEMP_BASE"' EXIT
export HOME="$TEMP_BASE/home"
export USERPROFILE="$HOME"
export MGC_DATA_DIR="$TEMP_BASE/mgc-data"
unset MGC_COMPAT_RUNTIME
mkdir -p "$HOME" "$MGC_DATA_DIR"
cd "$TEMP_BASE"

echo "--- Test 0: missing-project diagnostic matches real discovery markers ---"
mkdir -p proj-unmarked
cd proj-unmarked
NO_PROJECT_LOG="$TEMP_BASE/no-project.log"
NO_PROJECT_EXIT=0
"$MGC_BIN" list --core web >"$NO_PROJECT_LOG" 2>&1 || NO_PROJECT_EXIT=$?
if [ "$NO_PROJECT_EXIT" -eq 0 ]; then
    echo "✗ FAIL: mgc list accepted a directory without a project marker"
    exit 1
fi
if ! grep -q "mgc.toml, mgc.lock, package.json, Cargo.toml, pyproject.toml, or .git" "$NO_PROJECT_LOG"; then
    echo "✗ FAIL: project discovery diagnostic does not list the implemented markers:"
    cat "$NO_PROJECT_LOG"
    exit 1
fi
if grep -q ".magicore/project.toml" "$NO_PROJECT_LOG"; then
    echo "✗ FAIL: project discovery diagnostic advertises an unsupported .magicore/project.toml marker"
    exit 1
fi
cd ..
echo "✓ missing-project error names the markers the product actually detects"

run_compat_dev_lane() {
    local RUNTIME="$1" SCRIPT_NAME="$2"

    mkdir -p "proj-$RUNTIME"
    cd "proj-$RUNTIME"

    cat > package.json <<EOF
{
  "name": "compat-$RUNTIME",
  "version": "1.0.0",
  "scripts": {
    "dev": "$SCRIPT_NAME"
  },
  "dependencies": {}
}
EOF

    cat > mgc.toml <<EOF
name = "compat-$RUNTIME"
version = "1.0.0"
ecosystem = "web"
EOF

    local INSTALL_LOG="install.log"
    if ! "$MGC_BIN" install >"$INSTALL_LOG" 2>&1; then
        echo "✗ FAIL: native install did not prepare the real dev precondition:"
        cat "$INSTALL_LOG"
        cd ..
        return 1
    fi

    # 1. NATIVE lane: must REFUSE the rival-runtime script.
    local NATIVE_EXIT=0
    local NATIVE_LOG="native-dev.log"
    "$MGC_BIN" dev >"$NATIVE_LOG" 2>&1 || NATIVE_EXIT=$?
    if [ "$NATIVE_EXIT" -eq 0 ]; then
        echo "✗ FAIL: native mgc dev accepted a $RUNTIME script (must refuse)"
        cat "$NATIVE_LOG"
        cd ..
        return 1
    fi
    if ! grep -q "native MagiCore runtime" "$NATIVE_LOG"; then
        echo "✗ FAIL: native mgc dev failed before its runtime policy gate:"
        cat "$NATIVE_LOG"
        cd ..
        return 1
    fi
    echo "✓ native 'mgc dev' refuses the $RUNTIME script"

    # 2. Historical compat lane: must fail before spawning the runtime.
    local COMPAT_LOG="compat-dev.log"
    "$MGC_BIN" dev --compat-runtime "$RUNTIME" >"$COMPAT_LOG" 2>&1 || true
    if grep -q "native MagiCore runtime" "$COMPAT_LOG"; then
        echo "✓ --compat-runtime $RUNTIME refused by native-only policy"
    else
        echo "✗ FAIL: compatibility flag must be refused clearly, got:"
        cat "$COMPAT_LOG"
        cd ..
        return 1
    fi

    cd ..
    return 0
}

echo "--- Test 1: Bun compat lane ---"
run_compat_dev_lane "bun" "bun run server.ts" || exit 1

echo "--- Test 2: Deno compat lane ---"
run_compat_dev_lane "deno" "deno run server.ts" || exit 1

echo "--- Test 3: real native dev execution and project audit path ---"
mkdir -p proj-native
cd proj-native
cat > package.json <<'EOF'
{
  "name": "native-dev-audit-e2e",
  "version": "1.0.0",
  "scripts": { "dev": "node server.js" },
  "dependencies": {}
}
EOF
cat > mgc.toml <<'EOF'
name = "native-dev-audit-e2e"
version = "1.0.0"
ecosystem = "web"
EOF
cat > server.js <<'EOF'
console.log("MGC_NATIVE_DEV_E2E_OK");
EOF

INSTALL_LOG="$TEMP_BASE/native-install.log"
if ! "$MGC_BIN" install >"$INSTALL_LOG" 2>&1; then
    echo "✗ FAIL: native install did not prepare the Node dev project:"
    cat "$INSTALL_LOG"
    exit 1
fi

NATIVE_LOG="$TEMP_BASE/native-dev.log"
if ! "$MGC_BIN" dev >"$NATIVE_LOG" 2>&1; then
    echo "✗ FAIL: native Node dev command failed:"
    cat "$NATIVE_LOG"
    exit 1
fi
if ! grep -q "MGC_NATIVE_DEV_E2E_OK" "$NATIVE_LOG"; then
    echo "✗ FAIL: mgc dev did not execute the Node project script:"
    cat "$NATIVE_LOG"
    exit 1
fi

AUDIT_LOG="$PWD/.magicore/exec.log"
if [ ! -f "$AUDIT_LOG" ]; then
    echo "✗ FAIL: real mgc dev execution did not create .magicore/exec.log"
    exit 1
fi
if [ -e "$PWD/.mgc/exec.log" ]; then
    echo "✗ FAIL: real mgc dev execution wrote the legacy .mgc/exec.log path"
    exit 1
fi
node - "$AUDIT_LOG" <<'NODE'
const fs = require("node:fs");
const path = require("node:path");
const [auditPath] = process.argv.slice(2);
const expectedCwd = process.cwd();
const rows = fs.readFileSync(auditPath, "utf8").trim().split(/\r?\n/).map((line) => JSON.parse(line));
const normalizeCommand = (command) => path.basename(String(command).replaceAll("\\", "/"))
  .toLowerCase().replace(/\.(exe|cmd|bat)$/, "");
const samePath = (actual, expected) => {
  try {
    const actualPath = fs.realpathSync(actual);
    const expectedPath = fs.realpathSync(expected);
    return process.platform === "win32"
      ? actualPath.toLowerCase() === expectedPath.toLowerCase()
      : actualPath === expectedPath;
  } catch {
    return false;
  }
};
const found = rows.some((row) => normalizeCommand(row.cmd) === "node"
  && Array.isArray(row.args) && row.args.includes("server.js")
  && samePath(row.cwd, expectedCwd)
  && row.exit_code === 0);
if (!found) {
  console.error("Audit log did not record the successful native Node dev execution");
  console.error(`Expected project cwd: ${expectedCwd}`);
  const observed = rows.map(({ cmd, args, cwd, exit_code }) => ({ cmd, args, cwd, exit_code }));
  console.error(JSON.stringify(observed, null, 2));
  process.exit(1);
}
NODE
echo "✓ real Node dev execution is recorded in .magicore/exec.log"

echo "--- Test 4: real mgc dev tolerates short-lived descendants under process churn ---"
case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
        echo "SKIP: process-table churn regression exercises the Unix process scanner; Windows uses Job Object tests"
        ;;
    *)
        CHURN_PROJECT="$TEMP_BASE/proj-native-process-churn"
        mkdir -p "$CHURN_PROJECT"
        cd "$CHURN_PROJECT"
        cat > package.json <<'EOF'
{
  "name": "native-process-churn-e2e",
  "version": "1.0.0",
  "scripts": { "dev": "node churn.js" },
  "dependencies": {}
}
EOF
        cat > mgc.toml <<'EOF'
name = "native-process-churn-e2e"
version = "1.0.0"
ecosystem = "web"
EOF
        cat > churn.js <<'EOF'
const { spawn } = require("node:child_process");
const deadline = Date.now() + 8000;
let active = 0;
let spawned = 0;
let failed = 0;
function refill() {
  while (active < 3 && Date.now() < deadline) {
    active++;
    const child = spawn(process.execPath, ["-e", "setTimeout(() => {}, 35)"], { stdio: "ignore" });
    let settled = false;
    const done = () => {
      if (!settled) {
        settled = true;
        active--;
      }
    };
    child.once("spawn", () => { spawned++; });
    child.once("exit", done);
    child.once("error", () => {
      failed++;
      done();
    });
  }
  if (Date.now() >= deadline && active === 0) {
    console.log(`spawned=${spawned} failed=${failed}`);
    return;
  }
  setTimeout(refill, 10);
}
refill();
EOF

        CHURN_INSTALL_LOG="$TEMP_BASE/process-churn-install.log"
        if ! "$MGC_BIN" install >"$CHURN_INSTALL_LOG" 2>&1; then
            echo "✗ FAIL: native install did not prepare the process-churn fixture:"
            cat "$CHURN_INSTALL_LOG"
            exit 1
        fi
        CHURN_LOG="$TEMP_BASE/native-process-churn.log"
        if ! "$MGC_BIN" dev >"$CHURN_LOG" 2>&1; then
            echo "✗ FAIL: real mgc dev failed while short-lived Node descendants were exiting:"
            cat "$CHURN_LOG"
            exit 1
        fi
        if ! grep -Eq "spawned=([1-9][0-9]{2,})" "$CHURN_LOG"; then
            echo "✗ FAIL: process-churn fixture did not spawn at least 100 short-lived children:"
            cat "$CHURN_LOG"
            exit 1
        fi
        echo "✓ real mgc dev completed while pruning short-lived descendant process records"
        cd "$TEMP_BASE"
        ;;
esac

echo "--- Test 5: real mgc dev stops a detached child before npm can run ---"
PROCESS_GUARD_E2E="skipped"
case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
        echo "SKIP: the detached-session regression uses POSIX sessions; Windows is covered by Job Object tests"
        ;;
    *)
        GUARD_PROJECT="$TEMP_BASE/proj-native-process-guard"
        mkdir -p "$GUARD_PROJECT"
        cd "$GUARD_PROJECT"
        cat > package.json <<'EOF'
{
  "name": "native-process-guard-e2e",
  "version": "1.0.0",
  "scripts": { "dev": "node server.js" },
  "dependencies": {}
}
EOF
        cat > mgc.toml <<'EOF'
name = "native-process-guard-e2e"
version = "1.0.0"
ecosystem = "web"
EOF
        INSTALL_LOG="$TEMP_BASE/process-guard-install.log"
        if ! "$MGC_BIN" install >"$INSTALL_LOG" 2>&1; then
            echo "✗ FAIL: native install did not prepare the process-guard project:"
            cat "$INSTALL_LOG"
            exit 1
        fi

        GUARD_DIR="$TEMP_BASE/process-guard-fixture"
        mkdir -p "$GUARD_DIR"
        FAKE_NPM="$GUARD_DIR/npm"
        HELPER_JS="$GUARD_DIR/detached-helper.js"
        HELPER_READY_FILE="$GUARD_DIR/helper-ready"
        HELPER_PID_FILE="$GUARD_DIR/helper-pid"
        FAKE_PM_STARTED_FILE="$GUARD_DIR/fake-pm-started"
        FAKE_PM_PID_FILE="$GUARD_DIR/fake-pm-pid"
        FAKE_PM_PROBE_FILE="$GUARD_DIR/fake-pm-probe"
        cat > "$FAKE_NPM" <<EOF
#!/bin/sh
if [ "\$1" = "--probe" ]; then
  printf ready > "$FAKE_PM_PROBE_FILE"
  exit 0
fi
printf '%s\n' "\$\$" > "$FAKE_PM_PID_FILE"
printf started > "$FAKE_PM_STARTED_FILE"
exec /bin/sleep 5
EOF
        chmod +x "$FAKE_NPM"
        "$FAKE_NPM" --probe
        if [ ! -f "$FAKE_PM_PROBE_FILE" ]; then
            echo "✗ FAIL: detached process-guard fixture did not execute its probe"
            exit 1
        fi
        cat > "$HELPER_JS" <<EOF
const fs = require("node:fs");
const { spawn } = require("node:child_process");
fs.writeFileSync("$HELPER_PID_FILE", String(process.pid));
fs.writeFileSync("$HELPER_READY_FILE", "ready");
setTimeout(() => {
  const child = spawn("$FAKE_NPM", ["install"], { stdio: "ignore" });
  child.on("error", (error) => {
    console.error(error);
    process.exitCode = 1;
  });
}, 300);
EOF
        cat > server.js <<EOF
const fs = require("node:fs");
const { spawn } = require("node:child_process");
const child = spawn(process.execPath, ["$HELPER_JS"], {
  detached: true,
  stdio: "ignore",
});
child.once("error", (error) => {
  console.error(error);
  process.exit(88);
});
const deadline = Date.now() + 2000;
function waitForDetachedChild() {
  if (fs.existsSync("$HELPER_READY_FILE")) {
    process.exit(0);
  }
  if (Date.now() >= deadline) {
    console.error("detached helper did not become ready");
    process.exit(89);
  }
  setTimeout(waitForDetachedChild, 5);
}
child.once("spawn", waitForDetachedChild);
EOF

        GUARD_LOG="$TEMP_BASE/native-process-guard.log"
        GUARD_EXIT=0
        "$MGC_BIN" dev >"$GUARD_LOG" 2>&1 || GUARD_EXIT=$?
        if [ "$GUARD_EXIT" -eq 0 ]; then
            echo "✗ FAIL: real mgc dev returned success while its detached child remained"
            cat "$GUARD_LOG"
            exit 1
        fi
        if [ ! -f "$HELPER_READY_FILE" ]; then
            echo "✗ FAIL: real mgc dev failed before the detached Node fixture reached its ready point:"
            cat "$GUARD_LOG"
            exit 1
        fi
        if ! grep -Eqi "forbidden package manager 'npm' spawned|child processes remained|process tree" "$GUARD_LOG"; then
            echo "✗ FAIL: real mgc dev failed without reporting process-guard enforcement:"
            cat "$GUARD_LOG"
            exit 1
        fi

        for _ in $(seq 1 40); do
            [ ! -f "$FAKE_PM_STARTED_FILE" ] || break
            /bin/sleep 0.05
        done
        if [ -f "$FAKE_PM_STARTED_FILE" ]; then
            if [ -s "$FAKE_PM_PID_FILE" ]; then
                kill -KILL "$(cat "$FAKE_PM_PID_FILE")" 2>/dev/null || true
            fi
            echo "✗ FAIL: detached code reached the fake npm side-effect marker"
            cat "$GUARD_LOG"
            exit 1
        fi
        echo "✓ real mgc dev rejected the detached child before its npm side effect"
        PROCESS_GUARD_E2E="passed"
        cd "$TEMP_BASE"
        ;;
esac

echo
echo "--- Test 6: optimizer-generated Rust profiles affect real mgc build and test ---"
RUST_PROJECT="$TEMP_BASE/proj-rust-optimizer"
mkdir -p "$RUST_PROJECT/src"
cat > "$RUST_PROJECT/Cargo.toml" <<'EOF'
[package]
name = "mgc-optimizer-profile-e2e"
version = "0.1.0"
edition = "2024"
build = "build.rs"
EOF
cat > "$RUST_PROJECT/build.rs" <<'EOF'
use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=MGC_EXPECTED_OPT_LEVEL");
    println!("cargo:rerun-if-env-changed=MGC_EXPECTED_CARGO_JOBS");
    let expected_level = env::var("MGC_EXPECTED_OPT_LEVEL")
        .expect("E2E expected optimizer profile level must be set");
    let actual_level = env::var("OPT_LEVEL").expect("Cargo must expose the active OPT_LEVEL");
    assert_eq!(actual_level, expected_level, "optimizer profile was not applied");
    if let Ok(expected_jobs) = env::var("MGC_EXPECTED_CARGO_JOBS") {
        let actual_jobs = env::var("NUM_JOBS").expect("Cargo must expose the configured NUM_JOBS");
        assert_eq!(actual_jobs, expected_jobs, "optimizer Cargo jobs setting was not applied");
    }
}
EOF
cat > "$RUST_PROJECT/src/lib.rs" <<'EOF'
pub fn optimizer_profile_is_live() -> bool {
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn product_test_runner_builds_with_the_optimizer_profile() {
        assert!(super::optimizer_profile_is_live());
    }
}
EOF
cargo generate-lockfile --offline --manifest-path "$RUST_PROJECT/Cargo.toml"
cd "$RUST_PROJECT"
"$MGC_BIN" optimizer --force
OPTIMIZER_CONFIG="$RUST_PROJECT/.mgc-optimizer/rust_cargo_profile.toml"
if [ ! -f "$OPTIMIZER_CONFIG" ]; then
    echo "✗ FAIL: real mgc optimizer did not create the Rust Cargo profile"
    exit 1
fi
EXPECTED_OPT_LEVEL=""
EXPECTED_CARGO_JOBS=""
OPTIMIZER_SECTION=""
while IFS= read -r CONFIG_LINE || [ -n "$CONFIG_LINE" ]; do
    case "$CONFIG_LINE" in
        "[profile.dev]") OPTIMIZER_SECTION="profile" ;;
        "[build]") OPTIMIZER_SECTION="build" ;;
        "["*) OPTIMIZER_SECTION="" ;;
        opt-level*)
            if [ "$OPTIMIZER_SECTION" = "profile" ]; then
                EXPECTED_OPT_LEVEL="${CONFIG_LINE#*=}"
                EXPECTED_OPT_LEVEL="${EXPECTED_OPT_LEVEL//[[:space:]]/}"
                EXPECTED_OPT_LEVEL="${EXPECTED_OPT_LEVEL//\"/}"
            fi
            ;;
        jobs*)
            if [ "$OPTIMIZER_SECTION" = "build" ]; then
                EXPECTED_CARGO_JOBS="${CONFIG_LINE#*=}"
                EXPECTED_CARGO_JOBS="${EXPECTED_CARGO_JOBS//[[:space:]]/}"
            fi
            ;;
    esac
done < "$OPTIMIZER_CONFIG"
if [ -z "$EXPECTED_OPT_LEVEL" ]; then
    echo "✗ FAIL: generated optimizer profile has no development optimization level"
    exit 1
fi
export MGC_EXPECTED_OPT_LEVEL="$EXPECTED_OPT_LEVEL"
if [ -n "$EXPECTED_CARGO_JOBS" ]; then
    export MGC_EXPECTED_CARGO_JOBS="$EXPECTED_CARGO_JOBS"
fi
OPTIMIZER_INSTALL_LOG="$TEMP_BASE/rust-optimizer-install.log"
if ! "$MGC_BIN" install >"$OPTIMIZER_INSTALL_LOG" 2>&1; then
    echo "✗ FAIL: native install did not prepare the Rust optimizer project:"
    cat "$OPTIMIZER_INSTALL_LOG"
    exit 1
fi
if [ ! -f "$RUST_PROJECT/mgc.lock" ]; then
    echo "✗ FAIL: native install did not create the Rust project lock"
    exit 1
fi
"$MGC_BIN" build
rm -rf "$RUST_PROJECT/target"
"$MGC_BIN" test
node - "$RUST_PROJECT/.magicore/exec.log" <<'NODE'
const fs = require("node:fs");
const path = require("node:path");
const [auditPath] = process.argv.slice(2);
const expectedCwd = process.cwd();
const rows = fs.readFileSync(auditPath, "utf8").trim().split(/\r?\n/).map((line) => JSON.parse(line));
const samePath = (actual, expected) => {
  try {
    const actualPath = fs.realpathSync(actual);
    const expectedPath = fs.realpathSync(expected);
    return process.platform === "win32"
      ? path.normalize(actualPath).toLowerCase() === path.normalize(expectedPath).toLowerCase()
      : actualPath === expectedPath;
  } catch {
    return false;
  }
};
for (const subcommand of ["build", "test"]) {
  const found = rows.some((row) => row.cmd === "cargo"
    && Array.isArray(row.args) && row.args[0] === subcommand
    && row.args.includes("--locked") && (subcommand === "build" || row.args.includes("--offline"))
    && samePath(row.cwd, expectedCwd)
    && row.exit_code === 0);
  if (!found) {
    console.error(`Audit log did not record a successful native Cargo ${subcommand}`);
    const cargoRows = rows.filter((row) => typeof row.cmd === "string"
      && row.cmd.toLowerCase().split(/[\\/]/).pop().replace(/\.exe$/, "") === "cargo")
      .map((row) => {
        let cwdMatches = false;
        try { cwdMatches = samePath(row.cwd, expectedCwd); } catch {}
        return { args: row.args, cwd: row.cwd, cwdMatches, exit_code: row.exit_code };
      });
    console.error(`Expected project cwd: ${expectedCwd}`);
    console.error(`Observed Cargo audit rows: ${JSON.stringify(cargoRows)}`);
    process.exit(1);
  }
}
NODE
echo "✓ optimizer-generated Cargo job/profile settings affected real mgc build and test"

echo
echo "Compat-runtime E2E complete:"
echo "  ✓ native and historical compat flags refuse bun + deno scripts"
echo "  ✓ explicit compat gate refuses the unsupported runtime clearly"
echo "  ✓ successful native dev execution is recorded at the canonical project audit path"
echo "  ✓ optimizer-generated Rust Cargo profiles are consumed by real build and test commands"
if [ "$PROCESS_GUARD_E2E" = "passed" ]; then
    echo "  ✓ real mgc dev stops a detached child before package-manager side effects"
else
    echo "  SKIP: detached-session process-guard E2E is Unix-only; Windows uses Job Object tests"
fi
echo "NOTE: dev process-guard checks and Rust optimizer build/test are separate product-evidence slices; neither establishes universal framework support."
exit 0
