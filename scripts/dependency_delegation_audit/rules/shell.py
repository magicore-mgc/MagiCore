"""Shell implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
import re

SHELL_SCAN_ROOTS = ["."]

SHELL_WRAPPERS = {
    "command", "env", "exec", "nice", "nohup", "sudo", "time", "timeout",
}

SHELL_ABSOLUTE_ENV_WRAPPERS = {"/usr/bin/env", "/bin/env"}

SHELL_REDIRECTION_RE = re.compile(r"^(?:[0-9]+)?(?:&>>|&>|>>|<>|>&|<&|>|<)")

SHELL_DYNAMIC_EXECUTORS = {".", "alias", "builtin", "eval", "source", "trap"}

SHELL_INTERPRETERS = {
    "ash", "bash", "csh", "dash", "fish", "ksh", "mksh", "nu", "oksh",
    "powershell", "pwsh", "sh", "tcsh", "zsh",
}

SHELL_APPLETS = {"ash", "bash", "sh"}

SHELL_SCRIPT_SUFFIXES = (".sh", ".bash", ".zsh", ".ps1")

SHELL_TRUSTED_SYSTEM_BIN_DIRS = {"/bin", "/sbin", "/usr/bin", "/usr/sbin"}

SHELL_WRAPPER_VALUE_OPTIONS = {
    "env": {"-u", "--unset", "-C", "--chdir", "-S", "--split-string"},
    "nice": {"-n", "--adjustment"},
    "sudo": {
        "-u", "--user", "-g", "--group", "-C", "--chdir", "-h", "--host",
        "-p", "--prompt", "-R", "--chroot", "-T", "--command-timeout",
    },
    "time": {"-o", "--output", "-f", "--format"},
    "timeout": {"-k", "--kill-after", "-s", "--signal"},
}

SHELL_WRAPPER_POSITIONAL_COUNTS = {"timeout": 1}

SHELL_CONTROL_WORDS = {
    "!", "{", "}", "case", "do", "done", "elif", "else", "esac",
    "fi", "for", "if", "in", "select", "then", "until", "while",
}

SHELL_ASSIGNMENT_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*(?:\[[^]]+\])?\+?=")

SHELL_ARGV_DISPATCH_REVIEWS = {
    "cli/tests/scripts/acceptance_phase2.sh": {
        'if "$@"; then',
    },
    "cli/tests/scripts/all_core_scaffold_stress.sh": {
        'if "$@" >/dev/null 2>&1; then',
        'output=$("$@" 2>&1) || status=$?',
    },
    "cli/tests/scripts/cli_syntax_stress.sh": {
        'if "$@" >/dev/null 2>&1; then',
        'output=$("$@" 2>&1 || true)',
    },
    "scripts/test/test-cli.sh": {
        'output=$("$@" 2>&1)',
    },
}

SHELL_EXPLICIT_DYNAMIC_REVIEWS = {
    "scripts/test/smoke-test.sh": {
        'if ! version_output=$("$MGC_PATH" --version 2>&1); then',
        'if ! help_output=$("$MGC_PATH" --help 2>&1); then',
        'if version_cmd=$("$MGC_PATH" version 2>&1); then',
    },
}

# Exact-source fingerprints narrowly review fixed dynamic command sites in CI
# fixtures and Flutter SDK bootstrap calls.
# Fingerprint source chính xác chỉ duyệt hẹp command động cố định trong fixture CI
# và lệnh bootstrap SDK Flutter.
SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS = {
    "cli/tests/scripts/runtime_full_e2e.sh": {
        "f58a70859c54a3c4fcd092f83dffef61ffe61f1e2f4a358c094b203257afc5d2": (
            "Invokes the selected executable MGC binary with fixed list args in an isolated directory to verify the no-project diagnostic.",
        ),
        "2717131da574245db1d41fd4c0b509fee1f8aa63ea9b47d7eeb618291b3cb6d6": (
            "Invokes the selected executable MGC binary with fixed install args in an isolated compatibility fixture.",
        ),
        "c5aeeadebe3afe6b38651a7338cb731e204f4847645ffdbcb77ffcdf9b9a7f72": (
            "Invokes the selected executable MGC binary with fixed native dev args to verify the Bun/Deno policy gate in an isolated fixture.",
        ),
        "efeea9421eb17f8500c605f932f49e40b1d7a3712e7646c2e0a14387e98dd050": (
            "Invokes the selected executable MGC binary with only literal Bun/Deno compat values and fixed dev args; no shell re-evaluation occurs.",
        ),
        "86f3b5f00751343b023fa0d858c434b593a9bc49596232a6d1000a3130232bb7": (
            "Invokes the selected executable MGC binary with fixed install args in the isolated native Node fixture.",
        ),
        "9277070496353c54c6229e97ae2f27c74ee4f2bb0e55a4be79291a059d0518f8": (
            "Invokes the selected executable MGC binary with fixed native dev args in the isolated Node audit-log fixture.",
        ),
        "61d09974357c4a0182ec1faf890a293fd8010a27a4d22beb1931e3b4774d93d1": (
            "Invokes the selected executable MGC binary with fixed install args in the isolated short-lived-descendant churn fixture.",
        ),
        "f4190a49972f376d42148f054064745781c7968afa5ce73e9dce8d974903b5f9": (
            "Invokes the selected executable MGC binary with fixed dev args to exercise the real process scanner under short-lived Node child churn.",
        ),
        "adfd095ba878f353713e20b0153acd7e0bb04c780c0c1fcc02acdfbd1e2e146a": (
            "Invokes the selected executable MGC binary with fixed install args in the isolated process-guard fixture.",
        ),
        "e23df9f3778da11ccfd1bfbdcbb06d22860a13c9b2584d47855d26871d9ebdfa": (
            "Runs the test-created inert fake npm executable with only its fixed probe argument before the MGC process-guard E2E.",
        ),
        "31a73628b48c608a49424dbbfa63d117dd265f4b6367ff48c2bf7181c6802036": (
            "Invokes the selected executable MGC binary with fixed dev args for the detached-child process-guard E2E; the child is a test-created fixture.",
        ),
        "f5f27e88f69547e715474a607c50d2106eeea730d0c1c6fd154da9924821c352": (
            "Invokes the selected executable MGC binary with fixed optimizer args in the isolated Rust optimizer fixture.",
        ),
        "8916b6b1076a142a50e3955954eaa7b47a650f2b3daf5e5df8733e9fb783b5e8": (
            "Invokes the selected executable MGC binary with fixed install args in the isolated Rust optimizer fixture.",
        ),
        "2ae7e042d229d4bcf39068bd0efc6d8979fbb9addede0c4416384f3fa9e1691e": (
            "Invokes the selected executable MGC binary with fixed build args to prove optimizer profile settings reach Cargo.",
        ),
        "315198dbd2f5aaed76da5564d1921f99b448626e9d876c4389e950c168c108fd": (
            "Invokes the selected executable MGC binary with fixed test args to prove optimizer profile settings reach the real test runner.",
        ),
    },
    "scripts/install/bootstrap_flutter_sdk.sh": {
        "4e5e71f8dd7dc89141ee3976a3c2897103f5b8c9ac9dfac4ee639b918de236b7": (
            "Resolves Pub dependencies only under Flutter SDK flutter_tools via its SDK-derived Dart executable.",
        ),
        "6ecce6da42d4f5a05d7713e920152268199a75403e0621b982cb3cb2db6e07ed": (
            "Runs the Flutter SDK version probe from the SDK root provisioned by the pinned CI setup action.",
        ),
    },
}

SHELL_SOURCE_INSPECTION_REVIEWS = {
    "scripts/audit/check-module-hygiene.sh": {
        'done < <(find "$dir" -name \'*.rs\' -print0 2>/dev/null | while IFS= read -r -d \'\' file; do',
    },
    "scripts/audit/check-no-pm.sh": {
        "BAD=$(grep -rnE '\\b(npm|npx|pnpm|yarn|bun)\\b' cli/src adapters core --include='*.rs' \\",
    },
    "scripts/maintenance/migrate_inline_tests.sh": {
        'files_with_inline_tests=$(grep -rl "#\\[cfg(test)\\]" adapters/ core/ cli/src/ 2>/dev/null | grep -E "\\.rs$" | grep -v "/test/" || true)',
    },
}

# Review only the exact smoke-test argv; this inventory does not certify a
# caller-selected executable's internal behavior. — Chỉ duyệt đúng argv smoke test;
# inventory này không chứng nhận hành vi bên trong executable do caller chọn.

# Dependency-manager mutations need a source-bound review, even when their
# command name is a visible literal. Hashes cover the parsed command segment, file,
# line, full source contents, and enclosing shell-control context. Any file edit,
# command move, or guard change invalidates the grant and requires a fresh review.
# (Lệnh thay đổi dependency phải có review gắn với source dù executable là literal.
# Hash gồm command, file, dòng, toàn bộ source và guard shell; mọi sửa file, di chuyển
# lệnh hoặc đổi guard đều cần review mới.)
SHELL_DEPENDENCY_COMMAND_REVIEWS = {
    'benchmark/scripts/quick_bench.sh': {
        "201462633aef5d3d5edc666da938c623da554fafe4c627dc9a3df55fd879aeb0",
        "abe607fc46bad7a650f07ffacb5bbe020e5ec8dc90676049574c49dff1b81715",
    },
    'benchmark/scripts/run_benchmark.sh': {
        "6a0d5c6f3c5dce334291d9fcb2373dfc18390f71d286882fee4364338959333d",
        "bdcd17b7382024d566c04ec2c1e955d46cd54a93ea5bd65ce398c20f2e11816c",
        "d546a1a9b51d39b05b8cd7c7a6f54a3f2fcf49c9372526e01343f4fa5d9c4f76",
        "f7062c9168c40738bc1e9c79eea920dd5942f9648c06634f9504b2865b1b5a1e",
    },
    'benchmark/scripts/run_benchmark_native.sh': {
        "c8c1e5a5cb4ea084107ee7c994621b9bdc13f2d4461296d54b32da76003439b8",
        "b97b60b2a16117468f90fcb92627935712bde057c34d0e0aff3ef9bd22791759",
        "bc21860550a152c91521aea5323b4509ed588fafe1ef5ca9a6e9859d278b552b",
        "2a1b1571c3bbea8a6bfab13476f7acd5302635a1d137ef0dfe218ad3740c08ad",
        "9344bc70be03b740441912a1b173befb008d9779a140c8a04e63f6bec117c0a4",
        "6b2ef7a7ac046b5c66ebca1c810b6e313cddc33e12b2e7d70de4c657da769559",
        "3a335ff8668e5325df15a35f49df500c06b3f947917f7ca13e7785ad9443c654",
        "ee9bf2627ded10df9d265e8f7caa6671078f3dfbb8737f534ee7ecdf7d727c42",
    },
    'benchmark/scripts/run_benchmark_phased.sh': {
        "8eea46f82aaed545925b7bac1e4e54ccb3a3e4f04fdcbdfce9bb809f64b4e82e",
        "8cfe1f0d81d033526dafa382d9f3d0ea3d9e1d4ec61116ff661faa3a4ef48b3d",
        "d74e9836ab1db51c0a545019fa480fb6ff799e912818df9bdffae016129d7449",
        "370219f80b9639a14ca668dd4f3348fe2dac43506e9c3873c3028107d415a787",
        "cafdd833df51b4b3e82ae1b74847d379a615a261853c8a66369a3f62287dbb47",
        "c90409c5675bf01f11ceb819d023acd241d81143bf626ce18d57858a7f72d4b9",
        "088bb8e469a7e7743ff21353cb930eba52598f17aff8d30c5b23933b5dba976a",
        "94ab6beb2eb340b6e48066ad6891dde8c93eb9cbc69f3e3b403de2cf19afec61",
    },
    'scripts/bench/bench-ci.sh': {
        "13948de58bddac2afa0af9befdd261b246b4a8b4643b90f5c4a9bdf0c826e444",
    },
    'scripts/bench/bench.sh': {
        "1cfbefa5055366f84dff70489ec88f1a8c473e493dee54279e92478ce903a545",
        "52237beeac30d475828aa2aa9055458e7105224fda0d2fc1bd5d7bf261e5d946",
        "62454aff28beaeb407605bdaa4c82194a9753a014d58e9cef744c1630f639dac",
        "659c8c9696c7f3193ea9baeadcf2535dfa5daf81a15e0b43aa4b3818eeddb578",
        "79a618976fcda9b4905ec7ec5999c278754883603e159d03f4f6f2144527c72e",
        "b4284a45ad4b387dae366ef27c812e0efa543f0cbc122a1871a6c51e9602fdd7",
        "bf4c722e41cbf3e422356289d5bcde1bb143b45565eea4666d565a7cf07f514f",
        "c148f7537178d65701b4c1618974617d3df7be6ad1e43599953b121ab7d263f7",
        "e1ef18349181fbdc173d25f2a4b6541886cfe398b7407475fa5e0ef73f90bb14",
        "ee922d9bd0b860eb6de00bd1e16bbd5ae434cd69e7fb1f8df2fecfc786004d45",
    },
    'scripts/bench/build.sh': {
        "5736e0fd020d15527e595e3212f5cc46433639d47f9186f9a3e0c5acdfa8f30b",
        "7d8bde068e755ad195c71b3015e597928d1a24aafbe0719c734e527f251ec4b7",
        "f7c2023d4122a08269d4ba44a561b3d7700a9fc4c190e30877d3c14e33259393",
    },
    'scripts/install/install.sh': {
        "0dead42400b503bb7a862db51f02875df1ebe6a3edec7bb1264673c1dbb93046",
    },
    'scripts/release/release.sh': {
        "1571862ee6a05a49710cbe4a22520f874a74266c2015b3ff6e105152fab993bd",
        "c880fcc18ff0231e67e72eba9702b29a72074d64a9009543f2863ae8d8dcd93d",
    },
    'cli/tests/scripts/competitive_benchmark_impl.sh': {
        "629ce7020efb842bd715a9ab592f2203ee4755659dde73f8e26c1d9ca6e8a005",
    },
    'scripts/maintenance/gen-real-lockfile-fixtures.sh': {
        "9d65b723b5e30c1a6cc5a148e4f508eb9334e285b606687932b79dd7551f084f",
        "b747a9713bc0c6ccaa854c9ca06a5e1f20dd34453ab6d38fb3b6176c953f6afc",
    },
    'scripts/release/publish.sh': {
        "63a6ab0b735d38df5e2f8a7db33d47e297e8a7fef9f856df9c350209bec50319",
        "e6131deb7a325e3db47c41497d69992822a1aecd72b1e6d07c27ccb1dc782cc5",
    },
}

# Whole-source, line-scoped reviews for dynamic commands in release fixtures.
# A source edit invalidates every review for that file; only listed call lines
# can be cleared. Hash nguồn đầy đủ; chỉ các dòng được liệt kê mới được duyệt.
SHELL_SOURCE_DYNAMIC_COMMAND_REVIEWS = {
    "scripts/release/binary_e2e/web/create_install_test_build_audit.sh": {
        "source_sha256": "661dde6fbb84b22abfe39f7eb680e7577d304b7d9da662a51cbe72d14a7bc464",
        "lines": {
            6: "create mode: runs the freshly built mgc release binary with a literal create-web template invocation in an isolated fixture directory.",
            20: "install mode: literal mgc install against the generated fixture project.",
            28: "test mode: literal mgc test against the generated fixture project.",
            29: "build mode: literal mgc build against the generated fixture project.",
            39: "audit mode: literal mgc audit against the generated fixture project.",
        },
    },
    "scripts/release/binary_e2e/web/run_script_probe.sh": {
        "source_sha256": "f1a6c25c9c83f6f0eeb045efde47f2f11d8b33571ddb99db4ecaaa4b5a5e5e06",
        "lines": {
            7: "sources the sibling probe_process.sh helper from the same fixed script directory.",
            28: "runs the literal mgc run subcommand with the fixture script name in a background job.",
        },
    },
    "scripts/ci/docker/app/build_flutter_container.sh": {
        "source_sha256": "e1f9dd97045f8e00366f088c87d5aa5ffadd7ed370fc6bb565ce04348d441da5",
        "lines": {18: "Runs the selected MGC release binary with fixed install arguments inside a disposable Docker fixture and isolated data/cache directories."},
    },
    "scripts/ci/docker/common/extract_cli_binary.sh": {
        "source_sha256": "8d60134901516f3552a664acdd1c71ec9f5ae67e0eec9cabd0728c626b00965e",
        "lines": {9: "Runs the extracted release artifact only for its version probe before the smoke workflow uses it."},
    },
    "scripts/ci/stress/toolchains/godot/install_unix.sh": {
        "source_sha256": "0264be01fa68402424cf5efbb16978d802db46642b0d2b8161e59167c86937c5",
        "lines": {47: "Version-probes the Godot executable installed from the checksum-verified official archive."},
    },
    "scripts/ci/stress/toolchains/kotlin/install_unix.sh": {
        "source_sha256": "f2c3b84be149632e5429fddb5be88ff18cee8dd0854ef0c89201854cf63747f8",
        "lines": {28: "Version-probes the Kotlin compiler installed from the checksum-verified official archive."},
    },
    "scripts/ci/stress/toolchains/php/install_macos.sh": {
        "source_sha256": "1dc8f23b447a992ba63ada5c50b190088cb4c8ce6ee10fc9deb487c8b0f5db5a",
        "lines": {6: "Version-probes the PHP executable resolved from the macOS toolchain installation."},
    },
    "scripts/ci/stress/toolchains/terraform/install_unix.sh": {
        "source_sha256": "212315ad887857c138b9aeb0b19da7f19b385ef5b36d05e01c2ab84ff79633fa",
        "lines": {40: "Version-probes Terraform after validating the downloaded archive checksum."},
    },
    "scripts/release/artifact_smoke/web_install.sh": {
        "source_sha256": "4935ddb48563f744beaeda0d838c03ccb4bd5cbee914f7f070e369b78ac2b25c",
        "lines": {12: "Runs the release artifact selected by the smoke matrix with the fixed web-install command."},
    },
    "scripts/release/binary_e2e/archive/install_clean_prefix.sh": {
        "source_sha256": "42a2d4f8b1dca3fb6bc01fcef9886b78cef311fd3cbf3ec4ffe7ada24530bc30",
        "lines": {20: "Version-probes the Windows release binary installed into the isolated clean-prefix fixture.", 25: "Version-probes the Unix release binary installed into the isolated clean-prefix fixture."},
    },
    "scripts/release/binary_e2e/lifecycle/ai_python.sh": {
        "source_sha256": "99ca0b99bad314d9f97150fb930c15fa9a966732c34a7462d4e6774ee96cf5bd",
        "lines": {8: "Exercises the selected release binary with fixed AI/Python project-generation arguments in a temporary fixture.", 13: "Adds fixed, version-pinned Python dependencies through the release binary in the temporary fixture.", 14: "Adds fixed, version-pinned Python dependencies through the release binary in the temporary fixture.", 15: "Adds fixed, version-pinned Python dependencies through the release binary in the temporary fixture.", 16: "Adds fixed, version-pinned Python dependencies through the release binary in the temporary fixture.", 17: "Runs the release binary's fixed install subcommand in the temporary fixture.", 26: "Runs the release binary's fixed test subcommand in the generated fixture.", 27: "Runs the release binary's fixed build subcommand in the generated fixture.", 30: "Runs the release binary's fixed audit subcommand in the generated fixture."},
    },
    "scripts/release/binary_e2e/lifecycle/app_flutter.sh": {
        "source_sha256": "325599bf0dbdf3bc0cc4ec834a966dadeaefd90711cd6da80b7dadeaf61ca689",
        "lines": {8: "Exercises the selected release binary with fixed Flutter project-generation arguments in a temporary fixture.", 13: "Adds the fixed Flutter fixture dependency through the release binary.", 14: "Runs the release binary's fixed install subcommand in the temporary fixture.", 17: "Runs the release binary's fixed test subcommand in the generated fixture.", 20: "Runs the release binary's fixed build subcommand in the generated fixture."},
    },
    "scripts/release/binary_e2e/lifecycle/app_swift.sh": {
        "source_sha256": "dcfa9280a68a10fc3fff221899e2472363679c88be8a491ea693bacb6488d2fc",
        "lines": {9: "Exercises the selected release binary with fixed Swift project-generation arguments in a temporary fixture.", 11: "Runs the release binary's fixed install subcommand in the temporary fixture.", 14: "Runs the release binary's fixed test subcommand in the generated fixture.", 16: "Runs the release binary's fixed build subcommand in the generated fixture."},
    },
    "scripts/release/binary_e2e/lifecycle/install_summary.sh": {
        "source_sha256": "1097d22ec0f8ebeac29e3a804ab729b88bd87138d857175af08f5a4ead9761f3",
        "lines": {8: "Exercises the selected release binary with fixed Rust library project-generation arguments in a temporary fixture.", 10: "Runs the release binary's fixed install subcommand while capturing its summary output."},
    },
    "scripts/release/binary_e2e/lifecycle/lib_rust.sh": {
        "source_sha256": "802dff7cb757eaabc201ed056e460b4febccf2286f17b0776ed300e7f2e320e1",
        "lines": {8: "Exercises the selected release binary with fixed Rust library project-generation arguments in a temporary fixture.", 10: "Runs the release binary's fixed install subcommand in the temporary fixture.", 11: "Runs the release binary's fixed test subcommand in the generated fixture.", 12: "Runs the release binary's fixed build subcommand in the generated fixture.", 15: "Runs the release binary's fixed audit subcommand in the generated fixture."},
    },
    "scripts/release/binary_e2e/lifecycle/lockfile_import.sh": {
        "source_sha256": "84acf27ad37de37ccd430aa750cc3da06cd8455241f55db3bc39f005a8b10524",
        "lines": {16: "Imports the test-created lockfile through the selected release binary in an isolated fixture."},
    },
    "scripts/release/binary_e2e/lifecycle/warm_reinstall.sh": {
        "source_sha256": "b598b053be1c772054411f18fa7585615ec48e36056b8e3c64541738eaa58d79",
        "lines": {8: "Exercises the selected release binary with fixed Rust library project-generation arguments in a temporary fixture.", 10: "Runs the release binary's fixed install subcommand in the temporary fixture.", 14: "Runs the release binary's fixed reinstall subcommand in the temporary fixture."},
    },
    "scripts/release/binary_e2e/web/cache_reinstall.sh": {
        "source_sha256": "28fefb4a5137934ca6cc6196de7d3ec6725b85bbbf185bc3b984e4a445180091",
        "lines": {27: "Runs the release binary's fixed install subcommand in the cache-reuse fixture.", 46: "Runs the release binary's fixed offline install subcommand after warming the cache."},
    },
    "scripts/release/binary_e2e/web/create_install_test_build_audit.sh": {
        "source_sha256": "661dde6fbb84b22abfe39f7eb680e7577d304b7d9da662a51cbe72d14a7bc464",
        "lines": {6: "Captures fixed web project-generation output from the selected release binary in a temporary fixture.", 20: "Runs the release binary's fixed install subcommand in the temporary fixture.", 28: "Runs the release binary's fixed test subcommand in the generated fixture.", 29: "Runs the release binary's fixed build subcommand in the generated fixture.", 39: "Runs the release binary's fixed audit subcommand in the generated fixture."},
    },
    "scripts/release/binary_e2e/web/dev_server_probe.sh": {
        "source_sha256": "d7501a600458c6df2726443c735c9aa751f6e1fa25a243b0a6ae85be53c2b0b8",
        "lines": {7: "Sources the fixed sibling process-probe helper from the repository-owned test directory.", 19: "Starts the selected release binary with fixed dev-server arguments on the isolated probe port.", 22: "Starts the selected release binary with fixed dev-server arguments on the isolated probe port."},
    },
    "scripts/release/binary_e2e/web/run_script_probe.sh": {
        "source_sha256": "f1a6c25c9c83f6f0eeb045efde47f2f11d8b33571ddb99db4ecaaa4b5a5e5e06",
        "lines": {7: "Sources the fixed sibling process-probe helper from the repository-owned test directory.", 28: "Runs the generated fixture script through the selected release binary without shell re-evaluation."},
    },
    "scripts/release/binary_e2e/web/web_lifecycle.sh": {
        "source_sha256": "d4042b82639e0f78a82960895956a9714ea6ff52a25259237aecdc18bd951ccd",
        "lines": {9: "Invokes a fixed repository-owned lifecycle test script with Bash.", 10: "Invokes a fixed repository-owned lifecycle test script with Bash.", 11: "Invokes a fixed repository-owned lifecycle test script with Bash.", 12: "Invokes a fixed repository-owned lifecycle test script with Bash."},
    },
    "scripts/release/prepublish/verify_archive_set.sh": {
        "source_sha256": "6d8c3b914359c0e1adc04fee77e988f9a89627c8dbdbd3f0630b17cb84c97c05",
        "lines": {14: "Reads names from the fixed repository-owned artifact-contract script; its OS/architecture/variant arguments are validated against finite allowlists."},
    },
}

SHELL_DEPENDENCY_MANAGER_ACTIONS = {
    "bun": {"install", "add", "remove", "uninstall", "update", "upgrade", "link", "publish", "x"},
    "cargo": {"fetch", "install", "add", "remove", "update", "publish", "run"},
    "composer": {"install", "update", "require", "remove", "create-project"},
    "dart": {"pub get", "pub upgrade", "pub add", "pub remove"},
    "deno": {"install", "add", "remove", "upgrade", "run", "task"},
    "dotnet": {"restore", "add", "remove"},
    "flutter": {"pub get", "pub upgrade", "pub add", "pub remove"},
    "go": {"get", "install", "mod download", "mod tidy", "mod vendor", "run"},
    "gradle": {"dependencies", "install", "publish"},
    "mvn": {"dependency:resolve", "dependency:get", "install", "deploy"},
    "npm": {"install", "i", "ci", "add", "remove", "uninstall", "update", "upgrade", "dedupe", "link", "publish", "pack", "cache clean", "exec"},
    "pip": {"install", "uninstall", "download"},
    "pip3": {"install", "uninstall", "download"},
    "pio": {"pkg install", "pkg update"},
    "pnpm": {"install", "i", "add", "remove", "uninstall", "update", "upgrade", "link", "publish", "pack", "store prune", "dlx"},
    "pod": {"install", "update", "repo update"},
    "pub": {"get", "upgrade", "add", "remove"},
    "swift": {"package resolve", "package update", "package add", "package remove"},
    "uv": {"install", "sync", "add", "remove", "pip install", "pip uninstall", "run", "tool run"},
    "west": {"update"},
    "yarn": {"install", "add", "remove", "upgrade", "link", "publish", "pack", "dlx"},
}

__all__ = []
