# Install command orchestration / Điều phối install

`install.rs` owns `run`: project context, pre-install hook, monorepo fan-out and per-project orchestration. It delegates mutation and workspace discovery to their owners. Failure reporting retains the shared workspace error constructor. This source currently has no command-level behavior tests; add coverage in the later test phase.

`install.rs` sở hữu `run`: context, pre-install hook, chạy workspace và điều phối theo project. Không có test hành vi command trong bộ test hiện tại; bổ sung ở phase test sau.
