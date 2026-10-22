//! Write the current ESP32 Rust starter files.
//! Ghi các file khởi tạo ESP32 Rust hiện có.

use super::super::serialize_toml_string;
use mgc_types::MgResult;
use std::path::Path;

pub(in crate::scaffold) async fn scaffold_esp32(
    name: &str,
    board: &str,
    dir: &Path,
) -> MgResult<()> {
    let cargo_name = serialize_toml_string(name);
    let cargo = format!(
        "[package]\nname={}\nversion=\"0.1.0\"\nedition=\"2021\"\n\n[dependencies]\nesp-hal=\"1.2\"\n",
        cargo_name
    );
    std::fs::write(dir.join("Cargo.toml"), cargo)?;

    let mgc_name = serialize_toml_string(name);
    let mgc = format!(
        "name={}\nversion=\"0.1.0\"\necosystem=\"iot\"\n\n[iot]\nframework=\"esp32-rust\"\nboard=\"{}\"\n",
        mgc_name, board
    );
    std::fs::write(dir.join("mgc.toml"), mgc)?;

    std::fs::create_dir_all(dir.join("src"))?;
    std::fs::write(
        dir.join("src/main.rs"),
        "#![no_std]\n#![no_main]\n\nfn main() {}\n",
    )?;
    Ok(())
}
