# Web HMR owners

This Web-owned HMR service watches project files, filters build/install noise, publishes reload events, serves the WebSocket protocol, and owns the embedded browser client script. The CLI keeps only compatibility exports; its `MgDevServer` still owns HTTP route registration and listener lifecycle.

Service HMR do Web sở hữu theo dõi file project, lọc nhiễu build/install, phát event reload, phục vụ WebSocket protocol và quản lý client script nhúng. CLI chỉ giữ export tương thích; `MgDevServer` vẫn sở hữu đăng ký HTTP route và vòng đời listener.

The watcher retains the existing 100-event broadcast channel, atomic version ordering, connected-subscription barrier, path-noise filter and user-visible logs. Tests beside each behavior supplement—but do not replace—the existing real-server WebSocket integration test.

Watcher giữ channel broadcast 100 event, thứ tự atomic version, barrier xác nhận subscription, path-noise filter và log hiển thị hiện có. Test gần từng behavior bổ sung nhưng không thay integration test WebSocket chạy server thật.
