import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			fallback: 'fallback.html'
		}),
		paths: {
			base: process.env.VITE_BASE_HREF || ''
		},
		inlineStyleThreshold: 1024
	}
};

export default config;
