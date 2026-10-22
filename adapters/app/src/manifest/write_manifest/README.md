# App manifest writing

EN: Owns `AppLanguage` write dispatch and the Multi fallback order: Flutter, Gradle, Swift, React Native. Multi detection retains the current `Path::exists()` checks and forwards each format owner's result/error. MAUI and Tauri keep the existing Unsupported result and message.

VI: Sở hữu dispatch ghi theo `AppLanguage` và thứ tự fallback Multi: Flutter, Gradle, Swift, React Native. Multi tiếp tục dùng `Path::exists()` và trả nguyên kết quả/lỗi từ format owner. MAUI và Tauri giữ nguyên Unsupported cùng thông báo hiện tại.
