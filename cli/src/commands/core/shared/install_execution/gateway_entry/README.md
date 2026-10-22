# Gateway entry

Owns the standalone install entry that acquires the dependency-mutation writer lock and enters the locked tail. Mutation recovery stays at the gateway boundary.

Sở hữu entry install độc lập lấy writer lock cho dependency mutation rồi gọi tail đã khóa. Recovery vẫn nằm tại ranh giới gateway.
