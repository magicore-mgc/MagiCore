"""Execution implementation."""

from __future__ import annotations

import posixpath
import re
import shlex

from ..rules.common import (
    TOOLS,
)

from ..rules.shell import (
    SHELL_ABSOLUTE_ENV_WRAPPERS,
    SHELL_APPLETS,
    SHELL_ASSIGNMENT_RE,
    SHELL_CONTROL_WORDS,
    SHELL_DEPENDENCY_MANAGER_ACTIONS,
    SHELL_DYNAMIC_EXECUTORS,
    SHELL_INTERPRETERS,
    SHELL_REDIRECTION_RE,
    SHELL_SCRIPT_SUFFIXES,
    SHELL_TRUSTED_SYSTEM_BIN_DIRS,
    SHELL_WRAPPERS,
    SHELL_WRAPPER_POSITIONAL_COUNTS,
    SHELL_WRAPPER_VALUE_OPTIONS,
)

from .aliases import (
    _shell_script_argument_targets,
)

def _shell_environment_unset_before(segment: list, variable: str, command_index: int) -> bool:
    """Require a clean environment before launching a Bash child.
    (Yêu cầu môi trường sạch trước khi chạy Bash con.)"""
    sanitized = False
    index = 0
    while index < command_index:
        env_token = segment[index].replace("\\", "/").lower()
        is_absolute_env = env_token in SHELL_ABSOLUTE_ENV_WRAPPERS
        is_command_env = (
            index > 0
            and segment[index - 1].lower() == "command"
            and env_token == "env"
        )
        if not (is_absolute_env or is_command_env):
            index += 1
            continue
        index += 1
        while index < command_index:
            argument = segment[index]
            if argument == "-i" or argument == "--ignore-environment":
                sanitized = True
                index += 1
                continue
            if SHELL_ASSIGNMENT_RE.match(argument):
                name = argument.partition("=")[0]
                if name == variable or re.match(r"^BASH_FUNC_[^=]*%%$", name):
                    return False
                index += 1
                continue
            if argument.startswith("-"):
                index += 1
                if argument in {"-u", "--unset"}:
                    if index >= command_index:
                        return False
                    index += 1
                    continue
                if argument.startswith("--unset="):
                    index += 1
                    continue
                if argument in SHELL_WRAPPER_VALUE_OPTIONS["env"] and index < command_index:
                    index += 1
                continue
            break
    return sanitized
def _shell_function_defined_before(text: str, name: str, line_no: int) -> bool:
    pattern = re.compile(
        r"^\s*(?:function\s+)?" + re.escape(name) + r"\s*(?:\(\s*\))?\s*\{"
    )
    lines = text.splitlines()
    for index, line in enumerate(lines[: line_no - 1]):
        if not pattern.match(line) or line.rstrip().split("{", 1)[-1].strip():
            continue
        if any(
            re.fullmatch(r"\s*}\s*;?\s*", body_line)
            for body_line in lines[index + 1 : line_no - 1]
        ):
            return True
    return False
