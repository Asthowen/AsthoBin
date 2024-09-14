// noinspection JSUnusedGlobalSymbols

import eslintPluginPrettier from "eslint-plugin-prettier";
import eslintPluginImport from "eslint-plugin-import";
import eslintPluginTypescript from "@typescript-eslint/eslint-plugin";
import eslintParserTypescript from "@typescript-eslint/parser";


export default [
    {
        ignores: ["node_modules/**", "static/**", "target/**", "src/**", "migrations/**"],
        files: ["frontend/ts/*.ts", "vite.config.mts", "tailwind.config.ts"],
        languageOptions: {
            ecmaVersion: "latest",
            sourceType: "module",
            parser: eslintParserTypescript,
            parserOptions: {
                project: "./tsconfig.json",
            },
        },
        plugins: {
            prettier: eslintPluginPrettier,
            import: eslintPluginImport,
            "@typescript-eslint": eslintPluginTypescript,
        },
        rules: {
            "prettier/prettier": "error",
            "import/extensions": [
                "error",
                "ignorePackages",
                {
                    js: "never",
                    jsx: "never",
                    ts: "never",
                    tsx: "never",
                },
            ],
        },
    },
];