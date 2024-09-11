/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        "./src/**/*.{rs,html}"
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require('daisyui'),
    ],
    daisyui: {
        themes: [
            "bumblebee",
            {
                aver: {
                    ...require("daisyui/src/theming/themes")["night"],
                    "accent": "#f74c00ff",
                },
            },
        ],
        darkTheme: "aver",
        base: true,
        styled: true,
        utils: true,
        prefix: "",
        logs: true,
        themeRoot: ":root",
    },
};
