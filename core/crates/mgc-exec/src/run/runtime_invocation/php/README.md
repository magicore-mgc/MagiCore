# php

Owns PHP file-operand parsing for positional scripts, -f/--file and --file= forms, plus value-taking configuration flags. Inline -r/--run/-a modes remain opaque.

Sở hữu parse file operand PHP dạng positional, -f/--file, --file= và các flag cấu hình có value. Chế độ inline -r/--run/-a luôn opaque.

An unrecognized or inline invocation returns None as a normal non-match. Nearest tests are in test/php.rs.
