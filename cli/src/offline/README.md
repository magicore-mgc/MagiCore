# Offline mode state

## Purpose

Owns the global offline flag propagated through `MGC_OFFLINE_MODE`: one setter used at install entry before adapters spawn workers, and one reader used by adapters to close resolve/install network gates fail-closed.

## Vietnamese

Sở hữu cờ offline toàn cục qua `MGC_OFFLINE_MODE`: một setter dùng ở entry install trước khi adapter spawn worker, và một reader để adapter đóng cổng mạng resolve/install theo hướng fail-closed.
