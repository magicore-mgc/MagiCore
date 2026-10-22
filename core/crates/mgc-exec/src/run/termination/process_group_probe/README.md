# process_group_probe / probe process group

Owns Unix PID-to-PGID lookup, liveness probing and descendant-group enumeration. Expected ESRCH races return the existing absent/dead result; invalid identities and unexpected OS errors retain their typed source.

Sở hữu tra PID→PGID, probe liveness và liệt kê group hậu duệ Unix. Race ESRCH dự kiến giữ kết quả vắng/chết hiện có; identity sai và lỗi OS khác giữ source typed.
