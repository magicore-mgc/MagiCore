//! Regression: `mgc exec` children observe the caller environment.
//! Hồi quy: tiến trình con của `mgc exec` thấy được environment của caller.

/// Variable the child prints back to the test.
/// Biến mà child in ra cho test.
const PROBE_ENV_KEY: &str = "MGC_TEST_EXEC_ENV_PROBE";
const PROBE_ENV_VALUE: &str = "inherited-by-mgc-exec";

/// Serializes process-wide environment and cwd writes across exec tests.
/// Tuần tự hóa ghi environment và cwd toàn cục process giữa các test exec.
static PROCESS_STATE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(unix)]
#[test]
fn exec_child_inherits_caller_environment() {
    use super::run;

    // Both `exec` and `ProjectContext` read the process cwd and the process
    // environment, so this test owns those two globals while it runs.
    // (`exec` và `ProjectContext` đều đọc cwd và environment của process, nên
    // test này giữ hai global đó trong suốt thời gian chạy.)
    let _guard = PROCESS_STATE_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let dir = tempfile::tempdir().expect("temporary project");
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"exec-probe\"\necosystem = \"web\"\n",
    )
    .expect("write mgc.toml");
    let observed = dir.path().join("env-probe.txt");
    let previous_cwd = std::env::current_dir().expect("current directory");
    std::env::set_current_dir(dir.path()).expect("enter temporary project");

    // SAFETY: the test process mutates its own environment under the test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(PROBE_ENV_KEY, PROBE_ENV_VALUE);
    }

    // `node -e` writes the observed value itself: the CLI's stdout is not
    // capturable from inside the process, and the file survives any child exit.
    // (`node -e` tự ghi giá trị quan sát: stdout của CLI không bắt được từ trong
    // process, còn file thì còn lại sau khi child thoát.)
    let script = format!(
        "require('fs').writeFileSync(process.argv[1], String(process.env.{PROBE_ENV_KEY}))"
    );
    let result = run(
        None,
        "node".to_string(),
        vec![
            "-e".to_string(),
            script,
            observed.to_string_lossy().into_owned(),
        ],
        None,
    );

    // SAFETY: the test process mutates its own environment under the test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var(PROBE_ENV_KEY);
    }
    std::env::set_current_dir(&previous_cwd).expect("restore working directory");

    result.expect("`mgc exec node` must run the allowlisted tool");
    assert_eq!(
        std::fs::read_to_string(&observed).expect("child writes its observation"),
        PROBE_ENV_VALUE,
        "PORT/NODE_ENV-style caller variables must reach `mgc exec` children"
    );
}
