# Help output owner

Preserves MagiCore help text and stdout printing. `help.rs` coordinates core profile selection and section composition; `sections/` owns each output section. Public API remains `mgc_ui::help::{custom_help_text, print_custom_help}`.

No typed error owner is added: formatting is infallible and the print wrapper returns no error. This boundary does not own command policy or capability truth.

# Owner render help

Giữ việc ghép văn bản help MagiCore và in stdout. `help.rs` điều phối profile và section; `sections/` sở hữu từng phần output. Public API vẫn là `mgc_ui::help::{custom_help_text, print_custom_help}`.

Không thêm typed error owner: render không fallible và wrapper không trả lỗi. Owner này không sở hữu policy command hay sự thật capability.
