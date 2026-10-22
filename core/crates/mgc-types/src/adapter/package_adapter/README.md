# PackageAdapter façade owner / Owner façade PackageAdapter

Owns the stable PackageAdapter trait contract that joins capability traits and keeps orchestrator-level methods at the adapter boundary. It intentionally remains one cohesive public trait: splitting methods into new supertraits would change the external implementer contract.

Sở hữu contract trait PackageAdapter ổn định, ghép capability trait và giữ method cấp orchestrator tại ranh giới adapter. Trait public được giữ nguyên khối: tách method thành supertrait sẽ đổi contract của code implement bên ngoài.
