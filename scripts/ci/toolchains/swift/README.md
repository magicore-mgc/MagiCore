# Shared Swift signing-key import / Nhập khóa ký Swift dùng chung

`import_signing_keys_linux.sh` downloads Swift.org's published key bundle, imports it into the runner keyring and verifies both required fingerprints. The stress and lifecycle matrices use the same exact operation before their pinned Swift setup. The release-binary E2E has a separate curl retry/timeout policy and therefore retains a separate owner.

`import_signing_keys_linux.sh` tải key bundle do Swift.org công bố, nhập vào keyring runner rồi xác minh đủ hai fingerprint bắt buộc. Stress và lifecycle matrix dùng chung nguyên thao tác trước khi setup Swift ghim phiên bản chạy. Release-binary E2E có retry/timeout curl khác nên giữ owner riêng.
