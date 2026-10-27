import { defineConfig } from "oxlint";

export default defineConfig({
    $schema: "./node_modules/oxlint/configuration_schema.json",
    plugins: [],
    categories: {
        correctness: "error",
        suspicious: "warn",
        pedantic: "warn",
        perf: "warn",
    },
    options: {
        typeAware: true,
        typeCheck: true,
    },
    env: {
        builtin: true,
    },
    overrides: [
        {
            files: [
                "frontend/ts/*.ts",
                "oxfmt.config.ts",
                "oxlint.config.mts",
                "vite.config.mts",
            ],
            plugins: ["eslint", "oxc", "typescript", "unicorn"],
        },
    ],
});
