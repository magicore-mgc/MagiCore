//! Web javascript framework qualification rows.
//! Các dòng qualification framework Web javascript.

use super::super::super::super::{EV_E2E_WEB_JS, EV_WIZARD, FrameworkRecord, FrameworkStatus};

pub(in crate::commands::framework_records::implementation::records::web) const RECORDS:
    &[FrameworkRecord] = &[
    FrameworkRecord {
        core: "web",
        framework: "angular",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "astro",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "express",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "fastify",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "hono",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "nestjs",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "nextjs",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "nuxt",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "qwik",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "react-fastify",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "web",
        framework: "react-spring",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "React + Spring Boot fullstack scaffold; dependency install is unsupported until both workspace lanes are MGC-owned",
    },
    FrameworkRecord {
        core: "web",
        framework: "react-vite",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "remix",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "solidjs",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "sveltekit",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "trpc",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "vue-vite",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
];
