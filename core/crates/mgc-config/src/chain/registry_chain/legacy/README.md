# Legacy registry chain / Registry-chain legacy

This owner preserves the original infallible API and its best-effort .npmrc load behavior. An explicit registry URL override still avoids loading project .npmrc; load errors remain swallowed only on this compatibility route.

Owner này giữ API infallible ban đầu và cách load .npmrc best-effort. Khi có URL override tường minh, project .npmrc vẫn không được load; lỗi load chỉ bị bỏ qua trên route tương thích này.

The checked APIs are implemented by the checked owner and continue to surface their existing errors. Existing integration tests remain the later runtime backstop.

API checked do owner checked triển khai và tiếp tục trả về lỗi như hiện tại. Integration test hiện có vẫn là backstop cho phase runtime sau.
