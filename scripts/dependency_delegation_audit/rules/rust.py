"""Rust process-boundary classifications and source patterns.

The reviewed literals are unchanged from the security gate baseline."""

from __future__ import annotations

import re

from .common import TOOLS

_TOOL_ALT = "|".join(re.escape(tool) for tool in TOOLS)

# Exact first-party OS process helpers are kept distinct from ecosystem tool
# delegation. They remain blocking `review-required` findings: this exception
# does not approve the implementation or claim native behavior, it only keeps
# an OS process-control primitive from being mislabeled as a package manager.
# Exact helper/path/tool matching is intentional; dynamic executables and any
# package/build tool launched from the same function still use normal rules.
# (Các helper OS được phân loại riêng và vẫn blocking; không miễn executable
# động hoặc package/build tool gọi từ cùng hàm.)
PLATFORM_PROCESS_BOUNDARY_REVIEWS = {
    (
        "core/crates/mgc-exec/src/run/run.rs",
        "execute_command",
        "cmd.exe",
    ): "Windows batch-script command wrapper; review shell-boundary safety",
    (
        "core/crates/mgc-exec/src/run/run.rs",
        "terminate_process_tree",
        "taskkill",
    ): "Windows process-tree termination; review native Job Object replacement",
    (
        "core/crates/mgc-exec/src/run/run.rs",
        "find_forbidden_descendant",
        "ps",
    ): "Unix process-table inspection; review native platform API replacement",
}

# These two first-party toolchain calls are explicitly constrained by
# `mgc-exec`: Cargo is locked/offline and rustc is limited to the read-only
# host-target query. Keep the exception exact by source, function, and tool.
# (Hai lệnh toolchain nội bộ này bị giới hạn bởi `mgc-exec`: Cargo khóa
# dependency/offline, rustc chỉ truy vấn host-target chỉ đọc. Ngoại lệ phải
# khớp chính xác file, hàm và executable.)
# A `src/**/test/**` module is a test owner (RULE §5): its helper functions
# are compiled only under `#[cfg(test)]` even when they are not `#[test]`
# themselves. Matching is path-scoped so production routes are unaffected.
# (Module `src/**/test/**` là owner test (RULE §5): helper của nó chỉ biên
# dịch dưới `#[cfg(test)]` dù bản thân không mang `#[test]`. Khớp theo path nên
# route production không bị ảnh hưởng.)
TEST_ONLY_MODULE_REVIEWS = {
    "cli/src/commands/core/dev/cicd/templates/test/catalog.rs",
}