def _shell_interpreter_script_target(segment: list, executable: str, aliases: dict) -> set:
    """Return statically resolved script paths, or an empty set if dynamic.
    (Trả đường dẫn script phân giải tĩnh; trả tập rỗng nếu đường dẫn động.)"""
    interpreter_index = next(
        (
            index
            for index, token in enumerate(segment)
            if token.replace("\\", "/").rsplit("/", 1)[-1].lower() == executable
        ),
        None,
    )
    if interpreter_index is None:
        return set()
    args = segment[interpreter_index + 1 :]
    if executable in {"powershell", "pwsh"}:
        for index, argument in enumerate(args):
            if argument.lower() in {"-command", "-c", "-encodedcommand", "-enc"}:
                return set()
            if argument.lower() in {"-file", "-f"}:
                if index + 1 >= len(args):
                    return set()
                args = args[index + 1 :]
                break
        else:
            return set()
    else:
        index = 0
        while index < len(args):
            argument = args[index]
            if argument in {"-c", "-s", "--command", "--stdin"} or argument.startswith("<<"):
                return set()
            if argument == "--":
                index += 1
                break
            if executable == "bash" and (
                argument == "--login"
                or re.fullmatch(r"-[A-Za-z]*l[A-Za-z]*", argument)
            ):
                return set()
            if argument in {"--rcfile", "--init-file"} or argument.startswith(
                ("--rcfile=", "--init-file=")
            ):
                # Startup hooks execute before the reviewed script. Treat them
                # as a dynamic boundary unless they receive their own scan.
                # (Hook khởi động chạy trước script đã rà; chặn nếu chưa được quét riêng.)
                return set()
            if argument in {"-o", "+o", "-O", "+O"}:
                index += 2
                continue
            if re.fullmatch(r"-[A-Za-z]+", argument) or re.fullmatch(r"\+[A-Za-z]+", argument):
                if "i" in argument:
                    return set()
                index += 1
                continue
            if argument.startswith("-"):
                return set()
            break
        args = args[index:]
    if not args:
        return set()

    argument = args[0]
    candidates = set()
    variable = re.fullmatch(r"\$\{?([A-Za-z_][A-Za-z0-9_]*)\}?(?:/(.*))?", argument)
    if variable:
        base_paths = aliases.get(variable.group(1), set())
        suffix = variable.group(2) or ""
        candidates.update(posixpath.normpath(posixpath.join(base, suffix)) for base in base_paths)
    return {
        path
        for path in candidates
        if path.endswith(SHELL_SCRIPT_SUFFIXES) and path != ".." and not path.startswith("../")
    }
def _shell_bash_shebang_is_login(shebang: str) -> bool:
    try:
        arguments = shlex.split(shebang.removeprefix("#!").strip())
    except ValueError:
        return True
    return any(
        argument in {"--login", "--interactive"}
        or re.fullmatch(r"-[A-Za-z]*[li][A-Za-z]*", argument)
        for argument in arguments[1:]
    )
