# mgc-crypto source owners

The crate root is a public wiring facade. Cryptographic operations are grouped by responsibility; the keyring owner now contains key-pair generation, keyring API, typed policy errors and platform-aware secure storage.

Root crate chỉ là facade wiring public. Thao tác mật mã được nhóm theo trách nhiệm; owner keyring hiện chứa tạo key pair, API keyring, lỗi policy có kiểu và storage bảo mật theo nền tảng.

| Owner | Responsibility | Trách nhiệm |
|---|---|---|
| `error/` | Stable shared `CryptoError` compatibility boundary; local feature errors remain in their owners. Runtime tests remain NOT_RUN. | Boundary `CryptoError` dùng chung ổn định; lỗi chức năng local vẫn ở owner riêng. Runtime test vẫn NOT_RUN. |
| `result/` | Exact `CryptoResult<T> = Result<T, CryptoError>` public alias. Runtime tests remain NOT_RUN. | Alias public chính xác `CryptoResult<T> = Result<T, CryptoError>`. Runtime test vẫn NOT_RUN. |
| `blake3_signer/` | BLAKE3 digest representation, encoding, hashing and file input, split into same-name owners with nearest tests. Runtime tests remain NOT_RUN. | Kiểu digest BLAKE3, encoding, hashing và file input được tách theo owner cùng tên, có test gần owner. Runtime test vẫn NOT_RUN. |
| `checksum/` | Adler-32 calculation and expected-value comparison. The algorithm is quick and non-cryptographic; nearest tests are authored but NOT_RUN. | Phép tính Adler-32 và so sánh giá trị kỳ vọng. Thuật toán nhanh, không có tính mật mã; test gần owner đã viết nhưng NOT_RUN. |
| `ed25519_signer/` | `ring`-backed signer, signature/public-key wrappers, Base64, verification and typed local errors mapped to stable `CryptoError`. Runtime tests remain NOT_RUN. | Signer dùng `ring`, wrapper signature/public key, Base64, verification và typed local error map về `CryptoError` ổn định. Runtime test vẫn NOT_RUN. |
| `integrity/` | SRI values, BLAKE3 compute/verification, file operations and local error mapping, split into documented nearest-test owners. Runtime tests remain NOT_RUN. | Value SRI, tính/verify BLAKE3, thao tác file và map lỗi local, tách thành owner có README/test gần nhất. Runtime test vẫn NOT_RUN. |
| `keyring/` | Key-pair lifecycle, key selection, secure persistence and path policy. | Vòng đời key pair, chọn key, lưu bảo mật và policy path. |
| `simd/` | Capability value/serde, architecture-gated detection and info rendering; fixed multipliers are unmeasured legacy estimates. Runtime tests remain NOT_RUN. | Value/serde capability, detection có cfg kiến trúc và dựng info; multiplier cố định là ước lượng legacy chưa đo. Runtime test vẫn NOT_RUN. |

The keyring public paths remain `mgc_crypto::keyring::{KeyPair, Keyring}`. Tests under `tests/` preserve public-crate integration; source-local tests cover focused private owners.

Public path keyring giữ nguyên `mgc_crypto::keyring::{KeyPair, Keyring}`. Test trong `tests/` giữ integration public-crate; test gần source bao phủ owner private.
