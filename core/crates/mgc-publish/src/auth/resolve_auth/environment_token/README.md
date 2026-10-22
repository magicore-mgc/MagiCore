# Environment token source

EN: Reads MGC_NPM_TOKEN first and NPM_TOKEN second, ignoring empty values.
VI: Đọc MGC_NPM_TOKEN trước, NPM_TOKEN sau và bỏ qua giá trị rỗng.

Tests: test/environment_token.rs checks precedence while restoring process environment.
