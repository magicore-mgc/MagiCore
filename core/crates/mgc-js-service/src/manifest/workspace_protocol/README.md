# Workspace protocol / Protocol workspace

Owns the predicate used by both manifest parsing and writing to recognize `workspace:` ranges.

Sở hữu predicate được parser và writer dùng để nhận diện range `workspace:`.

The accepted spellings and trimming behavior are frozen from the previous implementation; this module does not resolve workspace members.

Các spelling được chấp nhận và cách trim giữ nguyên như implementation trước; module này không resolve thành viên workspace.
