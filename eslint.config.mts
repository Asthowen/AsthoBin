import prettierPlugin from "eslint-plugin-prettier";
import typescriptPlugin from "@typescript-eslint/eslint-plugin";
import type { FlatConfig } from "@typescript-eslint/utils/ts-eslint";
import typescriptParser from "@typescript-eslint/parser";

export default [
  {
    ignores: [
      "node_modules/**",
      "static/**",
      "target/**",
      "src/**",
      "migrations/**",
      "docker/**",
      ".github/**",
    ],
    files: [
      "frontend/ts/*.ts",
      "vite.config.mts",
      "tailwind.config.ts",
      "postcss.config.mts",
      "eslint.config.mts",
    ],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      parser: typescriptParser,
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    plugins: {
      prettierPlugin,
      "@typescript-eslint": typescriptPlugin,
    },
    rules: {
      "prettierPlugin/prettier": "error",
      ...typescriptPlugin.configs.strict.rules,
      "@typescript-eslint/no-non-null-assertion": "off",
    },
    settings: {
      "import/resolver": {
        typescript: {
          project: "./tsconfig.json",
        },
      },
    },
  },
] satisfies FlatConfig.Config[];
