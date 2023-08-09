/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "all",
	content: [
		"./src/**/*.rs",
		"./index.html",
		"./src/**/*.html",
		"./src/**/*.css",
		"./static/scripts/*.js",
		"./node_modules/tw-elements/dist/js/**/*.js"
	],
	darkMode: 'class',
	theme: {},
	variants: {},
	plugins: [
		require("tw-elements/dist/plugin.cjs")
	],
};
