"""Crash-recovery lifecycle probe."""

from __future__ import annotations

from ..environment.recovery import (
    recovery_environment,
)
from ..process.process import (
    run_text_capture,
)
import os
import shutil
import subprocess
import time

# The recovery probe crashes `mgc install` at this failpoint phase (the
# FIRST phase of the web install orchestrator's generation lifecycle —
# see core/crates/mgc-store/src/failpoint KNOWN_PHASES).
# (Probe recovery crash `mgc install` ở phase failpoint này — phase ĐẦU
# TIÊN của vòng đời generation trong orchestrator install web.)
RECOVERY_PHASE = "after-generation-begin"

RECOVERY_READY_TIMEOUT_S = 60

def _recovery_probe(mgc_bin: str, project_path: str, sandbox: str,
                    timeout_s: int, detail: dict) -> bool:
    """THE recovery dimension's evidence: run `mgc install` with the
    built-in failpoint handshake — the binary parks at after-generation-
    begin after fsync-ing a READY marker, the harness SIGKILLs it, then
    `mgc store doctor --repair` must leave the store HEALTHY with 0 stale
    staging. Mirrors the Rust contract in cli/tests/kill_injection_matrix.rs.
    Returns True only on full pass; every violation returns False with the
    evidence tail in detail['recovery_output'] — never a guessed pass.
    (Bằng chứng dimension recovery: chạy `mgc install` với handshake
    failpoint có sẵn — binary đỗ tại after-generation-begin sau khi fsync
    marker READY, harness SIGKILL, rồi `mgc store doctor --repair` phải để
    store HEALTHY với 0 stale staging. Khuôn theo hợp đồng Rust trong
    cli/tests/kill_injection_matrix.rs. Chỉ trả True khi pass trọn vẹn;
    mọi vi phạm trả False kèm đuôi bằng chứng trong detail — không bao giờ
    đoán pass.)"""
    import signal

    # Fresh-project rule (mirrors cli/tests/kill_injection_matrix.rs: one
    # FRESH project per probe): the scenario runs on a COPY of the
    # scaffolded project WITHOUT node_modules/lockfile/store state, so the
    # crash-recovery evidence can never be masked by unrelated state from
    # the lane's earlier install (e.g. a stale lockfile tripping the
    # downgrade guard). Every install below is still a REAL install; no
    # security guard is weakened or bypassed.
    # (Luật project-mới (khuôn kill_injection_matrix.rs: mỗi probe một
    # project MỚI): kịch bản chạy trên BẢN COPY của project scaffold
    # KHÔNG kèm node_modules/lockfile/store, để bằng chứng crash-recovery
    # không bao giờ bị trạng thái cũ từ install trước đó của lane che
    # mất (vd lockfile cũ đụng downgrade guard). Mọi install bên dưới vẫn
    # là install THẬT; không guard bảo mật nào bị làm yếu hay bypass.)
    rec_project = os.path.join(sandbox, "recovery-project")
    shutil.copytree(
        project_path, rec_project,
        ignore=shutil.ignore_patterns(
            "node_modules", ".magicore*", ".venv", "target", "dist", "build"
        ),
    )

    # Isolated per-project store (the kill-matrix pattern): the whole
    # crash+repair scenario runs against ITS OWN store, never the user's.
    # (Store per-project cô lập (khuôn kill-matrix): toàn kịch bản
    # crash+repair chạy trên store RIÊNG, không bao giờ store của user.)
    env = recovery_environment(sandbox, rec_project)

    def _doctor(*extra: str):
        proc = run_text_capture(
            [mgc_bin, "store", "doctor", "--core", "web", *extra],
            capture_output=True, text=True, cwd=rec_project, env=env,
            timeout=min(timeout_s, 120),
        )
        return proc.returncode, (proc.stdout or "") + (proc.stderr or "")

    # Baseline: one healthy install populates the isolated store so the
    # victim crashes against a real (non-empty) store.
    # (Baseline: một install khỏe làm đầy store cô lập để victim crash
    # trên store thật (không rỗng).)
    try:
        baseline = run_text_capture(
            [mgc_bin, "install"], capture_output=True, text=True,
            cwd=rec_project, env=env, timeout=timeout_s,
        )
    except (OSError, subprocess.TimeoutExpired) as exc:
        detail["recovery_output"] = f"baseline install could not run: {exc}"
        return False
    if baseline.returncode != 0:
        detail["recovery_output"] = (
            "baseline install failed: "
            + (baseline.stdout or "") + (baseline.stderr or "")
        )[-2000:]
        return False

    # Victim: spawn the re-install parked at the failpoint; poll the
    # fsync'd READY marker (no guessed sleeps); SIGKILL; assert it truly
    # died by signal.
    # (Victim: spawn install-lại đỗ tại failpoint; poll marker READY đã
    # fsync (không đoán trễ); SIGKILL; khẳng định chết THẬT vì signal.)
    marker = os.path.join(sandbox, "recovery-ready.marker")
    if os.path.exists(marker):
        os.remove(marker)
    victim_err_path = os.path.join(sandbox, "recovery-victim.err")
    victim_env = dict(env)
    victim_env["MGC_FAILPOINT"] = RECOVERY_PHASE
    victim_env["MGC_FAILPOINT_READY_FILE"] = marker
    with open(os.path.join(sandbox, "recovery-victim.out"), "w") as out_f, \
            open(victim_err_path, "w") as err_f:
        try:
            victim = subprocess.Popen(
                [mgc_bin, "install"], cwd=rec_project, env=victim_env,
                stdout=out_f, stderr=err_f, start_new_session=True,
            )
        except OSError as exc:
            detail["recovery_output"] = f"victim spawn failed: {exc}"
            return False
        ready = False
        deadline = time.monotonic() + RECOVERY_READY_TIMEOUT_S
        try:
            while time.monotonic() < deadline:
                try:
                    with open(marker, "r", encoding="utf-8", errors="replace") as f:
                        if f"READY:{RECOVERY_PHASE}" in f.read():
                            ready = True
                            break
                except OSError:
                    pass
                if victim.poll() is not None:
                    break  # died/exited before parking — never a pass
                time.sleep(0.1)
            if not ready:
                if victim.poll() is None:
                    victim.kill()
                    try:
                        victim.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        pass
                return False
            os.kill(victim.pid, signal.SIGKILL)
            try:
                victim.wait(timeout=10)
            except subprocess.TimeoutExpired:
                pass
        except Exception:
            if victim.poll() is None:
                victim.kill()
            raise
    if victim.returncode is None or victim.returncode >= 0:
        try:
            with open(victim_err_path, "r", encoding="utf-8", errors="replace") as f:
                tail = f.read()[-800:]
        except OSError:
            tail = ""
        detail["recovery_output"] = (
            f"victim did not die by SIGKILL (rc={victim.returncode}); stderr tail: {tail}"
        )
        return False

    # Post-kill gates (the kill-matrix contract): no orphan claims, no lost
    # or corrupt blobs. Stale staging IS expected here — the doctor's exit
    # code at this point is ignored; the strings are the evidence.
    # (Cổng sau kill (hợp đồng kill-matrix): không claim mồ côi, không mất/
    # hỏng blob. Staging stale LÀ điều kỳ vọng — exit code doctor ở điểm
    # này bỏ qua; chuỗi output mới là bằng chứng.)
    rc, out = _doctor()
    gates = ("0 orphan claim(s)", "0 missing blob(s)", "0 corrupt blob(s)")
    if not all(g in out for g in gates):
        detail["recovery_output"] = ("post-kill doctor gates violated: " + out)[-2000:]
        return False

    # Repair must succeed, and the FINAL doctor must be HEALTHY with 0
    # stale staging (the P0-B exit contract is trusted here).
    # (Repair phải thành công, và doctor CUỐI phải HEALTHY với 0 stale
    # staging (hợp đồng exit P0-B được tin ở đây).)
    rc, out = _doctor("--repair")
    if rc != 0:
        detail["recovery_output"] = ("doctor --repair failed: " + out)[-2000:]
        return False
    rc, out = _doctor()
    if rc != 0 or "0 stale staging gen(s)" not in out or "HEALTHY" not in out:
        detail["recovery_output"] = ("final doctor not HEALTHY/0-stale: " + out)[-2000:]
        return False
    detail["recovery_output"] = (
        f"SIGKILL at {RECOVERY_PHASE} → doctor gates clean → --repair ok → "
        "final doctor HEALTHY with 0 stale staging"
    )
    return True

__all__ = ['RECOVERY_PHASE', 'RECOVERY_READY_TIMEOUT_S', '_recovery_probe']
