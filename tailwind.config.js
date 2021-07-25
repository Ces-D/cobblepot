// https://stackoverflow.com/questions/67679410/nextjs-not-able-to-use-custom-colors-in-tailwind-css-in
module.exports = {
  purge: [],
  darkMode: "media", // or 'media' or 'class'
  theme: {
    colors: {
      light: "#fcfcfc",
      dark: "#12151a",
      primary: "#710117",
      secondary: "#ECD5BB",
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