# The web lifecycle route is user-policy gated, parsed as one non-shell
# invocation, and constrained to the install execution scope.
# Route lifecycle web bị policy user chặn/mở, parse thành lệnh đơn không shell,
# và dùng phạm vi thực thi Install.
# Publish lifecycle execution requires a separate explicit one-command opt-in.
# Hook publish cần opt-in tường minh riêng cho từng lần chạy.
# Publish's Git checks accept only the closed read-only query enum and run
# against the selected project root; dynamic refs follow --end-of-options.
# Git check của publish chỉ nhận enum truy vấn đóng, chỉ đọc, chạy tại root
# project được chọn; ref động nằm sau --end-of-options.
AUDITED_EXECUTOR_ROUTES = {
('cli/src/commands/core/web/create/initializer/host_tool/host_tool.rs', 'run_host_tool', '<dynamic:program>'): 'framework host-tool initializer from the static catalog',
 ('cli/src/commands/core/web/create/initializer/host_tool/host_tool.rs', 'run_host_tool', '<dynamic:program>'): 'framework host-tool initializer from the static catalog',
 ('cli/src/scaffold/core_initializers/host_tool/host_tool.rs', 'run', '<dynamic:record>'): 'core host-tool initializer from the static catalog',
 ('cli/src/scaffold/core_initializers/host_tool/host_tool.rs', 'run', '<dynamic:record>'): 'core host-tool initializer from the static catalog',
('core/crates/mgc-exec/src/run/process_policy/invocation_shape/invocation_shape.rs', 'safe_pm_argument_shape', 'swift'): 'constant allow-list of safe Swift/PM subcommands; '
                                                                                                                    'no process is spawned here, the list only '
                                                                                                                    'shapes arguments for the guarded executor',
('cli/src/commands/model/bench/bench.rs', 'measure_runtime', '<dynamic:binary>'): 'explicit `mgc model bench` measurement: the runtime name is one of '
                                                                                    'CANDIDATE_RUNTIMES constants resolved on PATH, run through mgc-exec '
                                                                                    '(TestRunner scope + allowlist + 120s timeout)',
('cli/src/commands/core/install/library/install_lib/install_lib.rs', 'version_of', '<dynamic:binary>'): 'explicit `mgc install-lib-binary <target>` probe: the executable is the '
                                                                                                    'user-supplied target of that command, run through mgc-exec (execution '
                                                                                                    'scope + allowlist + timeout), and only `--version` is requested',
('core/crates/mgc-resolver/src/protocols/pypi/python_compatibility/python_compatibility.rs', 'probe_interpreter', '<dynamic:program>'): 'wheel-ABI consumer probe: the program is a `PYTHON_LAUNCHERS` constant '
                                                                                                    '(`python`/`python3`, the launchers `mgc run`/`mgc dev` use) run '
                                                                                                    'through mgc-exec TestRunner scope + allowlist + fixed 10s timeout, '
                                                                                                    'requesting only `-c` version output; no user input selects it',
('cli/src/commands/exec/exec.rs', 'run', '<dynamic:command>'): 'mgc exec runs the user command through the mgc-exec allowlist with '
                                                                                 'ExecutionScope::ScriptRunner; the allowlist and exec audit '
                                                                                 'gate every spawn before it happens',
('tools/mgc-dist/src/main.rs', 'build_package', 'cargo'): 'mgc-exec BuildRunner with --locked --offline; '
                                                           'package build route',
 ('tools/mgc-dist/src/main.rs', 'detect_host_target', 'rustc'): 'mgc-exec BuildRunner restricted to rustc '
                                                                '-vV; read-only host query',
 ('cli/src/commands/build/web_engine/rust_build/rust_build.rs', 'build_rust_with_env', 'cargo'): 'mgc-exec '
                                                                                                 'BuildRunner; '
                                                                                                 'Cargo '
                                                                                                 'build '
                                                                                                 'requires '
                                                                                                 '--locked '
                                                                                                 '--offline',
 ('cli/src/commands/build/web_engine/rust_build/rust_build.rs', 'build_rust_with_managed_source', 'cargo'): 'mgc-exec '
                                                                                                            'BuildRunner; '
                                                                                                            'verified '
                                                                                                            'MGC '
                                                                                                            'directory '
                                                                                                            'source, '
                                                                                                            'Cargo '
                                                                                                            'build '
                                                                                                            '--locked '
                                                                                                            '--offline',
 ('cli/src/commands/build/build/lib/lib.rs', 'build_lib', 'node'): 'mgc-exec BuildRunner; node executes the '
                                                                   'project-local tsc entry',
 ('cli/src/commands/build/build/cloud/cloud.rs', 'build_cloud', 'node'): 'mgc-exec BuildRunner; node '
                                                                         'executes the project-local CDK '
                                                                         'synth entry',
 ('cli/src/commands/core/dev/app/ios_simulator/ios_simulator.rs', 'run_xcrun', 'xcrun'): 'mgc-exec '
                                                                                         'DeviceControl; '
                                                                                         'allowlisted iOS '
                                                                                         'simulator/device '
                                                                                         'commands',
 ('cli/src/commands/core/dev/app/android_device/android_device.rs', 'run_android_device_command', 'adb'): 'mgc-exec '
                                                                                                          'DeviceControl; '
                                                                                                          'allowlisted '
                                                                                                          'Android '
                                                                                                          'device '
                                                                                                          'commands',
 ('adapters/web/src/lifecycle/script_execution.rs', 'run_script', '<dynamic:invocation>'): 'mgc-exec '
                                                                                           'Install; parsed '
                                                                                           'non-shell script '
                                                                                           'from '
                                                                                           'trust-approved '
                                                                                           'lifecycle '
                                                                                           'install',
 ('cli/src/commands/publish/lifecycle/lifecycle.rs', 'run_lifecycle', '<dynamic:invocation>'): 'mgc-exec '
                                                                                               'Install; '
                                                                                               'non-shell '
                                                                                               'lifecycle '
                                                                                               'runner after '
                                                                                               'explicit '
                                                                                               '--allow-scripts',
 ('cli/src/commands/publish/git_checks/git_checks.rs', 'run_git_capture', 'git'): 'mgc-exec; closed '
                                                                                  'read-only Git query enum '
                                                                                  'at the selected project '
                                                                                  'root',
 ('cli/src/commands/tool/native_install/native_install.rs', 'install_with_project_credentials', 'run_native_install'): 'MagiCore '
                                                                                                                       'self-child '
                                                                                                                       'only; '
                                                                                                                       'fixed '
                                                                                                                       'Web '
                                                                                                                       'install '
                                                                                                                       'argv, '
                                                                                                                       'mgc-exec '
                                                                                                                       'Install '
                                                                                                                       'scope, '
                                                                                                                       'and '
                                                                                                                       'package '
                                                                                                                       'lifecycle '
                                                                                                                       'scripts '
                                                                                                                       'remain '
                                                                                                                       'opt-in',
 ('cli/src/commands/build/build/ai/ai.rs', 'build_ai', 'go'): 'mgc-exec BuildRunner; Go build uses '
                                                              '-mod=readonly and offline environment',
 ('cli/src/commands/build/build/web_go_backend/web_go_backend.rs', 'build_web_go_backend', 'go'): 'mgc-exec '
                                                                                                  'BuildRunner; '
                                                                                                  'Go web '
                                                                                                  'build '
                                                                                                  'uses '
                                                                                                  '-mod=readonly '
                                                                                                  'and '
                                                                                                  'offline '
                                                                                                  'environment',
 ('cli/src/commands/build/build/lib/lib.rs', 'build_lib', 'go'): 'mgc-exec BuildRunner; Go build uses '
                                                                 '-mod=readonly and offline environment',
 ('cli/src/commands/build/build/app/app.rs', 'build_app', 'gradle'): 'mgc-exec BuildRunner; Gradle build is '
                                                                     '--offline',
 ('cli/src/commands/build/build/app/app.rs', 'build_app', 'swift'): 'mgc-exec BuildRunner; Swift build '
                                                                    'disables package resolution',
 ('cli/src/commands/build/build/app/app.rs', 'build_app', 'flutter'): 'mgc-exec BuildRunner; Flutter build '
                                                                      'uses --no-pub',
 ('cli/src/commands/build/build/app/multi/multi.rs', 'build_multi_app', 'swift'): 'mgc-exec BuildRunner; '
                                                                                  'Swift build disables '
                                                                                  'package resolution',
 # Host-tool and core initializers run a program the framework team publishes.
 # Both probe `--version` first, then run with a fixed argv whose only variable
 # part is the project path. The program name comes from a static catalog, never
 # from project input.
 # (Initializer host-tool và core chạy chương trình do framework phát hành.
 # Probe `--version` trước, argv cố định, chỉ đường dẫn project là biến đổi.
 # Tên chương trình đến từ catalog tĩnh, không từ input project.)
  ('cli/src/commands/build/web_engine/framework_script/framework_script.rs', 'run_framework_build_if_supported', '<dynamic:program>'): 'script '
                                                                                                                                      'parser '
                                                                                                                                      '+ '
                                                                                                                                      'framework '
                                                                                                                                      'allowlist '
                                                                                                                                      '+ '
                                                                                                                                      'BuildRunner '
                                                                                                                                      'deny '
                                                                                                                                      'package '
                                                                                                                                      'managers',
 ('cli/src/commands/build/web_engine/tool_execution/tool_execution.rs', 'run_allowlisted_tool_with_env', '<dynamic:program>'): 'BuildRunner '
                                                                                                                               'allowlist '
                                                                                                                               '+ '
                                                                                                                               'dependency-resolution '
                                                                                                                               'guard '
                                                                                                                               'in '
                                                                                                                               'mgc-exec',
 ('cli/src/commands/core/dev/app/tool_execution/tool_execution.rs', 'run_tool_with_env', '<dynamic:cmd>'): 'closed '
                                                                                                           'app '
                                                                                                           'toolchain '
                                                                                                           'set '
                                                                                                           '+ '
                                                                                                           'DevServer '
                                                                                                           'scope '
                                                                                                           '+ '
                                                                                                           'mgc-exec '
                                                                                                           'package-manager '
                                                                                                           'deny',
 ('cli/src/commands/core/dev/app/command_builder/command_builder.rs', 'flutter_dev_command', 'flutter'): 'static '
                                                                                                         'Flutter '
                                                                                                         'device '
                                                                                                         'command; '
                                                                                                         '--no-pub '
                                                                                                         'and '
                                                                                                         'mgc-exec '
                                                                                                         'DevServer '
                                                                                                         'boundary',
 ('cli/src/commands/core/dev/app/swift_package/swift_package.rs', 'run_swift_package', 'swift'): 'Swift app '
                                                                                                 'freshness '
                                                                                                 'preflight; '
                                                                                                 'fixed '
                                                                                                 'SwiftPM '
                                                                                                 'run argv '
                                                                                                 'disables '
                                                                                                 'updates '
                                                                                                 'and '
                                                                                                 'automatic '
                                                                                                 'resolution '
                                                                                                 'through '
                                                                                                 'mgc-exec '
                                                                                                 'DevServer '
                                                                                                 'scope',
 ('cli/src/commands/core/dev/iot/tool_execution/tool_execution.rs', 'run_tool', '<dynamic:cmd>'): 'closed '
                                                                                                  'cargo/espflash '
                                                                                                  'selector '
                                                                                                  'with '
                                                                                                  'BuildRunner '
                                                                                                  'or '
                                                                                                  'DeviceControl '
                                                                                                  'scope',
 ('cli/src/commands/core/ai/dev_runner/dev_runner.rs', 'run_ai_dev_at', '<dynamic:cmd>'): 'python_cmd '
                                                                                          'selector + '
                                                                                          'DevServer scope + '
                                                                                          'mgc-exec '
                                                                                          'package-manager '
                                                                                          'deny',
 ('cli/src/commands/core/web/dev/process_supervision/run_dev_launch_with_guard/run_dev_launch_with_guard.rs', 'run_dev_launch_with_guard', '<dynamic:launch>'): 'validated '
                                                                                                                                                                'framework '
                                                                                                                                                                'launch '
                                                                                                                                                                '+ '
                                                                                                                                                                'DevServer '
                                                                                                                                                                'scope '
                                                                                                                                                                '+ '
                                                                                                                                                                'package-manager '
                                                                                                                                                                'script '
                                                                                                                                                                'rejection',
 ('cli/src/commands/core/dev/game/game.rs', 'dev', '<dynamic:cmd>'): 'core-specific game/IoT command '
                                                                     'builders + DevServer/DeviceControl '
                                                                     'mgc-exec scopes',
 ('cli/src/commands/core/dev/iot/dev_command/dev_command.rs', 'dev', '<dynamic:cmd>'): 'core-specific '
                                                                                       'game/IoT command '
                                                                                       'builders + '
                                                                                       'DevServer/DeviceControl '
                                                                                       'mgc-exec scopes',
 ('cli/src/commands/exec/exec.rs', 'run', '<dynamic:command>'): 'explicit user-selected executable passes '
                                                           'mgc-exec allowlist and dependency-resolution '
                                                           'guards',
 ('cli/src/commands/run/task_execution/executor/executor.rs', 'execute_task_with_bin', '<dynamic:program>'): 'parsed '
                                                                                                             'project '
                                                                                                             'task '
                                                                                                             '+ '
                                                                                                             'package-manager '
                                                                                                             'script '
                                                                                                             'rejection '
                                                                                                             '+ '
                                                                                                             'mgc-exec '
                                                                                                             'allowlist',
 ('cli/src/commands/test/test.rs', 'test_at', '<dynamic:runner>'): 'detected test runner + TestRunner scope '
                                                                   '+ mgc-exec allowlist',
 ('core/crates/mgc-config/src/hooks/execution/execution.rs', 'run_hooks_with_config', '<dynamic:program>'): 'parsed '
                                                                                                            'argv '
                                                                                                            '+ '
                                                                                                            'dependency '
                                                                                                            'lifecycle '
                                                                                                            'hooks '
                                                                                                            'denied '
                                                                                                            '+ '
                                                                                                            'TestRunner '
                                                                                                            'scope',
 ('core/crates/mgc-exec/src/run/run.rs', 'run_inherited', '<dynamic:cmd>'): 'central executor checks scope '
                                                                            'allowlist, forbidden package '
                                                                            'managers, and script policy '
                                                                            'before spawn'}

