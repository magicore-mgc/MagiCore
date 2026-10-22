# publish_age

Checks publication timestamps against the configured quarantine age. The predicate reports missing/unparseable time as unknown (`Ok(false)`); its resolver caller applies the fail-closed default and accepts unknown time only under the explicit project escape setting.

Kiểm tra timestamp phát hành theo tuổi quarantine cấu hình. Predicate báo thời gian thiếu/không parse được là chưa rõ (`Ok(false)`); resolver áp dụng mặc định fail-closed và chỉ nhận thời gian chưa rõ khi project bật escape tường minh.

`mod.rs` declares the same-name implementation module and nearest tests live under `test/`. The two former cross-owner tests were moved here byte-identically in S530; S531 adds unknown-time and oversized-threshold cases. All remain NOT_RUN during the source-first phase.

`mod.rs` khai báo module triển khai cùng tên và test gần nhất nằm dưới `test/`. Hai test cũ từ owner khác được chuyển nguyên byte ở S530; S531 thêm ca thời gian chưa rõ và ngưỡng quá lớn. Tất cả còn NOT_RUN trong phase source-first.
