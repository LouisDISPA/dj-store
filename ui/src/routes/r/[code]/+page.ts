import type { PageLoad } from './$types';

export type Music = {
	id: string;
	title: string;
	artist: string;
};

export type Vote = {
	music: Music;
	count: number;
};

export const load: PageLoad = async ({ params }) => {
	await new Promise((resolve) => setTimeout(resolve, 500));

	const votes: Array<Vote> = [
		{
			music: {
				id: '1',
				title: 'Never Gonna Give You Up',
				artist: 'Rick Astley'
			},
			count: 1
		},
		{
			music: {
				id: '2',
				title: 'Sandstorm',
				artist: 'Darude'
			},
			count: 2
		},
		{
			music: {
				id: '3',
				title: 'Africa',
				artist: 'Toto'
			},
			count: 3
		},
		{
			music: {
				id: '4',
				title: 'Shake It Off',
				artist: 'Taylor Swift'
			},
			count: 4
		}
	];

	return { code: params.code, votes };
};
