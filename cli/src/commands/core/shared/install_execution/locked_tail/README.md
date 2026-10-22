# Locked install tail

Owns the adapter install, progress rendering, profiling and summary output while retaining the caller's writer lock for the full operation. It does not recover journals.

Sở hữu adapter install, progress, profiling và summary khi giữ writer lock của caller xuyên operation. Owner này không tự recovery journal.
