# Flutter install owner / Owner install Flutter

EN: Owns the native Flutter install call and its existing canonical-lock publication condition. It does not spawn `flutter pub get`; network/cache behavior stays in the existing native Flutter owner. `test/` freezes the lock-write policy.

VI: Sở hữu lời gọi install Flutter native và điều kiện ghi canonical lock hiện tại. Owner không spawn `flutter pub get`; hành vi mạng/cache vẫn thuộc owner Flutter native hiện có. `test/` khóa policy ghi lock.
