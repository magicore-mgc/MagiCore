# V4 finalization / Hoàn thiện v4

This owner fills the payload digest, emits canonical v4 TOML, and reparses the bytes to reject serializer drift. Callers own staging and atomic file replacement.

Owner này điền digest payload, sinh TOML v4 canonical và parse lại bytes để chặn serializer drift. Caller chịu trách nhiệm staging và thay file atomic.