# Direct `Command::new` constructors are not execution delegation. Only the
# executor's private spawn primitive and its fixed Windows batch interpreter
# receive an exact source/function/target route here. Docker's async stdin
# handoff also has one dedicated route with a closed executable and argv.
# Constructor `Command::new` không tự chứng minh delegation; chỉ cho phép
# primitive nội bộ, batch interpreter và Docker async handoff theo route hẹp.
AUDITED_DIRECT_PROCESS_ROUTES = {('core/crates/mgc-exec/src/run/spawn/command_builder/command_builder.rs', 'build_command', '<dynamic:resolved_cmd>'): 'private command-construction primitive reached only after validated executor preflight; platform-specific argv construction remains guarded',
 ('core/crates/mgc-exec/src/run/spawn/windows_command/windows_command.rs', 'windows_batch_command', '<dynamic:windows_system_tool_path>'): 'fixed '
                                                                                                                                           'System32 '
                                                                                                                                           'cmd.exe; '
                                                                                                                                           'batch '
                                                                                                                                           'path '
                                                                                                                                           'and '
                                                                                                                                           'argv '
                                                                                                                                           'reject '
                                                                                                                                           'cmd.exe '
                                                                                                                                           'operators '
                                                                                                                                           'before '
                                                                                                                                           'construction',
 ('cli/src/commands/publish/trusted_oci_image/trusted_oci_image.rs', 'docker_command', 'docker'): 'fixed '
                                                                                                  'Docker '
                                                                                                  'CLI; argv '
                                                                                                  'only, '
                                                                                                  'isolated '
                                                                                                  'config, '
                                                                                                  'secret '
                                                                                                  'via '
                                                                                                  'stdin, '
                                                                                                  'async '
                                                                                                  'timeout, '
                                                                                                  'and '
                                                                                                  'kill-on-drop',
 ('core/crates/mgc-platform/src/paths/shell/windows/windows.rs', 'ensure_windows_user_path', 'powershell.exe'): 'Windows '
                                                                                                                'user '
                                                                                                                'PATH '
                                                                                                                'registration '
                                                                                                                'only; '
                                                                                                                'fixed '
                                                                                                                'non-interactive '
                                                                                                                'script '
                                                                                                                'receives '
                                                                                                                'one '
                                                                                                                'path '
                                                                                                                'through '
                                                                                                                'an '
                                                                                                                'environment '
                                                                                                                'variable'}

