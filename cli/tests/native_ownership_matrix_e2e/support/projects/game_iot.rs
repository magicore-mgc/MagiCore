//! Game and IoT project fixtures. / Fixture dự án Game và IoT.

use crate::native_ownership_matrix_e2e::support::write;

pub(crate) fn bevy_project(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"game\"\n[game]\nengine = \"bevy\"\n",
    );
    write(
        dir,
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
    );
}

pub(crate) fn esp32_project(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"iot\"\n[iot]\nframework = \"esp32-rust\"\n",
    );
    write(
        dir,
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
    )
}

pub(crate) fn bevy_with_dep(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"game\"\n[game]\nengine = \"bevy\"\n",
    );
    write(
        dir,
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde_json = \"1.0.100\"\n",
    );
}

pub(crate) fn esp32_with_dep(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"iot\"\n[iot]\nframework = \"esp32-rust\"\n",
    );
    write(
        dir,
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde_json = \"1.0.100\"\n",
    );
}
