# Stress runtime toolchains / Toolchain runtime cho stress

Owns custom tool installation and version gates that cannot be represented by the pinned setup actions. OS-specific branches stay in separate Bash/PowerShell owners; archive checksums, versions, install paths, GitHub path/environment writes and fail-fast behavior are part of each script's contract.

Sở hữu cài tool và kiểm tra phiên bản mà setup action ghim sẵn không thay thế. Nhánh theo OS nằm ở owner Bash/PowerShell riêng; checksum, version, đường dẫn cài đặt, ghi `GITHUB_PATH`/`GITHUB_ENV` và fail-fast là contract của script.

See each language/tool folder for exact ownership. `verification/` validates the complete runtime set after all installers have run.
