import js from "@eslint/js";
import ts from "typescript-eslint";

export default [
  {
    ignores: ["dist/**", "node_modules/**", "pnpm-lock.yaml"],
  },
  js.configs.recommended,
  ...ts.configs.recommended,
  {
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
    },
    rules: {
      indent: ["error", 2],
      quotes: ["error", "single"],
      semi: ["error", "always"],
    },
  },
];
