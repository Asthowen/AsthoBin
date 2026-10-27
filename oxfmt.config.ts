import { defineConfig } from "oxfmt";

export default defineConfig({
    $schema: "./node_modules/oxfmt/configuration_schema.json",
    semi: true,
    singleQuote: false,
    trailingComma: "all",
    tabWidth: 4,
    printWidth: 80,
    sortPackageJson: true,
    sortImports: true,
    ignorePatterns: [],
});
