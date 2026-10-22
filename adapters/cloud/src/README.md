# Cloud crate source root

`lib.rs` is the stable crate boundary: it wires the Cloud owners and re-exports their public APIs. Domain behavior belongs in the named owner folders beside it; this root does not own provider execution or framework implementations.

`lib.rs` là ranh giới crate ổn định: nối các owner Cloud và re-export API công khai. Hành vi domain thuộc các folder owner có tên bên cạnh; thư mục gốc này không sở hữu việc chạy provider hay implementation framework.
