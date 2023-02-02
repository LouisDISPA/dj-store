import { env } from '$lib/utils';
import { error, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export type Music = {
	id: number;
	title: string;
	artist: string;
	is_voted: boolean;
	votes: number;
};

export type PageData = {
	musics: Music[];
	authToken: string;
	roomCode: string;
};

export const load: PageLoad<PageData> = async ({ params }) => {
	const { code: roomCode } = params;

	if (roomCode.length !== 6 || roomCode.toUpperCase() !== roomCode) {
		const message = `Invalid code: ${roomCode}`;
		const detail = 'The code must be 6 uppercase letters.';

		throw error(500, { message, detail });
	}

	const authToken = localStorage.getItem('authToken');
	if (!authToken) {
		return redirect(301, '/login');
	}

	const tokenData = JSON.parse(atob(authToken.split('.')[1]));
	if (tokenData.role !== 'Admin') {
		return redirect(301, '/login');
	}

	const musics = await getMusics(roomCode, authToken);
	console.log(musics);

	return { musics, authToken, roomCode };
};

async function getMusics(code: string, authToken: string): Promise<Music[]> {
	const res = await fetch(`${env.API_URL}/api/room/${code}/music/voted`, {
		headers: {
			Authorization: `Bearer ${authToken}`
		}
	});
	if (!res.ok) {
		localStorage.removeItem('authToken');
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}

	const musics: Music[] = await res.json();

	musics.sort((a, b) => b.votes - a.votes);

	return musics;
}
