//! Zephyr writer implementation.
//! Triển khai writer zephyr.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::iot::implementation) fn write(target: &Path) -> Result<()> {
    write_file(
        &target.join("west.yml"),
        "manifest:\n  version: 0.13\n  projects: []\n",
    )?;
    write_file(
        &target.join("CMakeLists.txt"),
        "cmake_minimum_required(VERSION 3.20.0)\nfind_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})\nproject(magicore_firmware)\ntarget_sources(app PRIVATE src/main.c)\n",
    )?;
    write_file(
        &target.join("prj.conf"),
        "CONFIG_PRINTK=y\nCONFIG_MAIN_STACK_SIZE=2048\n",
    )?;
    write_file(
        &target.join("src").join("main.c"),
        "#include <zephyr/kernel.h>\n#include <zephyr/sys/printk.h>\n\nint main(void)\n{\n    printk(\"MagiCore Zephyr firmware scaffold\\n\");\n    return 0;\n}\n",
    )?;
    Ok(())
}
