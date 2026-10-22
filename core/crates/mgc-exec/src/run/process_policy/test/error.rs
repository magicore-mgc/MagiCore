//! Missing child command-line error contract.
//! Contract lỗi thiếu command line của child.

use super::*;

#[test]
fn missing_command_line_error_keeps_pid_and_safe_image_diagnostic() {
    let error = ProcessPolicyError::MissingChildCommandLine {
        image_name: "worker.exe".to_owned(),
        process_id: 17,
    };
    assert_eq!(
        error.to_string(),
        "cannot read command line for monitored child 'worker.exe' PID 17"
    );
}

#[test]
fn forbidden_script_interpreter_keeps_the_existing_diagnostic() {
    let error = ProcessPolicyError::ForbiddenScriptInterpreter {
        path: "/project/bin/install.sh".to_owned(),
        tool: "npm".to_owned(),
    };
    assert_eq!(
        error.to_string(),
        "script '/project/bin/install.sh' uses forbidden package manager 'npm' as its interpreter"
    );
}
