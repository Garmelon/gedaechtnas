// https://eslint.org/docs/latest/use/configure/
// https://eslint.vuejs.org/user-guide/

import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";
import configPrettier from "eslint-config-prettier";

export default tseslint.config(
  { ignores: ["dist/", "src/vite-env.d.ts"] },
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  ...tseslint.configs.stylistic,
  ...pluginVue.configs["flat/recommended"],

  // We don't want prettier's formatting decisions being second-guessed by eslint.
  configPrettier,

  // We need to tell the vue parser that it should use the ts parser instead of the js parser.
  { files: ["**/*.vue"], languageOptions: { parserOptions: { parser: tseslint.parser } } },
);
