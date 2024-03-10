/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {},
        secondary: {},
        tertiary: {},
        dark: {},
        light: {},
        success: {},
        warning: {},
        danger: {},
        info: {},
      },
      fontFamily: {
        epilogue: ["Epilogue", "sans-serif"],
      },
      boxShadow: {
        secondary: "10px 10px 20px rgba(2, 2, 2, 0.25)",
      },
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic": "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
      typography: (theme) => ({
        DEFAULT: {
          css: {
            color: "#808191",
            blockquote: {
              borderLeftColor: theme("colors.gray.700"),
              color: theme("colors.gray.100"),
            },
            h1: {
              color: theme("colors.slate.200"),
            },
            h2: {
              color: theme("colors.slate.200"),
            },
            h3: {
              color: theme("colors.slate.200"),
            },
            h4: {
              color: theme("colors.slate.200"),
            },
            h5: {
              color: theme("colors.slate.200"),
            },
            h6: {
              color: theme("colors.slate.200"),
            },
            a: {
              color: theme("colors.indigo.500"),
            },
            code: {
              padding: "3px 5px",
              borderRadius: 5,
              color: theme("colors.white"),
              background: theme("colors.gray.800"),
            },
            "pre > code": {
              background: "none",
              padding: 0,
            },
            hr: { borderColor: theme("colors.gray.700") },
            strong: { color: theme("colors.white") },
            thead: {
              color: theme("colors.gray.100"),
            },
            tbody: {
              tr: {
                borderBottomColor: theme("colors.gray.700"),
              },
            },
            mark: {
              background: theme("colors.yellow.100"),
            },
          },
        },
      }),
    },
  },
  plugins: [require("@tailwindcss/typography"), require("@tailwindcss/forms")],
  darkMode: "class",
}