# Spawn-shaped contexts. Applied to the whole file text (so a tool literal
# on its own line inside a multi-line `run(\n "cargo",` call still matches),
# then mapped back to a line number. Each entry is (pattern,
# matches_inside_message): only the weak bare-literal pattern is silenced
# inside message-macro spans (see MESSAGE_MACROS).
# (Ngữ cảnh hình-spawn. Áp trên cả text file (để literal tool nằm dòng riêng
# trong call nhiều dòng vẫn khớp), rồi map về số dòng. Mỗi entry là
# (pattern, khớp_trong_message): chỉ pattern bare-literal yếu bị tắt trong
# span macro-message (xem MESSAGE_MACROS).)
SPAWN_PATTERNS = [
    # A known dynamic process wrapper stays blocked except at an exact,
    # source-bound MagiCore self-install route in AUDITED_EXECUTOR_ROUTES.
    # Wrapper process động vẫn bị chặn trừ route self-install MGC đã review chính xác.
    (re.compile(r"(?<!fn )\b(run_native_install)\s*\("), True),
    # Capture every literal executable, not only names in TOOLS. TOOLS is
    # an inventory for dispatch/string-table shapes, not a scanner boundary.
    # (Bắt mọi executable literal; TOOLS chỉ dùng cho dạng dispatch/bảng.)
    (re.compile(r'\bexec_tool\s*\([^()]*?"([^"\n]+)"'), True),
    # `run` is too common for a bare-name match: CLI command dispatchers also
    # expose `run(...)`. Match only the known process wrappers when unqualified,
    # and the explicit mgc_exec namespace for its generic `run` wrapper.
    (re.compile(r'\bmgc_exec(?:::[A-Za-z_][A-Za-z0-9_]*)*::run\s*\(\s*"([^"\n]+)"'), True),
    (re.compile(r'\bmgc_exec(?:::[A-Za-z_][A-Za-z0-9_]*)*::run_inherited\s*\(\s*"([^"\n]+)"'), True),
    (re.compile(r'(?<![\w:])(?:run_inherited|run_capture)\s*\(\s*"([^"\n]+)"'), True),
    (re.compile(r'\bmgc_run\s*\(\s*"([^"\n]+)"'), True),
    (re.compile(r'\bCommand::new\s*\(\s*"([^"\n]+)"'), True),
    (re.compile(r'\bwhich(?:::which)?\s*\(\s*"([^"\n]+)"'), True),
    # (tool, args) tables — `("cdk", vec!["deploy"])`, `("uv", vec![…])`, or
    # `("gradle", args.as_slice())`. The `.as_slice()`/`.as_ref()` form is a
    # real spawn descriptor and must not slip past the route inventory.
    # (Bảng (tool, args) gồm cả dạng `("gradle", args.as_slice())`.)
    # The `.as_slice()`/`.as_ref()` args form is only a spawn descriptor when
    # the leading literal is a real tool name; a generic tuple of file names is
    # data. Restrict that branch to known tools to avoid fixture false positives.
    # (Dạng `.as_slice()` chỉ là descriptor khi tên đầu là tool thật.)
    (re.compile(r'\(\s*"([^"\n]+)"\s*,\s*(?:&?\[|vec!)'), True),
    (re.compile(r'\(\s*"(' + "|".join(sorted(TOOLS)) + r')"\s*,\s*[A-Za-z_][A-Za-z0-9_]*\.as_(?:slice|ref)\(\)'), True),
    # Spawn descriptor fields — `tool: "flutter".to_string()`.
    (re.compile(r'\btool\s*:\s*"([^"\n]+)"'), True),
    # A bare tool literal alone on its line (variable assignment / call arg).
    (re.compile(rf'^\s*"({_TOOL_ALT})",\s*$', re.MULTILINE), False),
]

