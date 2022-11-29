import { env } from '$lib/utils';
import { error, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

type ApiRoom = {
	id: string;
	creation: string;
	expiration: string;
	user_count: number;
	active: boolean;
};

type Room = {
	id: string;
	creation: Date;
	expiration: Date;
	user_count: number;
	active: boolean;
};

export const load: PageLoad = async () => {
	const authToken = localStorage.getItem('authToken');
	if (!authToken) {
		throw redirect(301, '/login');
	}
	if (!checkToken(authToken)) {
		localStorage.removeItem('authToken');
		throw redirect(301, '/login');
	}

	const res = await fetch(`${env.API_URL}/api/room/all`, {
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

	const apiRooms: ApiRoom[] = await res.json();

	const rooms: Room[] = apiRooms.map(convertApiRoom);

	return { authToken, rooms };
};

function checkToken(authToken: string): boolean {
	const tokenData = JSON.parse(atob(authToken.split('.')[1]));
	if (tokenData.role !== 'Admin') {
		return false;
	}
	if (tokenData.exp < Date.now() / 1000) {
		return false;
	}
	return true;
}

export function convertApiRoom(apiRoom: ApiRoom): Room {
	const room = apiRoom as unknown as Room;
	room.creation = new Date(apiRoom.creation);
	room.expiration = new Date(apiRoom.expiration);
	return room;
}
