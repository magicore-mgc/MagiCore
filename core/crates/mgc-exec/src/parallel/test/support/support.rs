use crate::parallel::ParallelTask;

pub(crate) fn task(program: &str, args: &[&str]) -> ParallelTask {
    ParallelTask {
        program: program.to_string(),
        args: args
            .iter()
            .map(|argument| (*argument).to_string())
            .collect(),
    }
}

pub(crate) fn python() -> &'static str {
    "python3"
}