def _shell_executable(
    segment: list,
    command_aliases=None,
    static_script_aliases=None,
    scanned_shell_paths=None,
):
    """Resolve a direct known-tool command after common transparent wrappers.
    (Tìm executable thuộc inventory sau wrapper shell phổ biến.)"""
    index = 0
    command_aliases = command_aliases or {}
    static_script_aliases = static_script_aliases or {}
    scanned_shell_paths = set(scanned_shell_paths or ())
    while index < len(segment):
        token = segment[index]
        lowered = token.lower()
        if lowered == "case":
            index += 1
            while index < len(segment) and segment[index].lower() != "in":
                index += 1
            index += 1
            continue
        if token == "[[":
            index += 1
            while index < len(segment) and segment[index] != "]]":
                index += 1
            index += 1
            continue
        if token.startswith("(("):
            index += 1
            while index < len(segment) and not segment[index].endswith("))"):
                index += 1
            index += 1
            continue
        if lowered in SHELL_CONTROL_WORDS:
            index += 1
            continue
        if lowered == "command" and index + 1 < len(segment) and segment[index + 1] == "-v":
            # `command -v` is a shell builtin probe, not tool execution.
            # (`command -v` là builtin dò đường dẫn, không chạy executable.)
            return None
        if token in {"(", ")", "[[", "]]"}:
            index += 1
            continue
        if SHELL_ASSIGNMENT_RE.match(token):
            # Array values are shell data, not separate command heads.
            # (Giá trị mảng là dữ liệu shell, không phải command riêng.)
            array_assignment = re.match(r"^[A-Za-z_][A-Za-z0-9_]*\+?=\(", token)
            if array_assignment:
                array_depth = token.count("(") - token.count(")")
                index += 1
                while array_depth > 0 and index < len(segment):
                    array_depth += segment[index].count("(") - segment[index].count(")")
                    index += 1
                continue
            if "$((" in token:
                arithmetic_depth = token.count("(") - token.count(")")
                index += 1
                while arithmetic_depth > 0 and index < len(segment):
                    arithmetic_depth += (
                        segment[index].count("(") - segment[index].count(")")
                    )
                    index += 1
                continue
            substitution_depth = len(re.findall(r"(?:\$|<|>)\(", token)) - token.count(")")
            index += 1
            while substitution_depth > 0 and index < len(segment):
                substitution_depth += (
                    len(re.findall(r"(?:\$|<|>)\(", segment[index]))
                    - segment[index].count(")")
                )
                index += 1
            continue
        if SHELL_REDIRECTION_RE.match(token):
            # Redirection operands are paths, not executable heads.
            # (Toán hạng redirect là đường dẫn, không phải command.)
            redirection_only = bool(
                re.fullmatch(r"(?:[0-9]+)?(?:&>>|&>|>>|<>|>&|<&|>|<)", token)
            )
            index += 1
            if redirection_only and index < len(segment):
                index += 1
            continue
        absolute_token = token.replace("\\", "/").lower()
        if lowered in SHELL_WRAPPERS or absolute_token in SHELL_ABSOLUTE_ENV_WRAPPERS:
            wrapper = "env" if absolute_token in SHELL_ABSOLUTE_ENV_WRAPPERS else lowered
            index += 1
            positional_remaining = SHELL_WRAPPER_POSITIONAL_COUNTS.get(wrapper, 0)
            while index < len(segment):
                argument = segment[index]
                if SHELL_ASSIGNMENT_RE.match(argument):
                    index += 1
                    continue
                if wrapper == "env" and re.match(r"^BASH_FUNC_[^=]*%%=", argument):
                    return "<dynamic-shell-command>"
                if argument.startswith("-"):
                    if wrapper == "env" and (
                        argument in {"-S", "--split-string"}
                        or argument.startswith(("-S", "--split-string="))
                    ):
                        # env -S reparses one argument as an executable string;
                        # the generic wrapper token walk cannot model that shell.
                        # (env -S phân tích lại chuỗi thành command; parser wrapper không mô phỏng an toàn.)
                        return "<dynamic-shell-command>"
                    index += 1
                    if (
                        argument in SHELL_WRAPPER_VALUE_OPTIONS.get(wrapper, set())
                        and index < len(segment)
                    ):
                        index += 1
                    continue
                if positional_remaining:
                    positional_remaining -= 1
                    index += 1
                    continue
                break
            continue
        # Only the command-position token determines the executable. Variables
        # and substitutions in ordinary arguments do not turn a static
        # command into a dynamic executable (e.g. `echo "$HOME"`).
        # (Chỉ token vị trí command quyết định executable; biến trong đối số
        # không biến lệnh tĩnh thành executable động.)
        if (
            token.startswith("$")
            or "$" in token
            or any(marker in token for marker in ("$(", "`", "<(", ">("))
        ):
            variable = re.match(r"\$\{?([A-Za-z_][A-Za-z0-9_]*)", token)
            if variable:
                alias = command_aliases.get(variable.group(1))
                if alias in {"shell-script-dir", "shell-script-root"}:
                    targets = _shell_script_argument_targets(token, static_script_aliases)
                    if len(targets) == 1 and targets <= scanned_shell_paths:
                        return "shell-script"
                    return "<dynamic-shell-command>"
                if alias:
                    return alias
            return "<dynamic-shell-command>"
        basename = token.replace("\\", "/").rsplit("/", 1)[-1]
        if basename == "busybox" and index + 1 < len(segment):
            applet = segment[index + 1].replace("\\", "/").rsplit("/", 1)[-1]
            if applet in SHELL_APPLETS:
                return "<dynamic-shell-command>"
        if basename in SHELL_DYNAMIC_EXECUTORS:
            if basename in {"source", "."} and index + 1 < len(segment):
                source_target = segment[index + 1]
                targets = _shell_script_argument_targets(source_target, static_script_aliases)
                if len(targets) == 1 and targets <= scanned_shell_paths:
                    return "shell-source"
            return "<dynamic-shell-command>"
        if basename in SHELL_INTERPRETERS:
            # A literal interpreter plus a script path is visible and each
            # source file is scanned separately. Inline `-c` remains dynamic.
            # Interpreter + đường dẫn script rõ ràng được kiểm kê riêng; `-c`
            # nội tuyến vẫn là command động cần chặn.
            interpreter_args = segment[index + 1:]
            if (
                not interpreter_args
                or interpreter_args[0] in {"-c", "-command", "-Command"}
                or any(argument.startswith("<<") for argument in interpreter_args)
            ):
                return "<dynamic-shell-command>"
            return basename
        if basename in TOOLS:
            return basename
        if token.lower().endswith(SHELL_SCRIPT_SUFFIXES):
            # A direct script path depends on the caller's working directory;
            # only source-anchored aliases are safe to resolve statically.
            # (Đường dẫn script trực tiếp phụ thuộc working directory; chỉ alias
            # được neo theo source mới có thể phân giải tĩnh an toàn.)
            return "<dynamic-shell-command>"
        if token.startswith(("./", "../")):
            # Relative executable paths can name extensionless local scripts.
            # (Đường dẫn tương đối có thể gọi script nội bộ không có phần mở rộng.)
            return "<dynamic-shell-command>"
        if "/" in token.replace("\\", "/"):
            normalized_path = posixpath.normpath(token.replace("\\", "/"))
            parent = posixpath.dirname(normalized_path)
            if not posixpath.isabs(normalized_path) or parent not in SHELL_TRUSTED_SYSTEM_BIN_DIRS:
                # Non-system paths may be extensionless scripts or caller-controlled
                # executables, so keep them visible even without a shell suffix.
                # (Đường dẫn ngoài thư mục binary hệ thống có thể là script không
                # đuôi hoặc executable do caller kiểm soát; phải báo động.)
                return "<dynamic-shell-command>"
        break
    return None
