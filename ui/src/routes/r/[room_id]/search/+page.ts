import { env } from '$lib/utils';
import { error, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export type SearchMusic = {
	id: number;
	title: string;
	artist: string;
};

export type PageData = {
	musics: SearchMusic[];
	authToken: string;
	roomCode: string;
};

// TODO: remove this and make the search page work

export const load: PageLoad<PageData> = async ({ params, url }) => {
	const { code: roomCode } = params;

	if (roomCode.length !== 6 || roomCode.toUpperCase() !== roomCode) {
		const message = `Invalid code: ${roomCode}`;
		const detail = 'The code must be 6 uppercase letters.';

		throw error(500, { message, detail });
	}
	const authToken = localStorage.getItem('authToken');

	if (!authToken) {
		console.log('no token');

		throw redirect(301, `/r/${roomCode}`);
	}

	const tokenData = JSON.parse(atob(authToken.split('.')[1]));
	if (tokenData.role !== 'Admin' && tokenData.room_id !== roomCode) {
		console.log('not admin or not in room');

		throw redirect(301, `/r/${roomCode}`);
	}

	const query = url.searchParams.get('query');

	if (!query) {
		console.log('no query');
		throw redirect(301, `/r/${roomCode}`);
	}

	const musics = await getSearch(roomCode, authToken, query);
	console.log(musics);

	return { musics, authToken, roomCode };
};

export async function getSearch(
	roomCode: string,
	authToken: string,
	query: string
): Promise<SearchMusic[]> {
	const res = await fetch(`${env.API_URL}/api/room/${roomCode}/search?query=${query}`, {
		headers: {
			Authorization: `Bearer ${authToken}`
		}
	});
	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}

	return await res.json();
}
