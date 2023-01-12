import { env } from '$lib/utils';
import { error } from '@sveltejs/kit';
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
	const authToken = await joinRoom(roomCode);

	const musics = await getMusics(roomCode, authToken);
	console.log(musics);

	return { musics, authToken, roomCode };
};

async function joinRoom(code: string): Promise<string> {
	let authToken = localStorage.getItem('authToken');

	if (authToken) {
		const tokenData = JSON.parse(atob(authToken.split('.')[1]));

		if (tokenData.role === 'Admin') {
			return authToken;
		}

		const isInRoom = tokenData.room_id === code;
		if (isInRoom) {
			return authToken;
		}
	}

	const res = await fetch(`${env.API_URL}/api/room/${code}/join`);
	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}
	authToken = (await res.json())['access_token'] as string;
	localStorage.setItem('authToken', authToken);

	return authToken;
}

async function getMusics(code: string, authToken: string): Promise<Music[]> {
	const res = await fetch(`${env.API_URL}/api/room/${code}/musics`, {
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

	return await res.json();
}
