# Web native Go dev owner

Owns `go.mod` detection and selects `go run ./cmd/server` when present, otherwise `go run .`; shared endpoint variables are applied by the environment owner.

Sở hữu nhận diện `go.mod`; chọn `go run ./cmd/server` nếu có, nếu không thì `go run .`; biến endpoint dùng owner environment chung.
