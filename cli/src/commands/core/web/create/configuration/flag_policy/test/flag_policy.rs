//! Web flag-policy owner tests.
//! Test owner policy cờ Web.

use crate::commands::core::web::scaffold_flags::ScaffoldFlags;

#[test]
fn rejects_simultaneous_typescript_and_javascript_flags() {
    let flags = ScaffoldFlags {
        ts: true,
        js: true,
        ..ScaffoldFlags::default()
    };

    assert!(super::validate_flags(&flags, "react").is_err());
}

#[test]
fn rejects_multiple_frontend_framework_flags() {
    let flags = ScaffoldFlags {
        react: true,
        vue: true,
        ..ScaffoldFlags::default()
    };

    assert!(super::validate_flags(&flags, "react").is_err());
}

#[test]
fn rejects_pinia_outside_vue_or_nuxt() {
    let flags = ScaffoldFlags {
        pinia: true,
        ..ScaffoldFlags::default()
    };

    assert!(super::validate_flags(&flags, "react").is_err());
}

#[test]
fn accepts_pinia_with_vue() {
    let flags = ScaffoldFlags {
        pinia: true,
        ..ScaffoldFlags::default()
    };

    assert!(super::validate_flags(&flags, "vue").is_ok());
}

#[test]
fn rejects_custom_package_manager_override() {
    let flags = ScaffoldFlags {
        pm: Some("pnpm".to_string()),
        ..ScaffoldFlags::default()
    };

    assert!(super::validate_flags(&flags, "react").is_err());
}

#[test]
fn applies_each_named_preset_to_its_expected_flags() {
    let mut t3 = ScaffoldFlags::default();
    super::apply_preset("t3", &mut t3);
    assert!(t3.next && t3.ts && t3.tailwindcss && t3.trpc && t3.prisma && t3.zod && t3.nextauth);

    let mut mern = ScaffoldFlags::default();
    super::apply_preset("mern", &mut mern);
    assert!(mern.react && mern.express && mern.mongoose && mern.mongodb && mern.zod && mern.jwt);

    let mut jamstack = ScaffoldFlags::default();
    super::apply_preset("jamstack", &mut jamstack);
    assert!(jamstack.astro && jamstack.tailwindcss && jamstack.dotenv && jamstack.vercel);

    let mut saas = ScaffoldFlags::default();
    super::apply_preset("saas", &mut saas);
    assert!(
        saas.next
            && saas.tailwindcss
            && saas.shadcn
            && saas.prisma
            && saas.postgres
            && saas.clerk
            && saas.docker
    );

    let mut mevn = ScaffoldFlags::default();
    super::apply_preset("mevn", &mut mevn);
    assert!(mevn.vue && mevn.express && mevn.mongoose && mevn.mongodb && mevn.zod);
}

#[test]
fn unknown_preset_leaves_flags_unchanged() {
    let mut flags = ScaffoldFlags::default();

    super::apply_preset("unknown", &mut flags);

    assert!(!flags.react && !flags.next && !flags.vue && !flags.ts);
}
