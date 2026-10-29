use super::super::ParallelTask;

#[test]
fn stores_the_program_and_arguments_without_reordering() {
    let task = ParallelTask {
        program: "python3".to_string(),
        args: vec!["-c".to_string(), "print(1)".to_string()],
    };
    assert_eq!(task.program, "python3");
    assert_eq!(task.args, vec!["-c".to_string(), "print(1)".to_string()]);
}
