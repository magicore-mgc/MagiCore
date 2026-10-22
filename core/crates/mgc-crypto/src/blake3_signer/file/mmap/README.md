# Former mmap API compatibility

This module deliberately performs streaming I/O. The historical public method name is preserved to avoid an API break, but it no longer claims mmap performance. Remove or replace only through a separately reviewed API migration. The safe streaming behavior is tested here.
