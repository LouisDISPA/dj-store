import { error } from '@sveltejs/kit';
import { writable, type Writable } from 'svelte/store';
import { getVotes } from './client';
import type { RoomId } from './types';
import { env } from './utils';

const auth: Writable<Auth | undefined> = writable();

export { auth, connect, joinRoom, disconnect, tryRecallUser };
export type { Role, Auth };

type Role = 'Admin' | 'User';

type Auth = {
	access_token: string;
	role: Role;
	room_id?: RoomId;
};

type Token = {
	role: Role;
	room_id?: RoomId;
	iat: number;
	exp: number;
	uuid: string;
};

const TOKEN_STORAGE_KEY = 'access_token';

async function connect(username: string, password: string) {
	disconnect();
	const res = await fetch(`${env.API_URL}/api/admin/login`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ username, password })
	});

	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}

	const access_token = (await res.json())['access_token'] as string;

	storeUserToken(access_token);
	auth.set({ access_token, role: 'Admin' });
}

async function joinRoom(room_id: RoomId) {
	disconnect();
	const res = await fetch(`${env.API_URL}/api/room/${room_id}/join`);

	if (!res.ok) {
		const message = res.statusText;
		const detail = await res.text();
		throw error(res.status, { message, detail });
	}

	const access_token = (await res.json())['access_token'] as string;

	const token_data = decodeToken(access_token);
	const role = token_data.role as Role;

	storeUserToken(access_token);
	auth.set({ access_token, role, room_id });
}

async function tryRecallUser() {
	console.log('Try recall user');
	const access_token = localStorage.getItem(TOKEN_STORAGE_KEY);
	if (!access_token) {
		disconnect();
		console.log('No token to recall');
		return false;
	}
	const token_data = decodeToken(access_token);

	const expiration = token_data.exp * 1000;
	if (expiration < Date.now()) {
		disconnect();
		console.log('Token recalled expired');
		return false;
	}

	const role = token_data.role as Role;

	if (role === 'Admin') {
		console.log('Admin recalled');
		auth.set({ access_token, role });
		return true;
	}

	if (role === 'User') {
		const room_id = token_data.room_id as string;

		try {
			await getVotes(access_token, room_id);
			console.log('User recalled');
			auth.set({ access_token, role, room_id });
			return true;
		} catch (e) {
			disconnect();
			return false;
		}
	}

	disconnect();
	return false;
}

function storeUserToken(access_token: string) {
	localStorage.setItem(TOKEN_STORAGE_KEY, access_token);
}

function disconnect() {
	localStorage.removeItem(TOKEN_STORAGE_KEY);
	auth.set(undefined);
}

function decodeToken(access_token: string): Token {
	const json_string = window.atob(access_token.split('.')[1]);
	return JSON.parse(json_string);
}
