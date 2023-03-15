import { error } from '@sveltejs/kit';
import type { Music } from './types';
import { env } from './utils';

async function getMusics(room_id: string, auth_token: string): Promise<Music[]> {
	const res = await fetch(`${env.API_URL}/api/room/${room_id}/music/all`, {
		headers: {
			Authorization: `Bearer ${auth_token}`
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

export { getMusics };
