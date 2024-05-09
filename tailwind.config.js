/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**.rs", "./generated/posts/**.html"],
  darkMode: "selector",
  theme: {
    extend: {
      typography: (theme) => ({
        slate: {
          css: {
            "--tw-prose-invert-hr": theme("colors.slate.700"),
            "--tw-prose-hr": theme("colors.slate.300"),
          },
        },
        DEFAULT: {
          css: {
            p: {
              "margin-top": "0px",
              "margin-bottom": "0.75rem",
            },
            h1: {
              "margin-bottom": "0.75rem",
            },
            h2: {
              "margin-top": "1rem",
              "margin-bottom": "0.5rem",
            },
            h3: {
              "margin-top": "1rem",
              "margin-bottom": "0.5rem",
            },
            figure: {
              "margin-top": "1rem",
              "margin-bottom": "1rem",
            },
          },
        },
      }),
    },
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("@tailwindcss/forms")({ strategy: "base" }),
  ],
};
