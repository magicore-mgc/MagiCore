# Web scaffold flags

## Purpose

Owns the `ScaffoldFlags` clap schema for `create-web`. The schema is pure data: it must stay available even when the `web` feature is off, because command definitions reference it.

## Vietnamese

Sở hữu schema clap `ScaffoldFlags` cho `create-web`. Schema là dữ liệu thuần: phải luôn tồn tại kể cả khi tắt feature `web`, vì phần định nghĩa lệnh tham chiếu tới nó.
