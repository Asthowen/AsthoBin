import { defineConfig } from "oxlint";

export default defineConfig({
    $schema: "./node_modules/oxlint/configuration_schema.json",
    plugins: ["eslint", "oxc", "typescript", "unicorn"],
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
    rules: {
        "eslint/max-lines-per-function": "off",
        "unicorn/prefer-query-selector": "off",
        "unicorn/prefer-add-event-listener": "off",
        "typescript/no-unsafe-type-assertion": "off",
        "typescript/prefer-readonly-parameter-types": "off",
    },
});
