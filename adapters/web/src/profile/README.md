# Web profile owners

EN: This module preserves the public `crate::profile::*` API while splitting install/resolve timing, pipeline metrics, materialization counters, and fetch-result types. `TarballFetchResult` remains the same public struct with identical fields; callers do not need import changes.

VI: Module này giữ API công khai `crate::profile::*`, đồng thời tách thời gian install/resolve, metric pipeline, bộ đếm materialization và type fetch result. `TarballFetchResult` vẫn là struct công khai cùng các field hiện tại; caller không cần đổi import.
