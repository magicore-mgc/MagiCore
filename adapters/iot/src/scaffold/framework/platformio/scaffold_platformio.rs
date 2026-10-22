//! Write the current PlatformIO starter files.
//! Ghi các file khởi tạo PlatformIO hiện có.

use super::super::serialize_toml_string;
use mgc_types::MgResult;
use std::path::Path;

pub(in crate::scaffold) async fn scaffold_platformio(
    name: &str,
    board: &str,
    dir: &Path,
) -> MgResult<()> {
    let ini = format!(
        "[env:{}]\nplatform=espressif32\nboard={}\nframework=arduino\n",
        board, board
    );
    std::fs::write(dir.join("platformio.ini"), ini)?;

    let mgc_name = serialize_toml_string(name);
    let mgc = format!(
        "name={}\nversion=\"0.1.0\"\necosystem=\"iot\"\n\n[iot]\nframework=\"platformio\"\nboard=\"{}\"\n",
        mgc_name, board
    );
    std::fs::write(dir.join("mgc.toml"), mgc)?;

    std::fs::create_dir_all(dir.join("src"))?;
    std::fs::write(
        dir.join("src/main.cpp"),
        "void setup() {}\nvoid loop() {}\n",
    )?;
    Ok(())
}
