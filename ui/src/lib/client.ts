import { error } from '@sveltejs/kit';
import { writable, type Writable } from 'svelte/store';
import type { Music } from './types';
import { env } from './utils';

const votes: Writable<Set<string>> = writable(new Set());

async function getMusics(auth_token: string, room_id: string): Promise<Music[]> {
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

async function getSearch(auth_token: string, rooom_id: string, query: string): Promise<Music[]> {
	const res = await fetch(`${env.API_URL}/api/room/${rooom_id}/search?query=${query}`, {
		headers: {
			Authorization: `Bearer ${auth_token}`
		}
	});
	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}

	return await res.json();
}

async function voteForMusic(auth_token: string, room_id: string, like: boolean, id: string) {
	const response = await fetch(`${env.API_URL}/api/room/${room_id}/vote`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${auth_token}`
		},
		body: JSON.stringify({
			music_id: id,
			like
		})
	});

	if (!response.ok) {
		throw new Error(`Error voting: ${await response.text()}`);
	}
	votes.update((set) => {
		if (like) {
			set.add(id);
		} else {
			set.delete(id);
		}
		return set;
	});
}

export { getMusics, getSearch, voteForMusic, votes };
