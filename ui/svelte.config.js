import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess({
		postcss: true
	}),

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
