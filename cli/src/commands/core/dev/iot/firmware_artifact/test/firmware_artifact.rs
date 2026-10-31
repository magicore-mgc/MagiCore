use super::*;
use std::path::PathBuf;

fn tmp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-iot-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn find_elf_locates_release_binary() {
    let dir = tmp_dir("elf");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"firmware\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let target = dir
        .join("target")
        .join("riscv32imc-unknown-none-elf")
        .join("release");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("firmware.elf"), "ELF").unwrap();
    let elf = find_elf(&dir, "riscv32imc-unknown-none-elf").unwrap();
    assert!(elf.ends_with("firmware.elf"));
}

#[test]
fn find_elf_prefers_requested_target() {
    let dir = tmp_dir("elf2");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"firmware\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    for triple in ["thumbv7em-none-eabihf", "riscv32imc-unknown-none-elf"] {
        let target = dir.join("target").join(triple).join("release");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::write(target.join("firmware.elf"), "ELF").unwrap();
    }
    let elf = find_elf(&dir, "riscv32imc-unknown-none-elf").unwrap();
    assert!(
        elf.to_string_lossy()
            .contains("riscv32imc-unknown-none-elf")
    );
}

#[test]
fn find_elf_bails_without_build() {
    let dir = tmp_dir("noelf");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"firmware\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    assert!(find_elf(&dir, "riscv32imc-unknown-none-elf").is_err());
}
