"""Lane-declared dependency/toolchain preparation."""

from __future__ import annotations
from ..contract.ownership import (
    _owner_pass_status,
)
from ..contract.status import (
    STATUS_FAILED,
    STATUS_NATIVE,
)
from ..diagnostics.diagnostics import (
    command_output_excerpt,
)
from ..environment.pulumi import (
    PULUMI_LIFECYCLE_STACK,
)
from ..environment.python import (
    mgc_add_python_test_build_tools,
)
from ..errors import (
    _fail,
)
from ..process.process import (
    run_text_capture,
)
from .context import (
    LaneRunContext,
)
import json
import os
import subprocess

def preparation_phase(ctx):
    lane = ctx.lane
    mgc_bin = ctx.mgc_bin
    timeout_s = ctx.timeout_s
    sandbox = ctx.sandbox
    dims = ctx.dims
    detail = ctx.detail
    project_dir = ctx.project_dir
    project_path = ctx.project_path
    lane_env = ctx.lane_env
    flutter_sdk_pub_cache = ctx.flutter_sdk_pub_cache
    flutter_sdk_home = ctx.flutter_sdk_home
    run_step = ctx.run_step

    blocked_lifecycle_steps: dict[str, str] = {}
    for step in lane.get("pre_steps", []):
        # Lane preparation INSIDE the project BEFORE the lifecycle steps
        # (mgc subcommands or provisioned tools). A failing prep is a
        # FAILED lane prep — recorded, never skipped.
        # Chuẩn bị lane TRONG project TRƯỚC các bước lifecycle. Prep fail
        # là lane prep FAILED — ghi lại, không skip.
        if dims.get("create") != STATUS_NATIVE:
            break
        if step == "mgc_add_real_dependency":
            # Run the lane-declared fixture through mgc before install. If
            # it already exists in the manifest, leave resolution to native
            # install and do not claim an add operation ran. A failed add
            # blocks lifecycle steps so an empty project cannot pass.
            # (Chạy fixture do lane khai báo qua mgc trước install. Nếu
            # dependency đã có trong manifest, để native install xử lý và
            # không claim đã chạy add. Add lỗi chặn lifecycle để project
            # rỗng không thể được tính pass.)
            fixture = lane.get("dependency_fixture")
            if not isinstance(fixture, dict):
                _fail("pre_step 'mgc_add_real_dependency' requires dependency_fixture metadata")
            command = fixture.get("command")
            package = fixture.get("package")
            if (
                not isinstance(command, str)
                or not command
                or not isinstance(package, str)
                or not package
            ):
                _fail("dependency_fixture requires non-empty command and package strings")

            if "skip_if_declared" in fixture and not isinstance(
                fixture["skip_if_declared"], bool
            ):
                _fail("dependency_fixture skip_if_declared must be a boolean")
            if fixture.get("skip_if_declared"):
                manifest_path = fixture.get("manifest_path")
                manifest_name = fixture.get("manifest_name")
                if (
                    not isinstance(manifest_path, str)
                    or not manifest_path
                    or os.path.isabs(manifest_path)
                    or not isinstance(manifest_name, str)
                    or not manifest_name
                ):
                    _fail(
                        "dependency_fixture skip_if_declared requires a relative manifest_path "
                        "and manifest_name"
                    )
                project_root = os.path.abspath(project_path)
                manifest_file = os.path.abspath(
                    os.path.join(project_root, manifest_path)
                )
                try:
                    if os.path.commonpath((project_root, manifest_file)) != project_root:
                        _fail("dependency_fixture manifest_path must stay inside the project")
                except ValueError:
                    _fail("dependency_fixture manifest_path must stay inside the project")
                try:
                    with open(manifest_file, "r", encoding="utf-8") as manifest_stream:
                        manifest = json.load(manifest_stream)
                except (OSError, json.JSONDecodeError) as exc:
                    dims["add"] = STATUS_FAILED
                    detail["add_output"] = (
                        f"dependency fixture manifest {manifest_path!r} could not be read: {exc}"
                    )[-2000:]
                    for lifecycle_step, _ in lane["steps"]:
                        blocked_lifecycle_steps[lifecycle_step] = (
                            "required dependency fixture manifest was unreadable; "
                            "this step was not run without its prerequisite"
                        )
                    continue

                dependency_sections = (
                    manifest.get("dependencies", {}),
                    manifest.get("devDependencies", {}),
                    manifest.get("optionalDependencies", {}),
                    manifest.get("peerDependencies", {}),
                ) if isinstance(manifest, dict) else ()
                if any(
                    isinstance(section, dict) and manifest_name in section
                    for section in dependency_sections
                ):
                    detail["add_output"] = (
                        f"{manifest_name} is already declared in {manifest_path}; "
                        "MGC install will resolve and materialize it"
                    )
                    continue

            try:
                proc = run_text_capture(
                    [mgc_bin, command, package],
                    capture_output=True,
                    text=True,
                    cwd=project_path, env=lane_env, timeout=timeout_s,
                )
            except FileNotFoundError:
                _fail("pre_step 'mgc_add_real_dependency' requires the mgc binary — provisioning bug")
            except subprocess.TimeoutExpired:
                _fail(f"{command} dependency fixture setup timed out after {timeout_s}s")
            if proc.returncode != 0:
                dims["add"] = STATUS_FAILED
                detail["add_output"] = (
                    f"mgc {command} {package} (dependency fixture) failed: "
                    + (proc.stdout or "") + (proc.stderr or "")
                )[-2000:]
                for lifecycle_step, _ in lane["steps"]:
                    blocked_lifecycle_steps[lifecycle_step] = (
                        "required real-dependency fixture setup failed; "
                        "this step was not run against an empty project"
                    )
            else:
                dims["add"] = _owner_pass_status(lane)
                detail["add_output"] = (
                    f"MagiCore added dependency fixture {package}"
                )
        elif step == "mgc_add_python_test_build_tools":
            # Dependency declarations themselves must pass through MGC;
            # installation is the explicit lifecycle step below.
            # Khai báo dependency cũng phải qua MGC; bước install lifecycle
            # phía dưới mới là nơi cài package.
            try:
                proc, lane_env = mgc_add_python_test_build_tools(
                    mgc_bin, project_path, lane_env, timeout_s
                )
            except (FileNotFoundError, OSError):
                _fail("pre_step 'mgc_add_python_test_build_tools' requires the mgc binary")
            except subprocess.TimeoutExpired:
                _fail(f"MGC Python tool dependency add timed out after {timeout_s}s")
            if proc.returncode != 0:
                detail["toolchain_setup_output"] = (
                    "mgc_add_python_test_build_tools failed: "
                    + (proc.stdout or "") + (proc.stderr or "")
                )[-2000:]
                for lifecycle_step in ("test", "build"):
                    if any(name == lifecycle_step for name, _ in lane["steps"]):
                        blocked_lifecycle_steps[lifecycle_step] = (
                            "MGC Python test/build dependency add failed; "
                            "the step was not run without its prerequisite"
                        )
        elif step == "initialize_local_pulumi_stack":
            backend_url = lane_env.get("PULUMI_BACKEND_URL")
            if not backend_url or not backend_url.startswith("file://"):
                _fail("Pulumi lifecycle requires an isolated file backend URL")
            commands = (
                ["pulumi", "login", backend_url, "--non-interactive"],
                ["pulumi", "stack", "init", PULUMI_LIFECYCLE_STACK, "--non-interactive"],
                ["pulumi", "stack", "select", PULUMI_LIFECYCLE_STACK, "--non-interactive"],
                ["pulumi", "stack", "get", "--show-name"],
            )
            setup_output = []
            setup_failed = False
            for command in commands:
                try:
                    proc = run_text_capture(
                        command,
                        capture_output=True,
                        text=True,
                        cwd=project_path,
                        env=lane_env,
                        timeout=timeout_s,
                    )
                except FileNotFoundError:
                    _fail("pre_step 'initialize_local_pulumi_stack' requires pulumi on PATH")
                except subprocess.TimeoutExpired:
                    _fail("Pulumi local backend setup timed out")
                setup_output.append((proc.stdout or "") + (proc.stderr or ""))
                if proc.returncode != 0 or (
                    command[:4] == ["pulumi", "stack", "get", "--show-name"]
                    and (proc.stdout or "").strip() != PULUMI_LIFECYCLE_STACK
                ):
                    setup_failed = True
                    break
            if setup_failed:
                detail["pulumi_stack_setup_output"] = command_output_excerpt(
                    "\n".join(setup_output)
                )
                if any(name == "build" for name, _ in lane["steps"]):
                    blocked_lifecycle_steps["build"] = (
                        "local Pulumi stack setup failed; preview was not run"
                    )
        else:
            _fail(f"unknown pre_step: {step}")
    ctx.lane_env = lane_env
    ctx.blocked_lifecycle_steps = blocked_lifecycle_steps

__all__ = ['preparation_phase']
