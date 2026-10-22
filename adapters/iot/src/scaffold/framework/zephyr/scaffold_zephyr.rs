//! Write the current Zephyr starter files.
//! Ghi các file khởi tạo Zephyr hiện có.

use super::super::serialize_toml_string;
use mgc_types::MgResult;
use std::path::Path;

pub(in crate::scaffold) async fn scaffold_zephyr(
    name: &str,
    board: &str,
    dir: &Path,
) -> MgResult<()> {
    let west = "manifest:\n  projects:\n    - name: zephyr\n      url: https://github.com/zephyrproject-rtos/zephyr\n"
        .to_string();
    std::fs::write(dir.join("west.yml"), west)?;

    let mgc_name = serialize_toml_string(name);
    let mgc = format!(
        "name={}\nversion=\"0.1.0\"\necosystem=\"iot\"\n\n[iot]\nframework=\"zephyr\"\nboard=\"{}\"\n",
        mgc_name, board
    );
    std::fs::write(dir.join("mgc.toml"), mgc)?;
    Ok(())
}
