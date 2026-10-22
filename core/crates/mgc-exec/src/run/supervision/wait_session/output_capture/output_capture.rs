//! Reader-thread setup and deadline-aware child-output collection.
//! Khởi tạo reader thread và thu output process con theo deadline.

use crate::run::implementation::output::{ExecOutcome, StreamReadError};
use crate::run::implementation::{read_stream, supervision};
use anyhow::Result;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

type ReaderResult = Result<Vec<u8>, StreamReadError>;

pub(in crate::run::implementation::supervision::implementation::wait_session) struct OutputCapture {
    stdout_rx: Receiver<ReaderResult>,
    stderr_rx: Receiver<ReaderResult>,
    stdout_reader_expected: bool,
    stderr_reader_expected: bool,
    _stdout_reader: Option<JoinHandle<()>>,
    _stderr_reader: Option<JoinHandle<()>>,
}

impl OutputCapture {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn start(
        child: &mut std::process::Child,
    ) -> Self {
        let (stdout_tx, stdout_rx) = mpsc::channel::<ReaderResult>();
        let (stderr_tx, stderr_rx) = mpsc::channel::<ReaderResult>();
        let stdout_reader_expected = child.stdout.is_some();
        let stderr_reader_expected = child.stderr.is_some();
        let stdout_reader = child.stdout.take().map(|stream| {
            std::thread::spawn(move || publish_reader_result(stdout_tx, read_stream(stream)))
        });
        let stderr_reader = child.stderr.take().map(|stream| {
            std::thread::spawn(move || publish_reader_result(stderr_tx, read_stream(stream)))
        });

        Self {
            stdout_rx,
            stderr_rx,
            stdout_reader_expected,
            stderr_reader_expected,
            _stdout_reader: stdout_reader,
            _stderr_reader: stderr_reader,
        }
    }

    pub(in crate::run::implementation::supervision::implementation::wait_session) fn drain(
        &self,
        child: &mut std::process::Child,
        bounded: Option<Duration>,
    ) -> Result<(ExecOutcome, bool)> {
        let deadline = bounded.map(|duration| Instant::now() + duration);
        let stdout = collect(
            &self.stdout_rx,
            "stdout",
            self.stdout_reader_expected,
            deadline,
        );
        let stderr = collect(
            &self.stderr_rx,
            "stderr",
            self.stderr_reader_expected,
            deadline,
        );
        let status = child
            .wait()
            .map_err(supervision::implementation::error::SupervisionError::WaitChild);
        let (stdout, stdout_complete) = stdout?;
        let (stderr, stderr_complete) = stderr?;
        let status = status?;
        Ok((
            ExecOutcome {
                status,
                stdout,
                stderr,
            },
            stdout_complete && stderr_complete,
        ))
    }
}

fn publish_reader_result(sender: Sender<ReaderResult>, result: ReaderResult) {
    match sender.send(result) {
        Ok(()) => {}
        Err(undelivered) => {
            // A bounded timeout/cancellation return drops its receiver by design.
            // Nhánh timeout/cancel có giới hạn đóng receiver theo chủ đích.
            drop(undelivered.0);
        }
    }
}

fn collect(
    receiver: &Receiver<ReaderResult>,
    stream: &'static str,
    reader_expected: bool,
    deadline: Option<Instant>,
) -> Result<(Vec<u8>, bool)> {
    let received = match deadline {
        Some(deadline) => {
            match receiver.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
                Ok(result) => Some(result),
                Err(mpsc::RecvTimeoutError::Timeout) => return Ok((Vec::new(), false)),
                Err(mpsc::RecvTimeoutError::Disconnected) if !reader_expected => {
                    return Ok((Vec::new(), true));
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(supervision::implementation::error::SupervisionError::OutputReaderDisconnected { stream }.into());
                }
            }
        }
        None => match receiver.recv() {
            Ok(result) => Some(result),
            Err(_) if !reader_expected => return Ok((Vec::new(), true)),
            Err(_) => {
                return Err(supervision::implementation::error::SupervisionError::OutputReaderDisconnected { stream }.into());
            }
        },
    };

    match received {
        Some(Ok(bytes)) => Ok((bytes, true)),
        Some(Err(source)) => Err(
            supervision::implementation::error::SupervisionError::OutputRead { stream, source }
                .into(),
        ),
        None => Ok((Vec::new(), true)),
    }
}
