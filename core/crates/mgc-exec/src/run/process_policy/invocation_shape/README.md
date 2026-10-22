# Invocation shape

This owner turns a detected package-manager match into a bounded diagnostic shape. It keeps recognized safe commands/flags, masks arbitrary arguments, limits the number of rendered arguments, and preserves exact read-only probe labels. It does not decide whether a process is allowed.

## Hình dạng invocation

Owner chuyển một match package-manager thành diagnostic có giới hạn: giữ command/flag an toàn đã nhận diện, che argument tùy ý, giới hạn số argument hiển thị và giữ nhãn probe đọc-only. Owner này không quyết định process có được phép hay không.