def _shell_dependency_lifecycle_action(executable: str, segment: list, command_aliases: dict):
    """Return a dependency-mutating action reachable from this command head.
    (Trả về thao tác thay đổi dependency gắn với command head đã tìm thấy.)"""
    if executable in {"npx", "bunx", "uvx", "pipx"}:
        return "exec" if executable in {"npx", "bunx"} else "run"
    if executable == "bounded-package-manager-selector":
        managers = set(SHELL_DEPENDENCY_MANAGER_ACTIONS)
    elif executable in SHELL_DEPENDENCY_MANAGER_ACTIONS:
        managers = {executable}
    elif executable in {"python", "python3"}:
        managers = {"pip"}
    else:
        return None

    # Use tokens already parsed as shell syntax, and recognize fixed command
    # aliases resolved by _shell_command_aliases. Never treat an arbitrary
    # argument containing a manager name as the executable.
    # (Dùng token shell đã parse và alias cố định; không coi argument bất kỳ
    # có chữ tên manager là executable.)
    for index, token in enumerate(segment):
        nested_tail = None
        normalized = token.strip("\"'`()")
        variable = normalized.removeprefix("$").strip("{}")
        manager = command_aliases.get(variable)
        if manager == "bounded-package-manager-selector":
            matched_managers = managers
        elif manager in managers:
            matched_managers = {manager}
        else:
            basename = normalized.rsplit("/", 1)[-1]
            matched_managers = {basename} if basename in managers else set()
        if not matched_managers:
            # Command substitutions can be lexed as a token prefix such as
            # `$(npm`; keep that case bounded to the known manager grammar.
            nested = re.search(
                r"(?:\$\(|`)(?:[^\s;&|]*/)?([A-Za-z0-9_.-]+)", token
            )
            if nested and nested.group(1) in managers:
                matched_managers = {nested.group(1)}
                nested_tail = token[nested.end() :]
            else:
                continue

        tail = " ".join(
            ([nested_tail] if nested_tail is not None else [])
            + segment[index + 1 :]
        )
        if executable in {"python", "python3"}:
            if not re.search(r"(?:^|\s)-m\s+pip(?:3)?(?:\s|$)", tail):
                continue
            tail = re.split(r"(?:^|\s)-m\s+pip(?:3)?(?:\s|$)", tail, maxsplit=1)[-1]

        for candidate in matched_managers:
            actions = SHELL_DEPENDENCY_MANAGER_ACTIONS[candidate]
            # Matching any command verb after the executable intentionally
            # fails closed for global options with separate values (for
            # example `npm --prefix dir install`). Exact source fingerprints
            # below prevent changed or newly added invocations from inheriting
            # a review.
            # (Tìm verb sau executable để không lọt global option có value;
            # fingerprint source chính xác ngăn lệnh mới thừa kế review.)
            for action in sorted(actions, key=len, reverse=True):
                if re.search(
                    r"(?<![A-Za-z0-9_-])" + re.escape(action) + r"(?![A-Za-z0-9_-])",
                    tail,
                ):
                    return action
    return None

__all__ = ['_shell_environment_unset_before', '_shell_function_defined_before', '_shell_interpreter_script_target', '_shell_bash_shebang_is_login', '_shell_executable', '_shell_dependency_lifecycle_action']
