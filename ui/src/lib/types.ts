export type Music = {
	id: string;
	title: string;
	artist: string;
	votes?: number;
};

export type Vote = {
	music_id: string;
	title: string;
	artist: string;
	vote_date: Date;
	like: boolean;
};

export type Room = {
	id: string;
	creation: Date;
	expiration: Date;
	user_count: number;
	active: boolean;
};
