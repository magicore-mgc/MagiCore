# Package Adapter IoT adapter owner

Owns the PackageAdapter façade methods, manifest identity/ownership and parse/list behavior. PlatformIO parsing is called only through the local parser owner.

Sở hữu method façade PackageAdapter, manifest identity/ownership và hành vi parse/list. Parser PlatformIO chỉ được gọi qua owner cục bộ.

The owner preserves the current IoT capability matrix and reuses shared types/errors.

Owner giữ nguyên capability matrix IoT và dùng lại type/error chung.
