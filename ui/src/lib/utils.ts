import { dev } from '$app/environment';
import { goto as svelteGoto } from '$app/navigation';

export const env = {
	BASE_HREF: import.meta.env.VITE_BASE_HREF || '',
	API_URL: import.meta.env.VITE_API_URL || dev ? 'http://192.168.1.68:3000' : ''
};

export async function goto(path: string) {
	await svelteGoto(env.BASE_HREF + path);
}
