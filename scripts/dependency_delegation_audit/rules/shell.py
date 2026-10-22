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
    "scripts/test-cli.sh": {
        'output=$("$@" 2>&1)',
    },
}

SHELL_EXPLICIT_DYNAMIC_REVIEWS = {
    "scripts/smoke-test.sh": {
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
    "scripts/bootstrap_flutter_sdk.sh": {
        "5ff619429bcee3f0f09a19b24a46e57f87894e694794ba93041936169ad6b1ab": (
            "Resolves Pub dependencies only under Flutter SDK flutter_tools via its SDK-derived Dart executable.",
        ),
        "a2e80d90cbe8673a991776e9bb38d6a2f4392061acef291ff63046af2834cb3a": (
            "Runs the Flutter SDK version probe from the SDK root provisioned by the pinned CI setup action.",
        ),
    },
}

SHELL_SOURCE_INSPECTION_REVIEWS = {
    "scripts/check-module-hygiene.sh": {
        'done < <(find "$dir" -name \'*.rs\' -print0 2>/dev/null | while IFS= read -r -d \'\' file; do',
    },
    "scripts/check-no-pm.sh": {
        "BAD=$(grep -rnE '\\b(npm|npx|pnpm|yarn|bun)\\b' cli/src adapters core --include='*.rs' \\",
    },
    "scripts/migrate_inline_tests.sh": {
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
        "8a58d7f2235ca1cddb66667555ccc1fa7f1d5bfadc3c4fe9e40735a7f24af542",
        "2dd889b391bd7760a9150d2bc7f45ca5416ac318ddd389627f99e2232183528a",
        "9df0d55af68a92788722ec0afc412533f05d44f89b31bc26e4e7739e3c6512c2",
        "67ef6755e580ae6f152bc677d3e5a9626766a33a1e88845126144470fddd3bca",
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
    'scripts/bench-ci.sh': {
        "ae14ff47bf2d8510c9b31ecc367454bc63d7307692ed4c390199545bb4cecb7c",
    },
    'scripts/bench.sh': {
        "fa63073d82c5485e222d2b436433a8591e074bddbb3c8dffa64dc5e540402aea",
        "fc0833bce56a3f4ed16a3c4149287cf6062227f3399c0789306d9c1ffdc57534",
        "d30628df28d92f2d4db1805cfd18df0b9cb23d37d14c321707eea5e8777abaa9",
        "c1e734ae4538b5070af94abd269b53518ffd57feee95b161c8f881c48e79320f",
        "c90459835e2ca1b58772b32e5a44c6875c3cda352867cca7068b5e27937b3d70",
        "63a1ededee0e16c504eb2de591bcd1b57300aea998b29cedcf015de8947d17c5",
        "5f98527bdd2a6a5804e95e874e1a216078597bfaedc378ba31041d9b5563eee8",
        "291e2847f4d0c8bc71cbf4bf8510557c035a2ebbd6f44764f8d430b2f325d5b9",
        "50eded51c060dcd70a1e0bf9a4e8d89a8c7b54cf49cd570a3428458556efcc72",
        "b384b1b1245c193dfb730075a7c67fdff299a52e1c7efb1175e354d2be153465",
    },
    'scripts/build.sh': {
        "a5184092f6314a2414644a9cae6b6f031626448804c2111f91c2fdc9d581fe7c",
        "7b902a6dca9b5e4c72a457e4442a49162b523c7b86d12f863764034caa7ee29c",
        "a890c9b6e122c2a1eea74d3d271e31e8b49e38cd79ee311ba2b936c3821fb1e0",
    },
    'scripts/install.sh': {
        "c48b21609d842a372096e81333bbcacc21b4f4bc54d7a173490d6d6cf321521e",
    },
    'scripts/release.sh': {
        # Fixed `cargo run -p mgc-dist -- build-all`; review binds both argv branches.
        # Lệnh cargo cố định cho build-all; fingerprint ràng buộc cả hai nhánh argv.
        "94e77fed0f8ba8ce7e68d641d9049e6fc1e17b3a84b29ae112fc8eb37b4ad7b1",
        "670c7e7c2b1b0da6b1a333e02ca655d6826088d32088c1e81342a26d9e0fde87",
    },
    'cli/tests/scripts/competitive_benchmark_impl.sh': {
        "629ce7020efb842bd715a9ab592f2203ee4755659dde73f8e26c1d9ca6e8a005",
    },
    'scripts/gen-real-lockfile-fixtures.sh': {
        # Bounded fixture-only writers: pinned tools, temporary home/cache,
        # and lockfile-only mode with dependency lifecycle scripts disabled.
        # Lệnh writer chỉ sinh fixture: tool được ghim, home/cache tạm,
        # chỉ ghi lockfile và tắt lifecycle script của dependency.
        "04934b72569dfde146263a8666e9f309b0b38fc1deb41cd2c9d7502cdd881eca",
        "e3713998bc60be1f7156a2196feeffa1199ed547f8e8a62c23b48c21e1519790",
    },
    'scripts/publish.sh': {
        "aed22d7245b1f5d808558dd46791d5cd374616a2a59dcba708491199619b98f5",
        "de7828bae3eb8b9e31eb0e37ee2372d3690be736d59fff259220b2c832a69e29",
    },
}

# Whole-source, line-scoped reviews for dynamic commands in release fixtures.
# A source edit invalidates every review for that file; only listed call lines
# can be cleared. Hash nguồn đầy đủ; chỉ các dòng được liệt kê mới được duyệt.
SHELL_SOURCE_DYNAMIC_COMMAND_REVIEWS = {
    "scripts/ci/docker/app/build_flutter_container.sh": {
        "source_sha256": "e1f9dd97045f8e00366f088c87d5aa5ffadd7ed370fc6bb565ce04348d441da5",
        "lines": {18: "Runs the selected MGC release binary with fixed install arguments inside a disposable Docker fixture and isolated data/cache directories."},
    },
    "scripts/ci/docker/common/extract_cli_binary.sh": {
        "source_sha256": "8d60134901516f3552a664acdd1c71ec9f5ae67e0eec9cabd0728c626b00965e",
        "lines": {9: "Runs the extracted release artifact only for its version probe before the smoke workflow uses it."},
    },
    "scripts/ci/stress/toolchains/godot/install_unix.sh": {
        "source_sha256": "99a65e0743eb9870525df34201b019399c112914b900d2c922e2c5abf38db5ca",
        "lines": {47: "Version-probes the Godot executable installed from the checksum-verified official archive."},
    },
    "scripts/ci/stress/toolchains/kotlin/install_unix.sh": {
        "source_sha256": "b67ebb9d68296a3e7441103bb627f6ada30ad62575fc104e9f2edadc41d7c3fa",
        "lines": {28: "Version-probes the Kotlin compiler installed from the checksum-verified official archive."},
    },
    "scripts/ci/stress/toolchains/php/install_macos.sh": {
        "source_sha256": "38bd2161c47fd9499db47b7e68c17e5a82911d69cbaa48f8cbb07ddbf30521e1",
        "lines": {6: "Version-probes the PHP executable resolved from the macOS toolchain installation."},
    },
    "scripts/ci/stress/toolchains/terraform/install_unix.sh": {
        "source_sha256": "63dcc8a6ddaf10f06a3b0b199eb37eda25ca9de603884cd921bbcd859f38dc7c",
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
        "source_sha256": "01be97a4c6ae97225a1ea711a677acfc223fbe6ce8ec56e2fd607e3508f411cd",
        "lines": {6: "Captures fixed web project-generation output from the selected release binary in a temporary fixture.", 20: "Runs the release binary's fixed install subcommand in the temporary fixture.", 28: "Runs the release binary's fixed test subcommand in the generated fixture.", 29: "Runs the release binary's fixed build subcommand in the generated fixture.", 39: "Runs the release binary's fixed audit subcommand in the generated fixture."},
    },
    "scripts/release/binary_e2e/web/dev_server_probe.sh": {
        "source_sha256": "d7501a600458c6df2726443c735c9aa751f6e1fa25a243b0a6ae85be53c2b0b8",
        "lines": {7: "Sources the fixed sibling process-probe helper from the repository-owned test directory.", 19: "Starts the selected release binary with fixed dev-server arguments on the isolated probe port.", 22: "Starts the selected release binary with fixed dev-server arguments on the isolated probe port."},
    },
    "scripts/release/binary_e2e/web/run_script_probe.sh": {
        "source_sha256": "d76452e93b1743207849096b52d86ca8e45981679a351ab5aa6e99e645786f57",
        "lines": {7: "Sources the fixed sibling process-probe helper from the repository-owned test directory.", 28: "Runs the generated fixture script through the selected release binary without shell re-evaluation."},
    },
    "scripts/release/binary_e2e/web/web_lifecycle.sh": {
        "source_sha256": "d4042b82639e0f78a82960895956a9714ea6ff52a25259237aecdc18bd951ccd",
        "lines": {9: "Invokes a fixed repository-owned lifecycle test script with Bash.", 10: "Invokes a fixed repository-owned lifecycle test script with Bash.", 11: "Invokes a fixed repository-owned lifecycle test script with Bash.", 12: "Invokes a fixed repository-owned lifecycle test script with Bash."},
    },
    "scripts/release/prepublish/verify_archive_set.sh": {
        "source_sha256": "ff031f67a350210f1e749c00420c4551abc1b8cb68b86ad96cf21855a381e582",
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
