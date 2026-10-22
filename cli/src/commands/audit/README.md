# CLI audit command owners / Owner lệnh CLI audit

The audit command coordinates scanner adapters for every core and exposes one stable output/exit contract. mod.rs is the public facade; audit.rs dispatches; output_format, strict_mode and report own their separate behavior. Per-core adapters remain in their existing modules.

Lệnh audit điều phối scanner adapter của mọi core và giữ một output/exit contract ổn định. mod.rs là facade; audit.rs dispatch; output_format, strict_mode và report sở hữu hành vi riêng. Adapter từng core giữ nguyên module hiện tại.

The existing test folder owns cross-owner format, strict coverage and fix-output contracts. This source-first change moves implementation only; tests have not been executed.

Thư mục test hiện tại giữ contract format, strict coverage và output khi fix đi qua nhiều owner. Thay đổi source-first này chỉ tách implementation; chưa chạy test.
