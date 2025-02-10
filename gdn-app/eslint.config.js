// @ts-check

// https://eslint.org/docs/latest/use/configure/
// https://eslint.vuejs.org/user-guide/

import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";
import configPrettier from "eslint-config-prettier";

export default tseslint.config(
  { ignores: ["dist/", "eslint.config.js", "vite.config.ts", "src/vite-env.d.ts"] },

  eslint.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
  ...pluginVue.configs["flat/recommended"],

  // Prettier's formatting decisions should not be second-guessed by eslint.
  configPrettier,

  // Set up the ts parser to use type info and understand vue files.
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,

        // Otherwise the parser will complain that it doesn't know vue files.
        extraFileExtensions: ["vue"],
      },
    },
  },

  // Tell the vue parser that it should use the ts parser instead of the js parser.
  { files: ["**/*.vue"], languageOptions: { parserOptions: { parser: tseslint.parser } } },

  // My own rules.
  //
  // https://eslint.org/docs/latest/rules/
  // https://typescript-eslint.io/rules/
  // https://eslint.vuejs.org/rules/
  {
    rules: {
      "@typescript-eslint/explicit-function-return-type": "error",
      "vue/block-lang": ["error", { script: { lang: "ts" } }],
      "vue/block-order": ["error", { order: ["script", "template", "style"] }],
      "vue/component-api-style": ["error", ["script-setup"]],
      "vue/v-for-delimiter-style": ["error", "of"],
    },
  },
);
