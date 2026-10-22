# IoT dependency install boundary

Dispatches by IoT framework and currently refuses package installation until MagiCore owns the required resolver, lock, fetch, verification, store and materialization pipeline. It does not shell out to `pio` or `west`.

Điều phối theo framework IoT và hiện từ chối cài package cho tới khi MagiCore sở hữu resolver, lock, fetch, verify, store và materialize cần thiết. Module không gọi shell `pio` hoặc `west`.
