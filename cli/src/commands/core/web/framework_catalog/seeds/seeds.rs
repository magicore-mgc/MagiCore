// Ordered framework seed catalog — catalog seed framework giữ thứ tự ổn định.
// This owner assembles framework-specific data without changing resolution policy.
#[path = "frameworks/mod.rs"]
mod frameworks;
#[path = "types.rs"]
mod types;
pub(in crate::commands::core::web::implementation) use types::WebFrameworkSeed;
#[cfg(test)]
pub(in crate::commands::core::web::implementation) use types::WebToolchainPackage;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(in crate::commands::core::web::implementation) const FRAMEWORK_SEEDS: &[WebFrameworkSeed] = &[
    frameworks::VANILLA_SEED,
    frameworks::SOLIDJS_SEED,
    frameworks::SVELTEKIT_SEED,
    frameworks::REACT_VITE_SEED,
    frameworks::NEXTJS_SEED,
    frameworks::FASTIFY_SEED,
    frameworks::VUE_VITE_SEED,
    frameworks::NUXT_SEED,
    frameworks::ANGULAR_SEED,
    frameworks::QWIK_SEED,
    frameworks::ASTRO_SEED,
    frameworks::EXPRESS_SEED,
    frameworks::HONO_SEED,
    frameworks::NESTJS_SEED,
    frameworks::TRPC_SEED,
];