# These are static command-route declarations rather than proof that the
# current line executes a process. Keep them in the blocking ledger so the
# route cannot disappear from review, but label them separately from spawn
# calls. (Descriptor là tuyến lệnh tĩnh, không phải bằng chứng process chạy.)
DESCRIPTOR_PATTERNS = [pattern for pattern, _ in SPAWN_PATTERNS[8:]]

# A variable executable is still a process boundary. Keep these separate so
# the ledger labels it as dynamic instead of pretending the variable name is
# the actual program. Fixed structural wrappers are covered; arbitrary
# method-call expressions remain a documented scanner limitation.
# (Executable qua biến vẫn là process boundary; ledger phải gắn nhãn dynamic.)
DYNAMIC_SPAWN_PATTERNS = [
    re.compile(r'(?<!fn )\bexec_tool\s*\(\s*&?([A-Za-z_][A-Za-z0-9_]*)'),
    re.compile(r'\bmgc_run\s*\(\s*&?([A-Za-z_][A-Za-z0-9_]*)'),
    re.compile(
        r'(?<!fn )\bmgc_exec(?:::[A-Za-z_][A-Za-z0-9_]*)*::exec_tool'
        r'\s*\(\s*&?([A-Za-z_][A-Za-z0-9_]*)'
    ),
    re.compile(r'\bCommand::new\s*\(\s*&?([A-Za-z_][A-Za-z0-9_]*)'),
    re.compile(
        r'\bmgc_exec(?:::[A-Za-z_][A-Za-z0-9_]*)*::'
        r'(?:run|run_inherited|run_capture)\s*\(\s*&?([A-Za-z_][A-Za-z0-9_]*)'
    ),
    re.compile(
        r'(?<![\w:])(?:run_inherited|run_capture)\s*\(\s*&?([A-Za-z_][A-Za-z0-9_]*)'
    ),
]

