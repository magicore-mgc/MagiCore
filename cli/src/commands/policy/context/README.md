# Policy context owner / Owner ngữ cảnh policy

Resolve framework/version/OS/operation and load a spawn profile only when the current `mgc.lock` digest matches. Config, framework and lock lookup failures keep the existing `None` behavior.

Owner này chỉ tạo lane tuple và đọc profile gắn với digest lock. Không đọc trust roots hoặc tải manifest. Shared policy types remain owned by `mgc_exec`; no private type or surfaced error contract exists here.

The nearest test confirms that a missing lock cannot activate a profile. Runtime test NOT_RUN.
