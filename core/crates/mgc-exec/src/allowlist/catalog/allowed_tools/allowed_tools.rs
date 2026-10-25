/// Tools được phép passthrough — allowlist bất biến (00-index §5.1).
/// Mỗi core khai báo subset; thêm tool phải review + ghi lý do.
pub const ALLOWED_TOOLS: &[&str] = &[
    "python3",
    // Android Debug Bridge is restricted to the dedicated DeviceControl scope.
    // ADB chỉ được dùng trong scope DeviceControl riêng, không phải scope dev chung.
    "adb",
    "pytest", // AI test runner
    // TypeScript compiler (lib/ts + web test lanes, P0 finding
    // 2026-09-12): `tsc --noEmit` is the scaffold's own typecheck test
    // script; read-only (no artifact mutation), same family as
    // cargo/pytest. web dev/build lanes already run vite/next via
    // node_modules/.bin; tsc enters the allowlist so the lib/ts
    // scaffold test lane can run honestly.
    // TypeScript compiler (lane test lib/ts + web, P0 finding
    // 2026-09-12): `tsc --noEmit` là script test typecheck của scaffold;
    // chỉ đọc (không đổi artifact), cùng họ với cargo/pytest. Lane
    // web đã chạy vite/next qua node_modules/.bin; tsc vào allowlist
    // để lane test scaffold lib/ts chạy trung thực được.
    "tsc",
    // tsx executes project-local TypeScript lifecycle scripts after MGC installs dependencies.
    // tsx chạy script TypeScript cục bộ sau khi MagiCore cài dependency.
    "tsx",
    // vue-tsc: the vue scaffold's typecheck test script (same read-only
    // family as tsc — P0 finding 2026-09-12).
    // vue-tsc: script test typecheck của scaffold vue (cùng họ chỉ-đọc
    // với tsc — P0 finding 2026-09-12).
    "vue-tsc",
    // vite: the web scaffold's dev/build/preview entry (P0 finding #6,
    // 2026-09-12). Project scripts run under the user's permission per
    // the threat model (Test/Build/Dev scope: project-local scripts
    // allowed); `run`/`dev` resolve vite from node_modules/.bin FIRST
    // (the run.rs PATH insert), so the spawned program is the
    // project's own dependency, not a PATH implant.
    // vite: entry dev/build/preview của scaffold web (P0 finding #6).
    // Theo threat model, script project chạy dưới quyền user (scope
    // Test/Build/Dev: script local được phép); `run`/`dev` resolve vite
    // từ node_modules/.bin TRƯỚC (run.rs chèn đầu PATH), nên program
    // được spawn là dependency của chính project, không phải file lạ
    // xâm nhập từ PATH.
    "vite",
    // The Angular CLI: `ng build`, `ng test` and `ng serve` are the
    // framework's own entry points, resolved from `node_modules/.bin` first.
    // Angular CLI: `ng build`, `ng test`, `ng serve` là entry point của chính
    // framework, resolve từ `node_modules/.bin` trước.
    "ng",
    // Framework test runners the scaffold installs and `mgc test` invokes.
    // Runner test do scaffold cài và `mgc test` gọi.
    "vitest",
    "go",
    "dart",
    "gradle",
    "mvn",
    // Language runtimes and compilers — ngôn ngữ chạy và compiler native.
    // These execute project code; dependency-manager verbs remain guarded below.
    // Chúng chạy code của project; verb quản lý dependency vẫn bị policy chặn.
    "bash",
    "clang",
    "clang++",
    "dotnet",
    "gcc",
    "g++",
    "java",
    // Java compilation executes project processors; restrict it to project scopes.
    // Compiler Java có thể chạy processor của project, không dùng cho install hook.
    "javac",
    "kotlinc",
    "php",
    // NOTE: `composer` is deliberately ABSENT here — it is a PHP package
    // manager and lives in FORBIDDEN_TOOLS below. (An older revision
    // listed it in both tables; the forbidden check runs first so it was
    // never actually allowed, but the duplicate lied about the policy.)
    // (`composer` cố ý VẮNG ở đây — nó là package manager PHP, nằm ở
    // FORBIDDEN bên dưới.)
    "node",
    "swift",
    "cargo",
    "zig",
    // `rustc -vV` is a read-only host-target probe; other direct calls are rejected below.
    // `rustc -vV` chỉ đọc host target; các lời gọi trực tiếp khác bị chặn bên dưới.
    "rustc",
    "espflash",
    "west",
    "pio",
    "platformio",
    "terraform",
    "tofu",
    "cdk",
    "pulumi",
    "aws",
    "wrangler",
    "gcloud",
    "gh",
    "git",
    "docker",
    "godot",
    "flutter",
    "kotlinc",
    "python",
    "unity",
    "upm",
    "xcodebuild",
    // xcrun simctl is the iOS counterpart of adb: simulator/device
    // control only (P0 delegation finding — direct Command::new("xcrun")
    // in cli/src/commands/core/dev/app/ bypassed the executor).
    // Restricted to DeviceControl scope like adb.
    // xcrun simctl là đối tác iOS của adb: chỉ điều khiển
    // simulator/thiết bị (P0 finding — Command::new("xcrun") trực tiếp
    // trong dev/app.rs đã bypass executor). Giới hạn scope
    // DeviceControl như adb.
    "xcrun",
    "echo", // Test tool: prove validator runs before allowlist check
    // Security scanners (audit framework, Tech Lead P1 2026-09-09):
    // official ecosystem scanners — read-only advisory lookups, no
    // package mutation, pinned versions in CI (security.yml).
    // Scanner bảo mật (audit framework): scanner chính thức của từng
    // ecosystem — chỉ tra advisory đọc-đọc, không đổi package, CI ghim
    // version (security.yml).
    "pip-audit",   // Official PyPA Python vulnerability scanner
    "govulncheck", // Official Go vulnerability scanner (golang.org/x/vuln)
    // ── Framework initializers ───────────────────────────────────────────
    // A scaffolding program the framework team publishes. Each one is probed
    // with `--version` first, then run with a fixed argv whose only variable
    // part is the project directory. They run in `FrameworkInitializer` scope
    // only, never in Install/Test/Build/Dev.
    // Chương trình scaffold do framework team phát hành. Mỗi cái được probe
    // `--version` trước, rồi chạy với argv cố định, chỉ thư mục project là biến
    // đổi. Chúng chỉ chạy trong scope `FrameworkInitializer`.
    "django-admin",
    "godot",
    "pio",
    "west",
    "terraform",
    "cdk",
    "pulumi",
];
