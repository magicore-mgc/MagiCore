//! Validate Web scaffold flags and apply named Web presets.
//! Kiểm tra cờ scaffold Web và áp dụng preset có tên.

use crate::commands::core::web::scaffold_flags::ScaffoldFlags;
use anyhow::Result;

pub(in crate::commands::core::web::implementation) fn validate_flags(
    flags: &ScaffoldFlags,
    fe_framework: &str,
) -> Result<()> {
    if let Some(pm) = flags.pm.as_deref() {
        return Err(crate::error::pm_not_supported(pm));
    }

    if flags.ts && flags.js {
        return Err(crate::error::ts_js_exclusive());
    }

    let fe_count = [
        flags.react,
        flags.next,
        flags.vue,
        flags.nuxt,
        flags.svelte,
        flags.sveltekit,
        flags.solid,
        flags.astro,
        flags.remix,
    ]
    .iter()
    .filter(|&&b| b)
    .count();

    if fe_count > 1 {
        return Err(crate::error::multiple_frameworks());
    }

    if flags.pinia && fe_framework != "vue" && fe_framework != "nuxt" {
        return Err(crate::error::pinia_requires_vue());
    }

    // Feature projection adds Tailwind for shadcn/daisyui; the parsed flag remains unchanged.
    // Feature projection tự thêm Tailwind cho shadcn/daisyui; không sửa cờ người dùng.

    Ok(())
}

pub(in crate::commands::core::web::implementation) fn apply_preset(
    name: &str,
    flags: &mut ScaffoldFlags,
) {
    match name {
        "t3" => {
            flags.next = true;
            flags.ts = true;
            flags.tailwindcss = true;
            flags.trpc = true;
            flags.prisma = true;
            flags.zod = true;
            flags.nextauth = true;
        }
        "mern" => {
            flags.react = true;
            flags.express = true;
            flags.mongoose = true;
            flags.mongodb = true;
            flags.zod = true;
            flags.jwt = true;
        }
        "jamstack" => {
            flags.astro = true;
            flags.tailwindcss = true;
            flags.dotenv = true;
            flags.vercel = true;
        }
        "saas" => {
            flags.next = true;
            flags.tailwindcss = true;
            flags.shadcn = true;
            flags.prisma = true;
            flags.postgres = true;
            flags.clerk = true;
            flags.docker = true;
        }
        "mevn" => {
            flags.vue = true;
            flags.express = true;
            flags.mongoose = true;
            flags.mongodb = true;
            flags.zod = true;
        }
        _ => {}
    }
}
