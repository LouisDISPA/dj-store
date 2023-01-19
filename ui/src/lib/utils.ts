import { dev } from '$app/environment';
import { goto as svelteGoto } from '$app/navigation';

export const env = {
	BASE_HREF: import.meta.env.VITE_BASE_HREF || '',
	API_URL: import.meta.env.VITE_API_URL || dev ? 'http://127.0.0.1:3000' : ''
};

export async function goto(path: string) {
	await svelteGoto(env.BASE_HREF + path);
}
