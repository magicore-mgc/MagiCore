//! IoT framework qualification rows.
//! Các dòng qualification framework IoT.

use super::super::super::{
    EV_DEPGATE, EV_E2E_IOT_PIO, EV_E2E_IOT_WEST, FrameworkRecord, FrameworkStatus,
};

pub(in crate::commands::framework_records::implementation::records) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "iot",
        framework: "esp32-rust",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_DEPGATE,
    },
    FrameworkRecord {
        core: "iot",
        framework: "platformio",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_IOT_PIO,
    },
    FrameworkRecord {
        core: "iot",
        framework: "zephyr-arm",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_IOT_WEST,
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32c3",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32dev",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32s3",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "nodemcu-32s",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "nrf52dk_nrf52832",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "stm32f4_disc",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
];
