# wheel_record entry tests / test entry wheel_record

These cases cover the stable public verifier's lock-digest preflight before ZIP parsing and preserve the protocol-internal CSV facade's `&'static str` error contract while typed parse errors stay local to the CSV owner. Full authenticated materialization behavior stays with `runtime_verification/test/`.

Các case tại đây kiểm tra preflight digest trong lock của verifier công khai trước khi parse ZIP và giữ contract lỗi `&'static str` của facade CSV nội bộ, còn lỗi parse typed nằm trong owner CSV. Hành vi materialization xác thực đầy đủ nằm tại `runtime_verification/test/`.
