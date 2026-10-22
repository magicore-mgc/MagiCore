# Job lifecycle tests

Windows-gated tests create and terminate a real private Job Object without starting a child. Child assignment, suspended-thread resume, kill-on-close and descendant cleanup remain Windows lifecycle/E2E responsibilities.

Test Windows tạo và terminate Job Object riêng thật nhưng không chạy child. Gắn child, resume thread suspend, kill-on-close và dọn descendant vẫn thuộc lifecycle/E2E Windows.
