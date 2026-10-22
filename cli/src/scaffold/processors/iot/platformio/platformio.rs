//! PlatformIO writer implementation.
//! Triển khai writer platformio.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::iot::implementation) fn write(
    target: &Path,
    board: &str,
) -> Result<()> {
    write_file(
        &target.join("platformio.ini"),
        &format!("[env:{board}]\nplatform = espressif32\nboard = {board}\nframework = arduino\n"),
    )?;
    write_file(
        &target.join("src").join("main.cpp"),
        "#include <Arduino.h>\n\nvoid setup() {\n}\n\nvoid loop() {\n}\n",
    )?;
    Ok(())
}
