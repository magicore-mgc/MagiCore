// Output-boundary tests — kiểm tra giới hạn và thứ tự output.

#[test]
fn diagnostic_tail_keeps_only_the_last_forty_lines_in_order() {
    let input = (0..=40)
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let output = super::tail(input.as_bytes());

    assert_eq!(output.lines().count(), 40);
    assert_eq!(output.lines().next(), Some("1"));
    assert_eq!(output.lines().last(), Some("40"));
}

#[test]
fn diagnostic_tail_keeps_only_the_last_megabyte() {
    let mut input = vec![b'x'; super::MAX_CAPTURE_BYTES + 4];
    input[..4].copy_from_slice(b"drop");

    let output = super::tail(&input);

    assert_eq!(output.len(), super::MAX_CAPTURE_BYTES);
    assert!(!output.contains("drop"));
}

#[test]
fn full_capture_keeps_all_lines_within_the_byte_limit() {
    let input = (0..=40)
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let output = super::full(input.as_bytes());

    assert_eq!(output.lines().count(), 41);
    assert_eq!(output.lines().next(), Some("0"));
    assert_eq!(output.lines().last(), Some("40"));
}

#[test]
fn full_capture_keeps_only_the_last_megabyte() {
    let mut input = vec![b'x'; super::MAX_CAPTURE_BYTES + 4];
    input[..4].copy_from_slice(b"drop");

    let output = super::full(&input);

    assert_eq!(output.len(), super::MAX_CAPTURE_BYTES);
    assert!(!output.contains("drop"));
}

#[test]
fn bounded_reader_drains_past_the_cap_and_retains_the_final_window() {
    let input = (0..super::MAX_CAPTURE_BYTES + super::STREAM_READ_CHUNK_BYTES)
        .map(|byte| (byte % 251) as u8)
        .collect::<Vec<_>>();
    let expected = input[input.len() - super::MAX_CAPTURE_BYTES..].to_vec();
    let mut reader = std::io::Cursor::new(input.as_slice());

    let output = super::read_stream(&mut reader).expect("read bounded output stream");

    assert_eq!(reader.position(), input.len() as u64);
    assert_eq!(output, expected);
}

struct InterruptedOnce<R> {
    inner: R,
    interrupted: bool,
}

impl<R: std::io::Read> std::io::Read for InterruptedOnce<R> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if !self.interrupted {
            self.interrupted = true;
            return Err(std::io::ErrorKind::Interrupted.into());
        }
        self.inner.read(buffer)
    }
}

#[test]
fn bounded_reader_retries_interrupted_reads() {
    let mut reader = InterruptedOnce {
        inner: std::io::Cursor::new(b"retained output"),
        interrupted: false,
    };

    assert_eq!(
        super::read_stream(&mut reader).expect("retry interrupted output read"),
        b"retained output"
    );
}

struct FailsAfterPrefix {
    has_written_prefix: bool,
}

impl std::io::Read for FailsAfterPrefix {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if !self.has_written_prefix {
            self.has_written_prefix = true;
            buffer[..7].copy_from_slice(b"partial");
            return Ok(7);
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "pipe read denied",
        ))
    }
}

#[test]
fn bounded_reader_returns_io_failures_with_their_source() {
    use std::error::Error;

    let error = super::read_stream(FailsAfterPrefix {
        has_written_prefix: false,
    })
    .expect_err("pipe read failure must not look like successful truncated output");

    assert!(
        error
            .to_string()
            .contains("failed to read child output stream")
    );
    assert_eq!(
        Error::source(&error)
            .expect("reader I/O source is retained")
            .to_string(),
        "pipe read denied"
    );
}
