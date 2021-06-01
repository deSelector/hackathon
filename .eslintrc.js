module.exports = {
    env: {
        browser: true,
        es2021: true
    },
    extends: [
        "plugin:react/recommended",
        "eslint:recommended"
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaFeatures: {
            jsx: true
        },
        ecmaVersion: 12,
        sourceType: "module"
    },
    plugins: [
        "react",
        "@typescript-eslint"
    ],
    rules: {
        "no-unused-vars": ["off", { vars: "all", args: "none" }],
        "quote-props": ["warn", "as-needed"],
        // enable additional rules
        indent: ["off", 2],
        quotes: ["off", "double"],
        semi: ["warn", "always"],

        // override configuration set by extending "eslint:recommended"
        "no-empty": "warn",
        "no-cond-assign": ["error", "always"],

        // disable rules from base configurations
        "for-direction": "off",
        "no-undef": "off"
    }
};