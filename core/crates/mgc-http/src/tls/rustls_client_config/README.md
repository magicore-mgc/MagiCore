# rustls client configuration owner

EN: Owns native/custom trust roots, TLS protocol selection, mTLS selection, and the existing NoVerify implementation. NoVerify bypasses certificate and handshake-signature checks and must remain reachable only through the current configuration branch.

VI: Sở hữu native/custom trust roots, chọn protocol TLS, chọn mTLS và implementation NoVerify hiện tại. NoVerify bỏ qua kiểm tra certificate cùng chữ ký handshake và chỉ được giữ reachable theo nhánh cấu hình hiện tại.
