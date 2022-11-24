import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export type Music = {
	id: number;
	title: string;
	artist: string;
	votes: number;
};

export type PageData = {
	musics: Music[];
};

export const load: PageLoad<PageData> = async ({ params }) => {
	const { code } = params;

	if (code.length !== 6 || code.toUpperCase() !== code) {
		const message = `Invalid code: ${code}`;
		const detail = 'The code must be 6 uppercase letters.';

		throw error(500, { message, detail });
	}

	const res = await fetch(`http://localhost:3000/api/${code}/musics`);

	if (res.status >= 300) {
		const message = 'Empty Room';
		const detail = 'Could not fetch the musics.';

		throw error(res.status, { message, detail });
	}

	const musics: Music[] = await res.json();
	return { musics };
};
