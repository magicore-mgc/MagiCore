//! Writes the frozen-install entrypoint that hands control to MGC.
//! Ghi entrypoint cài frozen rồi chuyển điều khiển cho MGC.

use anyhow::Result;
use std::path::Path;

use crate::scaffold::container::DEFAULT_CONTAINER_ENTRYPOINT_FILE;
use crate::scaffold::processor::Scaffolder;

pub(in crate::scaffold::container) fn write_container_entrypoint(
    target: &Path,
    install_command: &str,
) -> Result<()> {
    let script = format!(
        "#!/bin/sh\n\
         # Replay the project lock into the shared MGC store.\n\
         # Cài theo lock project vào store MGC dùng chung.\n\
         set -eu\n\
         {install_command} --frozen\n\
         exec mgc \"$@\"\n"
    );
    Scaffolder::write_file(&target.join(DEFAULT_CONTAINER_ENTRYPOINT_FILE), &script)
}
