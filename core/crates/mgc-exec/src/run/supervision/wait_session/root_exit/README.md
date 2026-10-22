# Root exit

Owns completion after `Child::try_wait` observes root-process exit, including the Unix descendant/process-group race checks and Windows Job cleanup handoff.

Unix retains its final descendant refresh, process-group descendant guard, remaining-time output drain, and timeout cleanup. Windows retains `finish_monitored_root_exit` and drops the Job guard before its bounded drain when monitoring is disabled.

# Root process thoát

> Owner này xử lý hoàn tất sau khi `Child::try_wait` thấy root process đã thoát, gồm kiểm tra race descendant/process group trên Unix và bàn giao cleanup Windows Job.

> Unix giữ lần refresh descendant cuối, kiểm tra process group, drain theo thời gian còn lại và cleanup timeout. Windows giữ `finish_monitored_root_exit`; khi monitoring tắt vẫn thả Job guard trước bounded drain.
