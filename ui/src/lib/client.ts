import { error } from '@sveltejs/kit';
import { writable, type Writable } from 'svelte/store';
import type { Music, MusicId, Room, RoomId } from './types';
import { convertApiRoom, env } from './utils';

const voted_for: Writable<Set<MusicId>> = writable(new Set());

async function getMusics(auth_token: string, room_id: RoomId): Promise<Music[]> {
	const res = await fetch(`${env.API_URL}/api/room/${room_id}/music/all`, {
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

async function getSearch(auth_token: string, rooom_id: RoomId, query: string): Promise<Music[]> {
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

async function voteForMusic(auth_token: string, room_id: RoomId, like: boolean, music_id: MusicId) {
	const response = await fetch(`${env.API_URL}/api/room/${room_id}/vote`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${auth_token}`
		},
		body: JSON.stringify({
			music_id,
			like
		})
	});

	if (!response.ok) {
		throw new Error(`Error voting: ${await response.text()}`);
	}
	voted_for.update((set) => {
		if (like) {
			set.add(music_id);
		} else {
			set.delete(music_id);
		}
		return set;
	});
}

async function getRooms(auth_token: string): Promise<Room[]> {
	const res = await fetch(`${env.API_URL}/api/room/all`, {
		headers: {
			Authorization: `Bearer ${auth_token}`
		}
	});

	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}

	const rooms = await res.json();

	return rooms.map(convertApiRoom);
}

async function deleteRoom(auth_token: string, room_id: RoomId) {
	const res = await fetch(`${env.API_URL}/api/room/${room_id}`, {
		method: 'DELETE',
		headers: {
			Authorization: `Bearer ${auth_token}`
		}
	});

	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}
}

async function createRoom(auth_token: string, room_id: RoomId, expiration: Date): Promise<Room> {
	const body = JSON.stringify({
		id: room_id,
		expiration: expiration.toISOString()
	});

	const res = await fetch(`${env.API_URL}/api/room`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${auth_token}`,
			'Content-Type': 'application/json'
		},
		body
	});

	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}

	return await res.json().then(convertApiRoom);
}

export { getMusics, getSearch, voteForMusic, getRooms, deleteRoom, createRoom, voted_for as votes };
