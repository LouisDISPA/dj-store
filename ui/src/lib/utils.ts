import { goto as svelteGoto } from '$app/navigation';

export const env = {
	BASE_HREF: import.meta.env.VITE_BASE_HREF || ''
};

export async function goto(path: string) {
	await svelteGoto(env.BASE_HREF + path);
}
