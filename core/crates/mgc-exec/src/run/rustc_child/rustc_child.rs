//! Rust toolchain child recognition — phân biệt rustc/rustdoc con với lệnh package manager.
//! (Chỉ chấp nhận argv biên dịch khi parent/toolchain cung cấp đủ bằng chứng tin cậy.)

use super::super::process_basename;
use std::ffi::{OsStr, OsString};
use std::path::Path;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::run::implementation) fn is_rustc_compilation_argv(argv: &[OsString]) -> bool {
    argv.first()
        .is_some_and(|program| process_basename(&program.to_string_lossy()) == "rustc")
        && is_rustc_compilation_flags(&argv[1..])
        && has_rustc_option(&argv[1..], "--emit")
        && has_rustc_option(&argv[1..], "--out-dir")
}

pub(in crate::run::implementation) fn is_rustc_wrapper_argv(argv: &[OsString]) -> bool {
    is_rustc_wrapper_argv_with_relative_compiler(argv, false)
}

pub(in crate::run::implementation) fn is_rustc_toolchain_wrapper_argv(argv: &[OsString]) -> bool {
    is_rustc_wrapper_argv_with_relative_compiler(argv, true)
}

pub(in crate::run::implementation) fn is_rustdoc_test_compilation_argv(argv: &[OsString]) -> bool {
    let Some(program) = argv.first() else {
        return false;
    };
    let program_name = process_basename(&program.to_string_lossy());
    let compiler_args = if program_name == "rustc" {
        &argv[1..]
    } else if program_name == "cargo"
        && argv.get(1).is_some_and(|compiler| {
            process_basename(&compiler.to_string_lossy()) == "rustc"
                && (Path::new(compiler).is_absolute() || compiler == OsStr::new("rustc"))
        })
    {
        &argv[2..]
    } else if argv
        .get(1)
        .is_some_and(|argument| argument.to_string_lossy().starts_with('-'))
    {
        // Cargo/rustdoc may invoke a compiler through a wrapper whose argv[0] is not
        // `rustc`; recognize it only when compiler options begin immediately and
        // the caller confirms a trusted toolchain parent. Cargo subcommands stay
        // rejected because they begin with a subcommand such as `install`.
        // (Cargo/rustdoc có thể gọi compiler qua wrapper có argv[0] không phải `rustc`;
        // chỉ nhận nếu argv kế tiếp là flag compiler và cha thuộc toolchain đáng tin.
        // Lệnh Cargo vẫn bị chặn vì argv bắt đầu bằng subcommand như `install`.)
        &argv[1..]
    } else if program.to_string_lossy().starts_with('-') {
        argv
    } else {
        return false;
    };

    !compiler_args.iter().any(|argument| argument == "--")
        && compiler_args
            .iter()
            .any(|argument| argument == "--crate-name")
        && compiler_args.iter().any(|argument| argument == "--test")
        && compiler_args.iter().any(|argument| {
            Path::new(argument)
                .extension()
                .is_some_and(|extension| extension == OsStr::new("rs"))
        })
}

fn is_rustc_wrapper_argv_with_relative_compiler(
    argv: &[OsString],
    allow_path_lookup: bool,
) -> bool {
    let Some(compiler) = argv.get(1) else {
        return false;
    };
    let compiler = Path::new(compiler);
    let compiler_path_is_trusted =
        compiler.is_absolute() || (allow_path_lookup && compiler == Path::new("rustc"));
    argv.first()
        .is_some_and(|wrapper| process_basename(&wrapper.to_string_lossy()) == "cargo")
        && compiler_path_is_trusted
        && process_basename(&compiler.to_string_lossy()) == "rustc"
        && is_rustc_compilation_flags(&argv[2..])
        && has_rustc_option(&argv[2..], "--emit")
        && has_rustc_option(&argv[2..], "--out-dir")
}

fn has_rustc_option(argv: &[OsString], option: &str) -> bool {
    argv.iter().any(|argument| {
        let argument = argument.to_string_lossy();
        argument == option
            || argument
                .strip_prefix(option)
                .is_some_and(|value| value.starts_with('='))
    })
}

pub(in crate::run::implementation) fn is_rustc_toolchain_child(
    image: &str,
    argv: &[OsString],
    parent_image: Option<&str>,
) -> bool {
    if process_basename(image) != "cargo" {
        return false;
    }
    let Some(parent_image) = parent_image.map(process_basename) else {
        return false;
    };
    let cargo_parent = parent_image == "cargo";
    let rustdoc_parent = parent_image == "rustdoc";
    if !cargo_parent && !rustdoc_parent {
        return false;
    }

    // Rustdoc compiles doctests through rustc; process snapshots can omit argv[0]
    // and report the compiler image as Cargo. Accept only the full rustc shape.
    // (Rustdoc biên dịch doctest qua rustc; snapshot có thể thiếu argv[0] và
    // ghi image compiler thành Cargo. Chỉ nhận diện khi đủ chữ ký rustc.)
    let direct_rustc_shape = is_rustc_compilation_argv(argv);
    let executable_omitted_shape = argv.first().is_some_and(|first| {
        first.to_string_lossy().starts_with('-')
            && is_rustc_compilation_flags(argv)
            && argv.iter().any(|arg| arg == "--emit")
            && argv.iter().any(|arg| arg == "--out-dir")
    });
    let rustdoc_doctest_shape =
        (cargo_parent || rustdoc_parent) && is_rustdoc_test_compilation_argv(argv);
    if direct_rustc_shape || executable_omitted_shape || rustdoc_doctest_shape {
        return true;
    }

    // Rustdoc and Cargo may pass `rustc` through PATH to their configured wrapper;
    // accept that relative operand only while the observed parent is trusted.
    // (Rustdoc và Cargo có thể truyền `rustc` qua PATH cho wrapper đã cấu hình;
    // chỉ nhận operand tương đối khi parent quan sát được thuộc toolchain.)
    is_rustc_wrapper_argv(argv) || is_rustc_toolchain_wrapper_argv(argv)
}

fn is_rustc_compilation_flags(argv: &[OsString]) -> bool {
    !argv.iter().any(|arg| arg == "--")
        && argv.iter().any(|arg| arg == "--crate-name")
        && argv.iter().any(|arg| arg == "--crate-type")
}
