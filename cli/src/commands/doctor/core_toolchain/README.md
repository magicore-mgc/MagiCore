# Core toolchain mapping

This owner maps a detected project core to the toolchain executables that core needs, so `mgc doctor` becomes a real readiness gate instead of a fixed six-tool inbox. `CORE_REQUIRED_TOOLCHAINS` is a named, closed const table covering all nine cores in canonical `mgc_types::Ecosystem` spelling (`clo`, not `cloud`): `web`→node; `lib`→rustc/cargo/python3/node/go/java (a superset, because the language is unknown until manifests are read); `ai`→python3; `app`→flutter/gradle/swift; `game`→cargo/godot; `iot`→cargo/pio/west; `clo`→terraform/node; `cicd`→git/node; `hardware`→none.

`requirements_for_core` is pure and read-only. It normalizes a `cloud` alias through the workspace alias table (`Ecosystem::canonical_core_name`) so the lookup stays single-tabled, then resolves each binary. A tool already in the general `TOOLCHAIN_PROBES` row reuses its exact fixed argv; a core-only tool (terraform, flutter, godot, pio, west, gradle, swift, java, go) uses the default `--version` argument. An unknown core returns an empty list.

The mapping is data, not execution: this owner never spawns anything and never writes. The probing itself lives in the `toolchain` owner.

# Ánh xạ toolchain theo core

Owner này ánh xạ core của project đã nhận diện sang các executable toolchain mà core đó cần, để `mgc doctor` trở thành cổng sẵn sàng thật thay vì hộp sáu tool cố định. `CORE_REQUIRED_TOOLCHAINS` là bảng const đóng, có tên, phủ đủ chín core theo cách viết chuẩn của `mgc_types::Ecosystem` (`clo`, không phải `cloud`): `web`→node; `lib`→rustc/cargo/python3/node/go/java (superset, vì chưa biết ngôn ngữ đến khi đọc manifest); `ai`→python3; `app`→flutter/gradle/swift; `game`→cargo/godot; `iot`→cargo/pio/west; `clo`→terraform/node; `cicd`→git/node; `hardware`→không.

`requirements_for_core` thuần khiết và chỉ-đọc. Nó chuẩn hoá alias `cloud` qua bảng alias workspace (`Ecosystem::canonical_core_name`) để tra cứu chỉ có MỘT bảng, rồi giải từng binary. Tool đã có trong hàng `TOOLCHAIN_PROBES` chung dùng lại argv cố định; tool chỉ thuộc core (terraform, flutter, godot, pio, west, gradle, swift, java, go) dùng đối số mặc định `--version`. Core lạ trả danh sách rỗng.

Ánh xạ là dữ liệu, không phải thực thi: owner này không spawn gì và không ghi gì. Việc probe nằm ở owner `toolchain`.
