# Registry URL policy

This owner validates registry bases before credential-bearing requests, builds encoded endpoint path segments without discarding a configured prefix, and creates clients that refuse redirects. Only HTTPS or HTTP to localhost/IP loopback is accepted. `MGC_REGISTRY_ADMIN_TIMEOUT_SECS` configures the admin request timeout within the documented bound in `registry_url.rs`.

Owner này xác thực URL registry trước request có credential, tạo endpoint bằng path segment đã encode và giữ prefix cấu hình, đồng thời tạo client không theo redirect. Chỉ nhận HTTPS hoặc HTTP tới localhost/IP loopback. `MGC_REGISTRY_ADMIN_TIMEOUT_SECS` cấu hình timeout admin trong giới hạn ghi ở `registry_url.rs`.
