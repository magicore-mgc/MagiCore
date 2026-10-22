# Polyglot audit plans / Plan audit polyglot

This owner groups one plan builder per language lane that can aggregate with Rust. Each builder preserves the shared `AuditPlan` execution path; Java and .NET also preserve their explicit Unsupported fallback when the ecosystem constructor declines.

Owner này nhóm một plan builder cho mỗi lane ngôn ngữ có thể gộp với Rust. Mỗi builder giữ đường chạy `AuditPlan` dùng chung; Java và .NET giữ fallback Unsupported rõ ràng khi constructor ecosystem từ chối.

The parent only wires the four owners. It does not select the language or detect manifests; those decisions remain in the audit dispatcher. Direct polyglot tests are pending in the later verification phase.

Parent chỉ nối bốn owner. Nó không chọn ngôn ngữ hoặc nhận diện manifest; quyết định đó vẫn ở dispatcher audit. Test polyglot trực tiếp đang chờ phase verify sau.
