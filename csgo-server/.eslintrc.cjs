module.exports = {
    root: true,
    extends: ["plugin:vue/vue3-essential", "eslint:recommended", "@vue/eslint-config-typescript"],
    parser: "vue-eslint-parser",
    parserOptions: {
        parser: "@typescript-eslint/parser",
        ecmaVersion: 2022,
        sourceType: "module"
    },
    rules: {
        indent: ["warn", 4, { SwitchCase: 1 }],
        semi: ["error", "always"],
        "space-before-function-paren": ["error", { anonymous: "never", named: "never", asyncArrow: "always" }],
        "no-console": process.env.NODE_ENV === "production" ? "warn" : "off",
        "no-debugger": process.env.NODE_ENV === "production" ? "warn" : "off",
        "arrow-parens": ["error", "as-needed"],
        curly: ["error", "multi"],
        quotes: ["error", "double", { avoidEscape: true }],
        "comma-dangle": [
            "error",
            { arrays: "never", objects: "never", imports: "never", exports: "never", functions: "never" }
        ],
        "linebreak-style": ["error", "windows"],
        "brace-style": "off",
        "no-useless-escape": "off",
        "no-case-declarations": "off",
        "no-mixed-operators": "off",
        "operator-linebreak": "off",
        "standard/no-callback-literal": "off",
        "standard/computed-property-even-spacing": "off",
        "node/no-callback-literal": "off",
        "vue/multi-word-component-names": "off"
    }
};
