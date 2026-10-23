// Select scaffold features in stable CLI order — chọn feature scaffold theo thứ tự CLI.
// Web scaffold feature selection.
use super::*;

pub(in crate::commands::core::web::implementation) fn web_features(
    flags: &ScaffoldFlags,
) -> Vec<String> {
    let mut features = Vec::new();
    push_language(flags, &mut features);
    push_styling(flags, &mut features);
    push_state(flags, &mut features);
    push_backend(flags, &mut features);
    push_database_orm(flags, &mut features);
    push_validation(flags, &mut features);
    push_auth(flags, &mut features);
    push_testing(flags, &mut features);
    push_linting(flags, &mut features);
    push_monorepo(flags, &mut features);
    push_api(flags, &mut features);
    push_deployment(flags, &mut features);
    push_misc(flags, &mut features);
    push_extra(flags, &mut features);

    features
}

fn push_language(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.ts {
        features.push("typescript".into());
    }
    if flags.js {
        features.push("javascript".into());
    }
}

fn push_styling(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.tailwindcss || flags.shadcn || flags.daisyui {
        features.push("tailwindcss".into());
    }
    if flags.css_modules {
        features.push("css-modules".into());
    }
    if flags.styled_components {
        features.push("styled-components".into());
    }
    if flags.sass {
        features.push("sass".into());
    }
    if flags.unocss {
        features.push("unocss".into());
    }
    if flags.shadcn {
        features.push("shadcn".into());
    }
    if flags.daisyui {
        features.push("daisyui".into());
    }
}

fn push_state(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.zustand {
        features.push("zustand".into());
    }
    if flags.redux {
        features.push("redux".into());
    }
    if flags.jotai {
        features.push("jotai".into());
    }
    if flags.recoil {
        features.push("recoil".into());
    }
    if flags.pinia {
        features.push("pinia".into());
    }
    if flags.tanstack_query {
        features.push("tanstack-query".into());
    }
}

fn push_backend(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.express {
        features.push("express".into());
    }
    if flags.fastify {
        features.push("fastify".into());
    }
    if flags.nestjs {
        features.push("nestjs".into());
    }
    if flags.hono {
        features.push("hono".into());
    }
    if flags.koa {
        features.push("koa".into());
    }
    if flags.trpc {
        features.push("trpc".into());
    }
}

fn push_database_orm(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.prisma {
        features.push("prisma".into());
    }
    if flags.drizzle {
        features.push("drizzle".into());
    }
    if flags.typeorm {
        features.push("typeorm".into());
    }
    if flags.mongoose {
        features.push("mongoose".into());
    }
    if flags.postgres {
        features.push("postgres".into());
    }
    if flags.mysql {
        features.push("mysql".into());
    }
    if flags.sqlite {
        features.push("sqlite".into());
    }
    if flags.mongodb {
        features.push("mongodb".into());
    }
}

fn push_validation(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.zod {
        features.push("zod".into());
    }
    if flags.yup {
        features.push("yup".into());
    }
    if flags.joi {
        features.push("joi".into());
    }
    if flags.valibot {
        features.push("valibot".into());
    }
}

fn push_auth(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.nextauth {
        features.push("nextauth".into());
    }
    if flags.clerk {
        features.push("clerk".into());
    }
    if flags.lucia {
        features.push("lucia".into());
    }
    if flags.jwt {
        features.push("jwt".into());
    }
    if flags.oauth {
        features.push("oauth".into());
    }
}

fn push_testing(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.vitest {
        features.push("vitest".into());
    }
    if flags.jest {
        features.push("jest".into());
    }
    if flags.playwright {
        features.push("playwright".into());
    }
    if flags.cypress {
        features.push("cypress".into());
    }
    if flags.testing_library {
        features.push("testing-library".into());
    }
}

fn push_linting(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.eslint {
        features.push("eslint".into());
    }
    if flags.prettier {
        features.push("prettier".into());
    }
    if flags.biome {
        features.push("biome".into());
    }
    if flags.husky {
        features.push("husky".into());
    }
    if flags.lint_staged {
        features.push("lint-staged".into());
    }
    if flags.commitlint {
        features.push("commitlint".into());
    }
}

fn push_monorepo(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.monorepo {
        features.push("monorepo".into());
    }
    if flags.turborepo {
        features.push("turborepo".into());
    }
    if flags.nx {
        features.push("nx".into());
    }
    if flags.workspaces {
        features.push("workspaces".into());
    }
    if flags.changesets {
        features.push("changesets".into());
    }
}

fn push_api(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.rest {
        features.push("rest".into());
    }
    if flags.graphql {
        features.push("graphql".into());
    }
    if flags.trpc_api {
        features.push("trpc-api".into());
    }
    if flags.grpc {
        features.push("grpc".into());
    }
}

fn push_deployment(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.docker || flags.k8s {
        features.push("docker".into());
    }
    if flags.k8s {
        features.push("k8s".into());
    }
    if flags.github_actions {
        features.push("github-actions".into());
    }
    if flags.vercel {
        features.push("vercel".into());
    }
    if flags.railway {
        features.push("railway".into());
    }
    if flags.fly {
        features.push("fly".into());
    }
}

fn push_misc(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    if flags.dotenv {
        features.push("dotenv".into());
    }
    if flags.i18n {
        features.push("i18n".into());
    }
    if flags.pwa {
        features.push("pwa".into());
    }
    if flags.storybook {
        features.push("storybook".into());
    }
    if flags.sentry {
        features.push("sentry".into());
    }
    if flags.analytics {
        features.push("analytics".into());
    }
}

fn push_extra(flags: &ScaffoldFlags, features: &mut Vec<String>) {
    for feature in &flags.features {
        if !features.contains(feature) {
            features.push(feature.clone());
        }
    }
}