EXECUTOR_ROUTE_PATTERNS = {
    SPAWN_PATTERNS[0][0],
    *(SPAWN_PATTERNS[index][0] for index in range(2, 6)),
    *DESCRIPTOR_PATTERNS,
    DYNAMIC_SPAWN_PATTERNS[1],
    DYNAMIC_SPAWN_PATTERNS[4],
    DYNAMIC_SPAWN_PATTERNS[5],
}

DIRECT_PROCESS_ROUTE_PATTERNS = {
    SPAWN_PATTERNS[6][0],
    DYNAMIC_SPAWN_PATTERNS[3],
}

# Top-level deny/allow-list declarations (`const X: &[&str] = &[…];`)
# name tools without spawning them — a bare literal there is a label, not
# a spawn. Only the weak bare-literal pattern consults these spans, and
# only OUTSIDE function bodies (a `const` initializer can never execute).
# Residual risk (documented): a deny-list-shaped array iterated by a
# dynamic spawn loop (`for t in TOOLS { run(t) }`) would also be skipped —
# but that loop carries no tool literal for ANY pattern to catch, so
# nothing measurable is lost; call-shaped spawns stay fully active.
# (Khai báo deny/allow-list top-level (`const X: &[&str]`) chỉ nêu tên
# tool, không spawn. Chỉ pattern bare-literal yếu dùng các span này, và
# chỉ NGOÀI thân hàm.)
CONST_ARRAY_RE = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?const\s+[A-Za-z_][A-Za-z0-9_]*\s*:[^=;]*=\s*&\[\s*$"
)

# Contexts that only LOOK like spawns: metadata labels and cache-directory
# naming never execute the tool.
# (Ngữ cảnh trông giống spawn nhưng không chạy tool: nhãn metadata và đặt
# tên thư mục cache.)
EXCLUDE_PATTERNS = [
    re.compile(r'\becosystem\s*[:=]'),
    re.compile(r'\bshared_root\s*\('),
    re.compile(r'^\s*//'),
    re.compile(r'^\s*#'),
]

FN_RE = re.compile(
    r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)'
)
