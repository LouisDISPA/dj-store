import { goto as svelteGoto } from '$app/navigation';
import type { Room } from './types';

export const env = {
	BASE_HREF: import.meta.env.VITE_BASE_HREF || '',
	API_URL: import.meta.env.VITE_API_URL || ''
};

export async function goto(path: string) {
	await svelteGoto(env.BASE_HREF + path);
}

export function randomRoomID(): string {
	let result = '';
	const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
	for (let i = 0; i < 6; i++) {
		result += characters.charAt(Math.floor(Math.random() * characters.length));
	}
	return result;
}

export function nowPlus(options: { days?: number; hours?: number }): Date {
	const now = new Date();
	let hours = now.getHours();

	if (options.hours !== undefined) {
		hours += options.hours;
	}
	if (options.days !== undefined) {
		hours += options.days * 24;
	}
	now.setHours(hours);
	return now;
}

export function convertApiRoom(room: Room): Room {
	return {
		...room,
		expiration: new Date(room.expiration),
		creation: new Date(room.creation)
	};
}

export const timeFormat = new Intl.DateTimeFormat(undefined, {
	year: 'numeric',
	month: 'short',
	day: 'numeric',
	hour: 'numeric',
	minute: 'numeric'
});
