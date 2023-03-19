/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	plugins: [require('@tailwindcss/typography'), require('daisyui')],
	daisyui: {
		themes: [
			{
				mytheme: {
					primary: '#1666dd',
					secondary: '#e5898b',
					accent: '#a3163e',
					neutral: '#111827',
					'base-100': '#202533',
					info: '#2C61F2',
					success: '#18A079',
					warning: '#A4610E',
					error: '#F90643'
				}
			}
		]
	}
};
